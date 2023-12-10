use join_scoreboard::JoinScoreboard;
use scoreboard::Scoreboard;
use separate_scoreboard::SeparateScoreboard;
use std::{fmt::Display, str::FromStr};

mod competition;
mod join_scoreboard;
mod scoreboard;
mod separate_scoreboard;

fn main() {
    aoc::aoc_main(aoc::input!(), part1, part2)
}

fn part1(input: &str) -> impl Display {
    common::<SeparateScoreboard>(input)
}

fn part2(input: &str) -> impl Display {
    common::<JoinScoreboard>(input)
}

fn common<T>(input: &str) -> impl Display
where
    T: Scoreboard + FromStr<Err = &'static str>,
{
    input
        .parse::<T>()
        .map(|scoreboard| scoreboard.wining_move_count())
        .map(|value| value.to_string())
        .unwrap_or_else(|err| err.to_string())
}
