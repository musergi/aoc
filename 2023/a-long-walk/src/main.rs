use aoc::{aoc_main, input};
use puzzle::Puzzle;
use std::fmt::Display;

mod generator;
mod graph;
mod ice;
mod path;
mod position;
mod puzzle;

fn main() {
    aoc_main(input!(), part1, part2);
}

fn part1(input: &str) -> impl Display {
    common(input, Puzzle::part1)
}

fn part2(input: &str) -> impl Display {
    common(input, Puzzle::part2)
}

fn common(input: &str, solver: fn(Puzzle) -> usize) -> impl Display {
    input
        .parse()
        .map(solver)
        .map(|value| value.to_string())
        .unwrap_or_else(|err| err.to_string())
}
