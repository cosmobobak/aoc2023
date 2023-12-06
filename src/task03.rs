use std::collections::{
    hash_map::{Entry, OccupiedEntry},
    HashMap,
};

pub fn task03() {
    let start = std::time::Instant::now();
    let task = include_str!("../tasks/task03.txt");

    let task = task.as_bytes();
    let row_len = task.iter().position(|&b| b == b'\n').unwrap() + 1;
    let mut num_pairings = HashMap::<usize, i64>::new();

    let result = task
        .split(|&b| b == b'\n')
        .enumerate()
        .flat_map(|(ridx, row)| {
            let mut string_slice = 0..=0;
            let mut currently_parsing = false;
            row.iter().enumerate().filter_map(move |(idx, byte)| {
                match (currently_parsing, byte.is_ascii_digit()) {
                    (false, false) => None,
                    (false, true) => {
                        currently_parsing = true;
                        string_slice = idx..=idx;
                        None
                    }
                    (true, false) => {
                        currently_parsing = false;
                        Some((ridx, &row[string_slice.clone()], string_slice.clone()))
                    }
                    (true, true) => {
                        string_slice = *string_slice.start()..=idx;
                        None
                    }
                }
            })
        })
        .filter_map(|(row, num, int_range)| {
            const SYMS: [u8; 10] = *b"%+*#@&/-$=";

            let row_offset = row * row_len;
            let rabove_fst = (row_offset + *int_range.start()).wrapping_sub(1 + row_len);
            let rabove_end = (row_offset + *int_range.end() + 1).wrapping_sub(row_len);
            for i in rabove_fst..=rabove_end {
                if task.get(i).is_some_and(|&b| SYMS.contains(&b)) {
                    return Some((i, num));
                }
            }
            let rbelow_fst = rabove_fst.wrapping_add(row_len * 2);
            let rbelow_end = rabove_end.wrapping_add(row_len * 2);
            for i in rbelow_fst..=rbelow_end {
                if task.get(i).is_some_and(|&b| SYMS.contains(&b)) {
                    return Some((i, num));
                }
            }
            let b_start = rabove_fst.wrapping_add(row_len);
            let a_start = rabove_end.wrapping_add(row_len);
            if task.get(b_start).is_some_and(|&b| SYMS.contains(&b)) {
                return Some((b_start, num));
            }
            if task.get(a_start).is_some_and(|&b| SYMS.contains(&b)) {
                return Some((a_start, num));
            }

            None
        })
        .map(|(k, slice)| {
            let mut num = 0;
            for &byte in slice {
                num *= 10;
                num += i64::from(byte - b'0');
            }
            if task[k] == b'*' {
                match num_pairings.entry(k) {
                    Entry::Occupied(mut entry) => {
                        *entry.get_mut() *= -num;
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(-num);
                    }
                }
            }
            num
        })
        .sum::<i64>();

    println!("Part 1: {result}");
    println!(
        "Part 2: {}",
        num_pairings.values().filter(|&&x| x > 0).sum::<i64>()
    );
    let elapsed = start.elapsed();
    println!("Elapsed: {:.3}ms", elapsed.as_secs_f64() * 1000.0);
}
