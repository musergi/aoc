use std::fmt::Display;

use puzzle::Puzzle;

mod disance_iter;
mod puzzle;
mod vec;

fn main() {
    aoc::aoc_main(aoc::input!(), part1, part2);
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
        .map(|v| v.to_string())
        .unwrap_or_else(|err| err.to_string())
}
