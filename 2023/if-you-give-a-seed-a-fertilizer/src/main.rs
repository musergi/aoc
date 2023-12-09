use almanac::Almanac;
use std::fmt::Display;

mod almanac;
mod mapping;
mod range;

fn main() {
    aoc::aoc_main(aoc::input!(), part1, part2)
}

fn part1(input: &str) -> impl Display {
    common(input, Almanac::get_min_location)
}

fn part2(input: &str) -> impl Display {
    common(input, Almanac::get_min_ranges_location)
}

fn common(input: &str, func: fn(&Almanac) -> Option<u64>) -> impl Display {
    input
        .parse::<Almanac>()
        .and_then(|almanac| func(&almanac).ok_or("no min location"))
        .map(|value| value.to_string())
        .unwrap_or_else(|err| err.to_string())
}
