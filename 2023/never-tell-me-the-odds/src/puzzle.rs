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

    pub fn part2(self) -> usize {
        let h0 = self.hailstones.get(0).unwrap();
        let h1 = self.hailstones.get(1).unwrap();
        let h2 = self.hailstones.get(2).unwrap();
        let p1 = (h1.position - h0.position).widen();
        let v1 = (h1.velocity - h0.velocity).widen();
        let p2 = (h2.position - h0.position).widen();
        let v2 = (h2.velocity - h0.velocity).widen();
        let t1 = -p1.cross(p2).dot(v2) / v1.cross(p2).dot(v2);
        let t2 = -p1.cross(p2).dot(v1) / p1.cross(v2).dot(v1);
        let c1 = h1.position.widen() + (h1.velocity.widen() * t1);
        let c2 = h2.position.widen() + h2.velocity.widen() * t2;
        let v = (c2 - c1) / (t2 - t1);
        let p = c1 - v * t1;
        (p.x + p.y + p.z) as usize
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

    #[test]
    fn example_part2() {
        let s = include_str!("../assets/example.txt");
        let puzzle: Puzzle = s.parse().unwrap();
        assert_eq!(puzzle.part2(), 47);
    }
}
