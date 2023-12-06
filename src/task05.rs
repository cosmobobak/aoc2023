#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_possible_wrap)]

pub fn task05() {
    let start = std::time::Instant::now();
    let task = include_str!("../tasks/task04.txt");

    let result = todo!()

    println!("Part 1: {}", result.0);
    println!("Part 2: {}", result.1);
    let elapsed = start.elapsed();
    println!("Elapsed: {:.3}ms", elapsed.as_secs_f64() * 1000.0);
}
