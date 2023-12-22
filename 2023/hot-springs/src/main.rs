use springs::Springs;
use std::fmt::Display;

mod counter_iter;
mod spring_row;
mod springs;

fn main() {
    aoc::aoc_main(aoc::input!(), part1, part2);
}

fn part1(input: &str) -> impl Display {
    input
        .parse::<Springs>()
        .map(|springs| springs.count_sum())
        .map(|value| value.to_string())
        .unwrap_or_else(|err| err.to_string())
}

fn part2(_input: &str) -> impl Display {
    0
}
