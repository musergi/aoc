use crate::play::Play;
use std::str::FromStr;

pub struct Game {
    plays: Vec<Play>,
}

impl Game {
    pub fn winings(mut self) -> u64 {
        self.plays.sort_by(|a, b| a.hand.cmp(&b.hand));
        self.plays
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

    #[test]
    fn example() {
        let string = include_str!("../assets/example.txt");
        let game: Game = string.parse().unwrap();
        assert_eq!(game.winings(), 6440);
    }
}
