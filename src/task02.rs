const LIMIT: [u64; 3] = [12, 13, 14];

pub fn task02() {
    let start = std::time::Instant::now();
    let task = include_str!("../tasks/task02.txt");

    let result = task
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let (_, game) = l.split_once(':').unwrap();
            let mut acc = [0; 3];
            for round in game.trim().split(';') {
                for colour_count in round.split(',') {
                    let (num, colour) = colour_count.trim().split_once(' ').unwrap();
                    let num = num.parse::<u64>().unwrap();
                    let colour = match colour {
                        "red" => 0,
                        "green" => 1,
                        "blue" => 2,
                        _ => unreachable!(),
                    };
                    acc[colour] = acc[colour].max(num);
                }
            }
            let is_within_limit =
                usize::from(acc[0] <= LIMIT[0] && acc[1] <= LIMIT[1] && acc[2] <= LIMIT[2]);
            let index_sum = (i + 1) * is_within_limit;
            let power = acc.iter().product::<u64>();

            (index_sum, power)
        })
        .fold((0, 0), |(sum1, sum2), (v1, v2)| (sum1 + v1, sum2 + v2));

    println!("Part 1: {}", result.0);
    println!("Part 2: {}", result.1);
    let elapsed = start.elapsed();
    println!("Elapsed: {:.3}ms", elapsed.as_secs_f64() * 1000.0);
}
