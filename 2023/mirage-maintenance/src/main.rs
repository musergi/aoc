use reading::Reading;
use std::fmt::Display;

mod history;
mod reading;

fn main() {
    aoc::aoc_main(aoc::input!(), part1, part2)
}

fn part1(input: &str) -> impl Display {
    common(input, Reading::predict_next_sum)
}

fn part2(input: &str) -> impl Display {
    common(input, Reading::predict_previous_sum)
}

fn common(input: &str, func: fn(&Reading) -> i32) -> impl Display {
    input
        .parse::<Reading>()
        .map(|reading| func(&reading))
        .map(|value| value.to_string())
        .unwrap_or_else(|err| err.to_string())
}
