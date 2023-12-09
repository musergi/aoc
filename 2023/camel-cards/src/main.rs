use game::Game;
use std::fmt::Display;

mod game;
mod hand;
mod play;

fn main() {
    aoc::aoc_main(aoc::input!(), part1, part2);
}

fn part1(input: &str) -> impl Display {
    input
        .parse::<Game>()
        .map(|game| game.winings())
        .map(|value| value.to_string())
        .unwrap_or_else(|err| err.to_string())
}

fn part2(_input: &str) -> impl Display {
    0
}
