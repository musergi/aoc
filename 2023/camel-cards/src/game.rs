use crate::{hand::Hand, play::Play};
use std::str::FromStr;

pub struct Game {
    plays: Vec<Play<Hand>>,
}

impl Game {
    pub fn winings<T>(self, wrap: fn(Hand) -> T) -> u64
    where
        T: Ord,
    {
        let mut plays: Vec<_> = self.plays.into_iter().map(|play| play.wrap(wrap)).collect();
        plays.sort_by(|a, b| a.hand.cmp(&b.hand));
        plays
            .into_iter()
            .enumerate()
            .map(|(idx, play)| play.bid * (idx + 1) as u64)
            .sum()
    }
}

impl FromStr for Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let plays = s
            .lines()
            .map(|line| line.parse())
            .collect::<Result<_, _>>()?;
        Ok(Game { plays })
    }
}

#[cfg(test)]
mod tests {
    use super::Game;
    use crate::hand::{NewHand, OldHand};

    #[test]
    fn example_part1() {
        let string = include_str!("../assets/example.txt");
        let game: Game = string.parse().unwrap();
        assert_eq!(game.winings(OldHand::new), 6440);
    }

    #[test]
    fn example_part2() {
        let string = include_str!("../assets/example.txt");
        let game: Game = string.parse().unwrap();
        assert_eq!(game.winings(NewHand::new), 5905);
    }
}
