#![allow(dead_code)]
use crate::line::Line;
use std::{
    iter::{self, repeat},
    str::FromStr,
};

pub struct Puzzle {
    lines: Vec<Line>,
}

impl Puzzle {
    pub fn part1(mut self) -> usize {
        self = self.fall();
        let supports = self.supports();
        (0..supports.len())
            .into_iter()
            .filter(|idx| {
                !supports
                    .iter()
                    .any(|supports| supports.len() == 1 && supports.contains(idx))
            })
            .count()
    }

    pub fn part2(mut self) -> usize {
        self = self.fall();
        let supports = self.supports();
        (0..supports.len())
            .into_iter()
            .map(|idx| {
                let mut removed: Vec<_> = repeat(false).take(supports.len()).collect();
                *removed.get_mut(idx).unwrap() = true;
                let mut recently_removed = vec![idx];
                while let Some(idx) = recently_removed.pop() {
                    for supported in supports.get(idx).unwrap() {
                        // If all other supports are removed
                        if supports
                            .iter()
                            .enumerate()
                            .filter(|(_, supporting)| supporting.contains(&supported))
                            .all(|(other_support, _)| *removed.get(other_support).unwrap())
                        {
                            // If not already removed
                            if !*removed.get(*supported).unwrap() {
                                recently_removed.push(*supported);
                                *removed.get_mut(*supported).unwrap() = true;
                            }
                        }
                    }
                }
                removed.into_iter().filter(|&removed| removed).count() - 1
            })
            .sum()
    }

    fn supports(&self) -> Vec<Vec<usize>> {
        let mut supports: Vec<_> = iter::repeat_with(Vec::new).take(self.lines.len()).collect();
        for (line_idx, line) in self.lines.iter().enumerate() {
            for (supporter_idx, _) in
                self.lines.iter().enumerate().filter(|(_, other)| {
                    other.top() == line.bottom() - 1 && line.z_intersect(other)
                })
            {
                supports.get_mut(supporter_idx).unwrap().push(line_idx);
            }
        }
        supports
    }

    fn fall(mut self) -> Puzzle {
        self.lines.sort_by_key(|line| line.bottom());
        for idx in 0..self.lines.len() {
            if let Some((last, previous)) = self.lines[..=idx].split_last_mut() {
                let target = previous
                    .iter()
                    .filter(|line| last.z_intersect(line))
                    .map(|line| line.top() + 1)
                    .max()
                    .unwrap_or(1);
                let bottom = last.bottom();
                last.fall(bottom - target);
            }
        }
        self
    }
}

impl FromStr for Puzzle {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .lines()
            .map(|line| line.parse())
            .collect::<Result<_, _>>()?;
        Ok(Puzzle { lines })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn single_block(z: i64) -> Line {
        Line {
            start: (1, 1, z).into(),
            end: (1, 1, z).into(),
        }
    }

    #[test]
    fn can_not_fall_beyond_ground() {
        let top = single_block(3);
        let bottom = single_block(1);
        let puzzle = Puzzle {
            lines: vec![top, bottom],
        };
        let fallen = puzzle.fall();
        assert!(fallen
            .lines
            .into_iter()
            .map(|line| line.bottom())
            .all(|height| height > 0));
    }

    #[test]
    fn already_at_ground_does_not_fall() {
        let bottom = single_block(1);
        let puzzle = Puzzle {
            lines: vec![bottom.clone()],
        };
        let fallen = puzzle.fall();
        assert_eq!(fallen.lines, vec![bottom]);
    }

    #[test]
    fn not_at_ground_falls_to_ground() {
        let bottom = single_block(2);
        let puzzle = Puzzle {
            lines: vec![bottom],
        };
        let fallen = puzzle.fall();
        assert_eq!(fallen.lines, vec![single_block(1)]);
    }

    #[test]
    fn has_block_bellow_falls_on_top() {
        let top = single_block(3);
        let bottom = single_block(1);
        let puzzle = Puzzle {
            lines: vec![top, bottom],
        };
        let fallen = puzzle.fall();
        assert!(fallen.lines.contains(&single_block(2)));
    }

    #[test]
    fn stacked_fall_in_order() {
        let top = single_block(5);
        let middle = single_block(3);
        let bottom = single_block(1);
        let puzzle = Puzzle {
            lines: vec![top, middle, bottom],
        };
        let fallen = puzzle.fall();
        assert!(fallen.lines.contains(&single_block(3)));
        assert!(fallen.lines.contains(&single_block(2)));
    }

    #[test]
    fn falling_on_column_fall_on_top_of_column() {
        let top = single_block(5);
        let bottom = Line {
            start: (1, 1, 1).into(),
            end: (1, 1, 3).into(),
        };
        let puzzle = Puzzle {
            lines: vec![top, bottom],
        };
        let fallen = puzzle.fall();
        assert!(fallen.lines.contains(&single_block(4)));
    }

    #[test]
    fn non_intersect_not_affected() {
        let first = Line {
            start: (0, 0, 2).into(),
            end: (0, 3, 2).into(),
        };
        let second = Line {
            start: (1, 0, 2).into(),
            end: (3, 0, 2).into(),
        };
        let puzzle = Puzzle {
            lines: vec![first, second],
        };
        let fallen = puzzle.fall();
        assert!(fallen
            .lines
            .iter()
            .map(|line| line.bottom())
            .all(|bottom| bottom == 1));
    }

    #[test]
    fn example_part1() {
        let s = include_str!("../assets/example.txt");
        let puzzle: Puzzle = s.parse().unwrap();
        assert_eq!(puzzle.part1(), 5);
    }

    #[test]
    fn example_part2() {
        let s = include_str!("../assets/example.txt");
        let puzzle: Puzzle = s.parse().unwrap();
        assert_eq!(puzzle.part2(), 7);
    }
}
