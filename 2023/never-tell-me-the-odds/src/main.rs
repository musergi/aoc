use aoc::{aoc_main, input};
use puzzle::Puzzle;
use std::fmt::Display;

mod area;
mod hailstone;
mod matrix;
mod puzzle;
mod vec;

fn main() {
    aoc_main(input!(), part1, part2);
}

fn part1(input: &str) -> impl Display {
    common(input, Puzzle::part1)
}

fn part2(input: &str) -> impl Display {
    common(input, Puzzle::part2)
}

fn common(input: &str, func: fn(Puzzle) -> usize) -> impl Display {
    input
        .parse::<Puzzle>()
        .map(func)
        .map(|value| value.to_string())
        .unwrap_or_else(|err| err.to_string())
}
