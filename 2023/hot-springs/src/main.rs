use springs::Springs;
use std::fmt::Display;

mod counter_iter;
mod spring_row;
mod springs;

fn main() {
    aoc::aoc_main(aoc::input!(), part1, part2);
}

fn part1(input: &str) -> impl Display {
    common(input, std::convert::identity)
}

fn part2(input: &str) -> impl Display {
    common(input, Springs::expanded)
}

fn common(input: &str, prefunc: fn(Springs) -> Springs) -> impl Display {
    input
        .parse::<Springs>()
        .map(prefunc)
        .map(|springs| springs.count_sum())
        .map(|value| value.to_string())
        .unwrap_or_else(|err| err.to_string())
}
