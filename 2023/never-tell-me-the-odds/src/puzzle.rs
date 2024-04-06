use crate::{area::Area, hailstone::Hailstone};
use std::str::FromStr;

pub struct Puzzle {
    hailstones: Vec<Hailstone>,
}

impl Puzzle {
    pub fn part1(self) -> usize {
        let area = Area::new(200000000000000.0, 400000000000000.0);
        self.part1_with_area(&area)
    }

    fn part1_with_area(self, area: &Area) -> usize {
        self.hailstones
            .iter()
            .enumerate()
            .map(|(idx, first)| {
                self.hailstones[(idx + 1)..]
                    .iter()
                    .filter(|second| first.xy_intersect_in(second, area))
                    .count()
            })
            .sum()
    }
}

impl FromStr for Puzzle {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .map(|line| line.parse::<Hailstone>())
            .collect::<Result<Vec<_>, _>>()
            .map(|hailstones| Puzzle { hailstones })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let s = include_str!("../assets/example.txt");
        let puzzle: Puzzle = s.parse().unwrap();
        let area = Area::new(7.0, 27.0);
        assert_eq!(puzzle.part1_with_area(&area), 2);
    }
}
