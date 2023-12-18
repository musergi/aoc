use pipes::Pipes;
use std::fmt::Display;

mod direction;
mod pipe_type;
mod pipes;
mod vec2;

fn main() {
    aoc::aoc_main(aoc::input!(), part1, part2)
}

fn part1(input: &str) -> impl Display {
    common(input, Pipes::farthest_distance)
}

fn part2(input: &str) -> impl Display {
    common(input, Pipes::inner_tiles)
}

fn common(input: &str, func: fn(Pipes) -> usize) -> impl Display {
    input
        .parse::<Pipes>()
        .map(func)
        .map(|value| value.to_string())
        .unwrap_or_else(|err| err.to_string())
}
