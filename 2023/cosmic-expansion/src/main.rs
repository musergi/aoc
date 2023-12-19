use std::fmt::Display;
use universe::Universe;

mod universe;

fn main() {
    aoc::aoc_main(aoc::input!(), part1, part2)
}

fn part1(input: &str) -> impl Display {
    common(input, 2)
}

fn part2(input: &str) -> impl Display {
    common(input, 1_000_000)
}

fn common(input: &str, factor: i64) -> impl Display {
    input
        .parse::<Universe>()
        .map(|universe| universe.shortest_path_sum(factor))
        .map(|value| value.to_string())
        .unwrap_or_else(|err| err.to_string())
}
