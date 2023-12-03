use std::fmt::Display;

use cube_set::CubeSet;
use game::Game;

mod cube_set;
mod game;

fn main() {
    aoc::aoc_main(aoc::input!(), part1, part2)
}

const MAX: CubeSet = CubeSet {
    red: 12,
    green: 13,
    blue: 14,
};

fn part1(input: &str) -> impl Display {
    sum_all(
        input,
        |game| if game.is_possible(&MAX) { game.id } else { 0 },
    )
}

fn part2(input: &str) -> impl Display {
    sum_all(input, |game| game.power())
}

fn sum_all(input: &str, mapper: fn(Game) -> u32) -> impl Display {
    input
        .lines()
        .map(|line| line.parse::<Game>().map(|game| mapper(game)))
        .sum::<Result<u32, _>>()
        .map(|v| v.to_string())
        .unwrap_or_else(|err| err.to_string())
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn part1_example() {
        assert_eq!(
            format!("{}", part1(include_str!("../assets/example.txt"))),
            "8"
        )
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            format!("{}", part2(include_str!("../assets/example.txt"))),
            "2286"
        )
    }
}
