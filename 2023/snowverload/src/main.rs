use aoc::{aoc_main, input};
use puzzle::Puzzle;
use std::fmt::Display;

mod puzzle;

fn main() {
    aoc_main(input!(), part1, part2);
}

fn part1(input: &str) -> impl Display {
    input
        .parse::<Puzzle>()
        .map(|puzzle| puzzle.solve().to_string())
        .unwrap_or_else(|err| err.to_string())
}

fn part2(_input: &str) -> impl Display {
    "Done"
}
