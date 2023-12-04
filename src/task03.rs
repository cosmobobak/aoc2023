pub fn task03() {
    let start = std::time::Instant::now();
    let task = include_str!("../tasks/task03.txt");

    let task = task.as_bytes();
    let row_len = task.iter().position(|&b| b == b'\n').unwrap() as i64 + 1;

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

            let row_offset = row as i64 * row_len;
            let row_above_start = row_offset + *int_range.start() as i64 - row_len - 1;
            let row_above_end = row_offset + *int_range.end() as i64 - row_len + 1;
            for i in row_above_start..=row_above_end {
                if task.get(i as usize).is_some_and(|&b| SYMS.contains(&b)) {
                    return Some((i, num))
                }
            }
            let row_below_start = row_above_start + row_len * 2;
            let row_below_end = row_above_end + row_len * 2;
            for i in row_below_start..=row_below_end {
                if task.get(i as usize).is_some_and(|&b| SYMS.contains(&b)) {
                    return Some((i, num))
                }
            }
            let one_before_start = row_above_start + row_len;
            let one_after_end = row_above_end + row_len;
            if task.get(one_before_start as usize).is_some_and(|&b| SYMS.contains(&b)) {
                return Some((one_before_start, num))
            }
            if task.get(one_after_end as usize).is_some_and(|&b| SYMS.contains(&b)) {
                return Some((one_after_end, num))
            }

            None
        })
        .map(|(_, slice)| {
            let mut num = 0;
            for &byte in slice {
                num *= 10;
                num += i64::from(byte - b'0');
            }
            num
        })
        .sum::<i64>();

    println!("Part 1: {}", result);
    // println!("Part 2: {}", result.1);
    let elapsed = start.elapsed();
    println!("Elapsed: {:.3}ms", elapsed.as_secs_f64() * 1000.0);
}
