#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]

#[derive(Debug, Clone, Copy)]
struct Mapping {
    dst_start: u64,
    src_start: u64,
    len: u64,
}

#[derive(Debug, Clone, Copy)]
struct ValueRange {
    start: u64,
    len: u64,
}

impl Mapping {
    const fn map(&self, value: u64) -> Option<u64> {
        if value < self.src_start || value >= self.src_start + self.len {
            return None;
        }
        Some(value + self.dst_start - self.src_start)
    }

    const fn map_vr(&self, vr: ValueRange) -> Option<(ValueRange, Option<ValueRange>, Option<ValueRange>)> {
        if vr.start + vr.len <= self.src_start || vr.start >= self.src_start + self.len {
            return None;
        }

        // if the value range is completely inside of the mapping, return the mapped value range
        if vr.start >= self.src_start && vr.start + vr.len <= self.src_start + self.len {
            let moved = ValueRange {
                start: vr.start + self.dst_start - self.src_start,
                len: vr.len,
            };
            return Some((moved, None, None));
        }

        // if the mapping is completely inside the value range, return the mapped value range and both halves of the leftovers
        if self.src_start >= vr.start && self.src_start + self.len <= vr.start + vr.len {
            let moved = ValueRange {
                start: self.dst_start,
                len: self.len,
            };
            let unmoved_left = ValueRange {
                start: vr.start,
                len: self.src_start - vr.start,
            };
            let unmoved_right = ValueRange {
                start: self.src_start + self.len,
                len: vr.len - moved.len - unmoved_left.len,
            };
            return Some((moved, Some(unmoved_left), Some(unmoved_right)))
        }

        // left overlap:
        if vr.start < self.src_start {
            // truncate this range to that which remains unmoved
            let unmoved = ValueRange {
                start: vr.start,
                len: self.src_start - vr.start,
            };
            // create the moved section
            let moved = ValueRange {
                start: self.dst_start,
                len: vr.len - unmoved.len,
            };
            return Some((moved, Some(unmoved), None));
        }

        // right overlap:
        let moved = ValueRange {
            start: vr.start + self.dst_start - self.src_start,
            len: (self.src_start + self.len) - vr.start,
        };
        let unmoved = ValueRange {
            start: self.src_start + self.len,
            len: vr.len - moved.len,
        };

        Some((moved, Some(unmoved), None))
    }
}

fn map_any(vr: ValueRange, mappings: &[Mapping]) -> Vec<ValueRange> {
    let mut source = vec![vr];
    let mut results = Vec::new();
    for &mapping in mappings {
        'scan: loop {
            for i in 0..source.len() {
                if let Some((m, m2, m3)) = mapping.map_vr(source[i]) {
                    results.push(m);
                    if let Some(m2) = m2 {
                        source[i] = m2;
                    }
                    if let Some(m3) = m3 {
                        source.push(m3);
                    }
                    continue 'scan;
                }
            }
            break 'scan;
        }
    }
    if results.is_empty() {
        results.push(vr);
    }
    results
}

fn compute_lowest_mapping(seeds: &[ValueRange], layers: &[Vec<Mapping>]) -> u64 {
    let n_seeds = seeds.len();
    let res = seeds
        .iter()
        .enumerate()
        .flat_map(|(i, s)| {
            let mut ranges = vec![*s];
            let mut new_ranges = Vec::new();
            for layer in layers {
                for &vr in &ranges {
                    let resulting_ranges = map_any(vr, layer);
                    println!(
                        "ValueRange [{}-{}] with layer\n{:?}\nmaps to [{}]",
                        vr.start,
                        vr.start + vr.len,
                        layer,
                        resulting_ranges.iter().map(|vr| format!("[{}-{}]", vr.start, vr.start + vr.len)).collect::<Vec<_>>().join(",")
                    );
                    new_ranges.extend_from_slice(&resulting_ranges);
                }
                ranges.clear();
                std::mem::swap(&mut ranges, &mut new_ranges);
            }
            // report progress
            print!("\r{}/{}", i + 1, n_seeds);
            // flush stdout
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            ranges
        })
        .map(|vr| vr.start)
        .min()
        .unwrap();
    println!();
    res
}

pub fn task05() {
    let start = std::time::Instant::now();
    let task = include_str!("../tasks/task05.txt");

    let (seeds, layers) = task.split_once("\n\n").unwrap();

    let seed_ints = seeds
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let layers = layers
        .split("\n\n")
        .map(|layer| {
            layer
                .lines()
                .skip(1)
                .map(|l| {
                    let mut parts = l.split_whitespace();
                    let dst_start = parts.next().unwrap().parse::<u64>().unwrap();
                    let src_start = parts.next().unwrap().parse::<u64>().unwrap();
                    let len = parts.next().unwrap().parse::<u64>().unwrap();
                    Mapping {
                        dst_start,
                        src_start,
                        len,
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let single_seeds = seed_ints
        .iter()
        .map(|&s| ValueRange { start: s, len: 1 })
        .collect::<Vec<_>>();

    let lowest_mapping = compute_lowest_mapping(&single_seeds, &layers);

    println!("Part 1: {lowest_mapping}");

    let seed_ranges = seed_ints
        .chunks(2)
        .map(|c| ValueRange {
            start: c[0],
            len: c[1],
        })
        .collect::<Vec<_>>();

    let lowest_mapping = compute_lowest_mapping(&seed_ranges, &layers);

    println!("Part 2: {lowest_mapping}");
    let elapsed = start.elapsed();
    println!("Elapsed: {:.3}ms", elapsed.as_secs_f64() * 1000.0);
}
