use aoc::{aoc_main, input};
use puzzle::Puzzle;
use std::fmt::Display;

mod ice;
mod position;
mod puzzle;

fn main() {
    aoc_main(input!(), part1, part2);
}

fn part1(input: &str) -> impl Display {
    input
        .parse()
        .map(Puzzle::part1)
        .map(|value| value.to_string())
        .unwrap_or_else(|err| err.to_string())
}

fn part2(_input: &str) -> impl Display {
    0
}
