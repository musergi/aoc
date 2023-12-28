use patterns::Patterns;
use std::fmt::Display;

mod pattern;
mod patterns;

fn main() {
    aoc::aoc_main(aoc::input!(), part1, part2)
}

fn part1(input: &str) -> impl Display {
    common(input, |patterns| patterns.summarize())
}

fn part2(input: &str) -> impl Display {
    common(input, |patterns| patterns.one_off_summarize())
}

fn common(input: &str, func: fn(Patterns) -> Result<usize, &'static str>) -> impl Display {
    input
        .parse::<Patterns>()
        .and_then(func)
        .map(|value| value.to_string())
        .unwrap_or_else(|err| err.to_string())
}
