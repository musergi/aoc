use contraption::Contraption;
use std::fmt::Display;

mod contraption;

fn main() {
    aoc::aoc_main(aoc::input!(), part1, part2)
}

fn part1(input: &str) -> impl Display {
    common(input, Contraption::energized)
}

fn part2(input: &str) -> impl Display {
    common(input, Contraption::max_energized)
}

fn common(input: &str, func: fn(&Contraption) -> usize) -> impl Display {
    input
        .parse::<Contraption>()
        .map(|contraption| func(&contraption))
        .map(|value| value.to_string())
        .unwrap_or_else(|err| err.to_string())
}
