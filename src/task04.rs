#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_possible_wrap)]

pub fn task04() {
    let start = std::time::Instant::now();
    let task = include_str!("../tasks/task04.txt");
    let max_win_count = task.lines().next().unwrap().split_once(": ").unwrap().1.split_once(" | ").unwrap().1.len() / 3;
    let mut won_future_copies = vec![1; max_win_count];

    let result = task
        .lines()
        .map(|line| line.split_once(": ").unwrap().1.split_once(" | ").unwrap())
        .map(|(winning_nums, our_nums)| {
            let winning_nums = winning_nums.as_bytes();
            let wins = our_nums
                .as_bytes()
                .chunks(3)
                .filter(|num| winning_nums.chunks(3).any(|win| win[0..2] == num[0..2]))
                .count();
            let value = 2f32.powi(wins as i32 - 1) as u64;

            let multiplier = won_future_copies[0];
            won_future_copies[0] = 1;
            won_future_copies.rotate_left(1);
            for v in &mut won_future_copies[..wins] {
                *v += multiplier;
            }

            (value, multiplier)
        })
        // sum both up
        .fold((0, 0), |(acc1, acc2), (a, b)| (acc1 + a, acc2 + b));

    println!("Part 1: {}", result.0);
    println!("Part 2: {}", result.1);
    let elapsed = start.elapsed();
    println!("Elapsed: {:.3}ms", elapsed.as_secs_f64() * 1000.0);
}
