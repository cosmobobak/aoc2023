use rayon::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Mapping {
    dst_start: u64,
    src_start: u64,
    len: u64,
}

impl Mapping {
    const fn map(&self, value: u64) -> Option<u64> {
        if value < self.src_start || value >= self.src_start + self.len {
            return None;
        }
        Some(value + self.dst_start - self.src_start)
    }
}

fn compute_lowest_mapping(seeds: impl ParallelIterator<Item = u64>, layers: &[&[Mapping]]) -> u64 {
    seeds
        .map(|s| {
            let mut s = s;
            for &layer in layers {
                for &mapping in layer {
                    if let Some(mapped) = mapping.map(s) {
                        s = mapped;
                        break;
                    }
                }
            }
            s
        })
        .min()
        .unwrap()
}

pub fn task05() {
    let start = std::time::Instant::now();
    let task = include_str!("../tasks/task05.txt");

    let (seeds, layers) = task.split_once("\n\n").unwrap();

    let seeds = seeds
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

    let lengths = layers.iter().map(Vec::len).collect::<Vec<_>>();
    let layers_buffer = layers.into_iter().flatten().collect::<Vec<_>>();
    let layers = lengths
        .iter()
        .scan(0, |offset, &len| {
            let res = &layers_buffer[*offset..*offset + len];
            *offset += len;
            Some(res)
        })
        .collect::<Vec<_>>();

    let lowest_mapping = compute_lowest_mapping(seeds.par_iter().copied(), &layers);

    println!("Part 1: {lowest_mapping}");

    let range_mapping = seeds.par_chunks(2).flat_map(|chunk| {
        let start = chunk[0];
        let count = chunk[1];
        start..start + count
    });
    let lowest_mapping = compute_lowest_mapping(range_mapping, &layers);

    println!("Part 2: {lowest_mapping}");
    let elapsed = start.elapsed();
    println!("Elapsed: {:.3}ms", elapsed.as_secs_f64() * 1000.0);
}
