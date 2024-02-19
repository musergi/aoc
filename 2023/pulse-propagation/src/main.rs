use std::fmt::Display;

use puzzle::Puzzle;

mod module;
mod module_line;
mod module_type;
mod puzzle;
mod signal;

fn main() {
    aoc::aoc_main(aoc::input!(), part1, part2)
}

fn part1(input: &str) -> impl Display {
    input
        .parse::<Puzzle>()
        .map(|puzzle| puzzle.part1())
        .map(|v| v.to_string())
        .unwrap_or_else(|err| err.to_string())
}

fn part2(input: &str) -> impl Display {
    0
}
