use crate::{ice::Ice, position::Position};
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

pub struct Puzzle {
    open: HashSet<Position>,
    ice: HashMap<Position, Ice>,
    start: Position,
    end: Position,
}

impl Puzzle {
    pub fn part1(self) -> usize {
        self.paths()
            .into_iter()
            .map(|path| path.len() - 1)
            .max()
            .unwrap_or(0)
    }

    fn paths(&self) -> Vec<Vec<Position>> {
        let mut open_paths = Vec::new();
        let mut closed_paths = Vec::new();
        open_paths.push(vec![self.start.clone()]);
        while let Some(open_path) = open_paths.pop() {
            let last_position = open_path.last().unwrap();
            for next_step in Ice::all()
                .map(|ice| ice.delta())
                .map(|delta| *last_position + delta)
                .into_iter()
                .filter_map(|new| self.next_path(&open_path, new))
            {
                let (paths, new) = match next_step {
                    NextStep::Closed(new) => (&mut closed_paths, new),
                    NextStep::Open(new) => (&mut open_paths, new),
                };
                let mut open_path = open_path.clone();
                open_path.push(new);
                paths.push(open_path);
            }
        }
        closed_paths
    }

    fn next_path(&self, path: &[Position], new: Position) -> Option<NextStep> {
        if new == self.end {
            Some(NextStep::Closed(new))
        } else if !path.contains(&new)
            && self.valid(&new)
            && self
                .ice
                .get(&new)
                .map(|ice| ice.delta())
                .map(|delta| new - *path.last().unwrap() == delta)
                .unwrap_or(true)
        {
            Some(NextStep::Open(new))
        } else {
            None
        }
    }

    fn valid(&self, position: &Position) -> bool {
        self.open.contains(position) || self.ice.contains_key(position)
    }
}

impl FromStr for Puzzle {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut open = HashSet::new();
        let mut ice = HashMap::new();
        for (row, line) in s.lines().enumerate() {
            for (column, char) in line.chars().enumerate() {
                if let Some(new) = Ice::from_char(char) {
                    ice.insert(
                        Position::new(
                            row.try_into().map_err(|_| "row overflow")?,
                            column.try_into().map_err(|_| "column overflow")?,
                        ),
                        new,
                    );
                } else if char == '.' {
                    open.insert(Position::new(
                        row.try_into().map_err(|_| "row overflow")?,
                        column.try_into().map_err(|_| "column overflow")?,
                    ));
                }
            }
        }
        let start = open
            .iter()
            .min_by_key(|key| key.row())
            .ok_or("start not found")?
            .clone();
        let end = open
            .iter()
            .max_by_key(|key| key.row())
            .ok_or("start not found")?
            .clone();
        Ok(Puzzle {
            open,
            ice,
            start,
            end,
        })
    }
}

enum NextStep {
    Open(Position),
    Closed(Position),
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../assets/example.txt");

    #[test]
    fn parse_open() {
        let s = "#.##\n#..#\n#..#\n##.#";
        let puzzle: Puzzle = s.parse().unwrap();
        assert_eq!(puzzle.open.len(), 6);
        assert!(puzzle.open.contains(&Position::new(0, 1)));
        assert!(puzzle.open.contains(&Position::new(1, 1)));
        assert!(puzzle.open.contains(&Position::new(1, 2)));
        assert!(puzzle.open.contains(&Position::new(2, 1)));
        assert!(puzzle.open.contains(&Position::new(2, 2)));
        assert!(puzzle.open.contains(&Position::new(3, 2)));
    }

    #[test]
    fn parse_start() {
        let s = "#.##\n#..#\n#..#\n##.#";
        let puzzle: Puzzle = s.parse().unwrap();
        assert_eq!(puzzle.start, Position::new(0, 1));
    }

    #[test]
    fn parse_end() {
        let s = "#.##\n#..#\n#..#\n##.#";
        let puzzle: Puzzle = s.parse().unwrap();
        assert_eq!(puzzle.end, Position::new(3, 2));
    }

    #[test]
    fn parse_ice() {
        let s = "#.##\n#><#\n#v^#\n##.#";
        let puzzle: Puzzle = s.parse().unwrap();
        assert_eq!(puzzle.ice.len(), 4);
        assert_eq!(puzzle.ice.get(&Position::new(1, 1)).unwrap(), &Ice::East);
        assert_eq!(puzzle.ice.get(&Position::new(1, 2)).unwrap(), &Ice::West);
        assert_eq!(puzzle.ice.get(&Position::new(2, 1)).unwrap(), &Ice::South);
        assert_eq!(puzzle.ice.get(&Position::new(2, 2)).unwrap(), &Ice::North);
    }

    #[test]
    fn single_solution_part1() {
        let s = "#.##\n#..#\n##.#\n##.#";
        let puzzle: Puzzle = s.parse().unwrap();
        assert_eq!(puzzle.part1(), 4);
    }

    #[test]
    fn only_can_fall_solution_part1() {
        let s = "#.##\n#..#\n##^#\n##.#";
        let puzzle: Puzzle = s.parse().unwrap();
        assert_eq!(puzzle.paths().len(), 0);
    }

    #[test]
    fn example_part1() {
        let puzzle: Puzzle = EXAMPLE.parse().unwrap();
        assert_eq!(puzzle.part1(), 94);
    }
}
