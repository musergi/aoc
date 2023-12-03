use std::str::FromStr;

use crate::cube_set::CubeSet;

pub struct Game {
    pub id: u32,
    cube_set: CubeSet,
}

impl Game {
    pub fn is_possible(&self, total: &CubeSet) -> bool {
        self.cube_set.red <= total.red
            && self.cube_set.green <= total.green
            && self.cube_set.blue <= total.blue
    }

    pub fn power(&self) -> u32 {
        self.cube_set.red * self.cube_set.green * self.cube_set.blue
    }
}

impl FromStr for Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game_s, grabs_s) = s.split_once(": ").ok_or("missing description separator")?;
        let index = game_s
            .strip_prefix("Game ")
            .ok_or("missing game prefix")?
            .parse()
            .map_err(|_| "invalid game index")?;
        let grab = grabs_s
            .split("; ")
            .map(|grab| grab.parse())
            .try_fold(CubeSet::default(), |acc, new| new.map(|new| acc.max(new)))?;
        Ok(Game {
            id: index,
            cube_set: grab,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Game;

    #[test]
    fn parse_example1() {
        let game: Game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
            .parse()
            .unwrap();
        assert_eq!(game.id, 1);
        assert_eq!(game.cube_set.red, 4);
        assert_eq!(game.cube_set.green, 2);
        assert_eq!(game.cube_set.blue, 6);
    }

    #[test]
    fn parse_example2() {
        let game: Game = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"
            .parse()
            .unwrap();
        assert_eq!(game.id, 2);
        assert_eq!(game.cube_set.red, 1);
        assert_eq!(game.cube_set.green, 3);
        assert_eq!(game.cube_set.blue, 4);
    }

    #[test]
    fn parse_example3() {
        let game: Game = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
            .parse()
            .unwrap();
        assert_eq!(game.id, 3);
        assert_eq!(game.cube_set.red, 20);
        assert_eq!(game.cube_set.green, 13);
        assert_eq!(game.cube_set.blue, 6);
    }

    #[test]
    fn parse_example4() {
        let game: Game = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
            .parse()
            .unwrap();
        assert_eq!(game.id, 4);
        assert_eq!(game.cube_set.red, 14);
        assert_eq!(game.cube_set.green, 3);
        assert_eq!(game.cube_set.blue, 15);
    }

    #[test]
    fn parse_example5() {
        let game: Game = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .parse()
            .unwrap();
        assert_eq!(game.id, 5);
        assert_eq!(game.cube_set.red, 6);
        assert_eq!(game.cube_set.green, 3);
        assert_eq!(game.cube_set.blue, 2);
    }
}
