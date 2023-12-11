use map::Map;
use std::fmt::Display;

mod instruction;
mod line;
mod map;

fn main() {
    aoc::aoc_main(aoc::input!(), part1, part2)
}

fn part1(input: &str) -> impl Display {
    common(input, |map| map.steps())
}

fn part2(input: &str) -> impl Display {
    common(input, |map| map.steps_all())
}

fn common(input: &str, func: fn(&Map) -> usize) -> impl Display {
    input
        .parse::<Map>()
        .map(|map| func(&map))
        .map(|value| value.to_string())
        .unwrap_or_else(|err| err.to_string())
}
