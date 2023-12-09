use map::Map;
use std::fmt::Display;

mod instruction;
mod line;
mod map;

fn main() {
    aoc::aoc_main(aoc::input!(), part1, part2)
}

fn part1(input: &str) -> impl Display {
    input
        .parse::<Map>()
        .map(|map| map.steps())
        .map(|value| value.to_string())
        .unwrap_or_else(|err| err.to_string())
}

fn part2(_input: &str) -> impl Display {
    0
}
