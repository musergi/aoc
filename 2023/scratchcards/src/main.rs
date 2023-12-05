use card::Card;
use std::fmt::Display;

mod card;

fn main() {
    aoc::aoc_main(aoc::input!(), part1, part2)
}

fn part1(input: &str) -> impl Display {
    input
        .lines()
        .map(|line| line.parse::<Card>().map(|card| card.points()))
        .sum::<Result<u32, _>>()
        .map(|v| v.to_string())
        .unwrap_or_else(|err| err.to_string())
}

fn part2(input: &str) -> impl Display {
    input
        .lines()
        .map(|line| line.parse::<Card>().map(|card| card.count()))
        .collect::<Result<Vec<_>, _>>()
        .map(compute_part2)
        .map(|v| v.to_string())
        .unwrap_or_else(|err| err.to_string())
}

fn compute_part2(card_matches: Vec<usize>) -> usize {
    let mut counts: Vec<_> = card_matches.iter().map(|_| 1usize).collect();
    for (idx, matches) in card_matches.iter().enumerate() {
        let start = idx + 1;
        let end = start + matches;
        let count = *counts.get(idx).unwrap();
        if *matches > 0 {
            for value in counts[start..end].iter_mut() {
                *value += count;
            }
        }
    }
    counts.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn example_part1() {
        let s = include_str!("../assets/example.txt");
        assert_eq!(format!("{}", part1(s)), "13");
    }

    #[test]
    fn example_part2() {
        let s = include_str!("../assets/example.txt");
        assert_eq!(format!("{}", part2(s)), "30")
    }
}
