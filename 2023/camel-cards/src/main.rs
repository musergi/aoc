use game::Game;
use hand::{Hand, NewHand, OldHand};
use std::fmt::Display;

mod game;
mod hand;
mod play;

fn main() {
    aoc::aoc_main(aoc::input!(), part1, part2);
}

fn part1(input: &str) -> impl Display {
    common(input, OldHand::new)
}

fn part2(input: &str) -> impl Display {
    common(input, NewHand::new)
}

fn common<T>(input: &str, func: fn(Hand) -> T) -> impl Display
where
    T: Ord,
{
    input
        .parse::<Game>()
        .map(|game| game.winings(func))
        .map(|value| value.to_string())
        .unwrap_or_else(|err| err.to_string())
}
