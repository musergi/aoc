use std::fmt::Display;

fn main() {
    aoc::aoc_main(aoc::input!(), part1, part2)
}

fn part1(input: &str) -> impl Display {
    0
}

fn part2(input: &str) -> impl Display {
    0
}

mod statement;
use statement::Statement;

mod expression;
use expression::Expression;

mod solver;