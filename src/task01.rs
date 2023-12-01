pub fn task01() {
    let start = std::time::Instant::now();
    let task = include_str!("../tasks/task01.txt");
    let result = task
        .lines()
        .map(str::as_bytes)
        .map(|l| (process_line_part1(l), process_line_part2(l)))
        .fold((0, 0), |(sum1, sum2), (v1, v2)| (sum1 + v1, sum2 + v2));

    println!("Part 1: {}", result.0);
    println!("Part 2: {}", result.1);
    let elapsed = start.elapsed();
    println!("Elapsed: {:.3}ms", elapsed.as_secs_f64() * 1000.0);
}

fn process_line_part1(line: &[u8]) -> u64 {
    let first_digit = *line.iter().find(|&&b| b.is_ascii_digit()).unwrap() - b'0';
    let last_digit = *line.iter().rfind(|&&b| b.is_ascii_digit()).unwrap() - b'0';
    let value = first_digit * 10 + last_digit;
    u64::from(value)
}

fn process_line_part2(line: &[u8]) -> u64 {
    const DIGIT_NAMES: [&[u8]; 18] = [
        b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine", b"1",
        b"2", b"3", b"4", b"5", b"6", b"7", b"8", b"9",
    ];
    let first_digit = (0..line.len())
        .map(|i| &line[i..])
        .find_map(|chunk| DIGIT_NAMES.iter().position(|&name| chunk.starts_with(name)))
        .unwrap()
        % 9
        + 1;
    let last_digit = (0..line.len())
        .rev()
        .map(|i| &line[i..])
        .find_map(|chunk| DIGIT_NAMES.iter().position(|&name| chunk.starts_with(name)))
        .unwrap()
        % 9
        + 1;
    let value = first_digit * 10 + last_digit;
    u64::try_from(value).unwrap()
}
