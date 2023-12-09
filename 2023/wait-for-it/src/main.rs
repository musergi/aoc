use scoreboard::Scoreboard;
use std::fmt::Display;

mod competition;
mod scoreboard;

fn main() {
    aoc::aoc_main(aoc::input!(), part1, part2)
}

fn part1(input: &str) -> impl Display {
    input
        .parse::<Scoreboard>()
        .map(|scoreboard| scoreboard.wining_move_count())
        .map(|value| value.to_string())
        .unwrap_or_else(|err| err.to_string())
}

fn part2(_input: &str) -> impl Display {
    0
}
