use platform::Platform;
use std::fmt::Display;

mod platform;

fn main() {
    aoc::aoc_main(aoc::input!(), part1, part2)
}

fn part1(input: &str) -> impl Display {
    common(input, Platform::north_load)
}

fn part2(input: &str) -> impl Display {
    common(input, Platform::cycled_north_load)
}

fn common(input: &str, func: fn(Platform) -> usize) -> impl Display {
    input
        .parse::<Platform>()
        .map(func)
        .map(|value| value.to_string())
        .unwrap_or_else(|err| err.to_string())
}
