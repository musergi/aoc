use std::fmt::Display;

pub fn aoc_main<F1, F2, R1, R2>(path: &str, part1: F1, part2: F2)
where
    F1: Fn(&str) -> R1,
    F2: Fn(&str) -> R2,
    R1: Display,
    R2: Display,
{
    let input = std::fs::read_to_string(path).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[macro_export]
macro_rules! input {
    () => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/assets/input.txt")
    };
}
