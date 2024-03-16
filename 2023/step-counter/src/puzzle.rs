use std::{
    collections::{HashMap, HashSet, VecDeque},
    usize,
};

pub struct Puzzle {
    start: (i64, i64),
    garden: HashSet<(i64, i64)>,
    rows: i64,
    columns: i64,
}

const PART1_STEPS: usize = 64;
const PART2_STEPS: usize = 26501365;

impl Puzzle {
    fn new(start: (i64, i64), garden: HashSet<(i64, i64)>) -> Result<Puzzle, &'static str> {
        let rows = garden
            .iter()
            .map(|position| position.0)
            .max()
            .ok_or("empty garden")?
            + 1;
        let columns = garden
            .iter()
            .map(|position| position.1)
            .max()
            .ok_or("empty garden")?
            + 1;
        Ok(Puzzle {
            start,
            garden,
            rows,
            columns,
        })
    }

    pub fn part1(self) -> usize {
        self.reachable(PART1_STEPS)
    }

    pub fn part2(self) -> usize {
        self.reachable_wrapped(PART2_STEPS)
    }

    fn reachable(self, steps: usize) -> usize {
        self.reachable_with_contains(steps, |p| self.garden.contains(p))
    }

    fn reachable_wrapped(&self, steps: usize) -> usize {
        self.reachable_with_contains(steps, |p| self.contains_wrapped(p))
    }

    fn reachable_with_contains<F>(&self, steps: usize, func: F) -> usize
    where
        F: Fn(&(i64, i64)) -> bool,
    {
        let mut queue = VecDeque::new();
        queue.push_back(self.start.clone());
        let mut shortest_distance = HashMap::new();
        shortest_distance.insert(self.start.clone(), 0);

        while let Some(position) = queue.pop_front() {
            let new_distance = shortest_distance.get(&position).unwrap() + 1;
            let (row, column) = position;
            if new_distance <= steps {
                for adjacent_position in [
                    (row + 1, column),
                    (row - 1, column),
                    (row, column + 1),
                    (row, column - 1),
                ]
                .into_iter()
                .filter(&func)
                {
                    if shortest_distance
                        .get(&adjacent_position)
                        .map(|&old_distance| new_distance < old_distance)
                        .unwrap_or(true)
                    {
                        shortest_distance.insert(adjacent_position, new_distance);
                        queue.push_back(adjacent_position)
                    }
                }
            }
        }
        let parity = steps % 2;
        shortest_distance
            .values()
            .into_iter()
            .filter(|&distance| distance % 2 == parity)
            .count()
    }

    fn contains_wrapped(&self, position: &(i64, i64)) -> bool {
        let x = position.0.rem_euclid(self.rows);
        let y = position.1.rem_euclid(self.columns);
        self.garden.contains(&(x, y))
    }
}

impl std::str::FromStr for Puzzle {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = None;
        let mut garden = HashSet::new();
        for (row, line) in s.lines().enumerate() {
            for (column, symbol) in line.chars().enumerate() {
                match symbol {
                    '.' => {
                        garden.insert((row as i64, column as i64));
                    }
                    'S' => {
                        garden.insert((row as i64, column as i64));
                        start = Some((row as i64, column as i64));
                    }
                    _ => (),
                }
            }
        }
        let start = start.ok_or("start not found")?;
        Puzzle::new(start, garden)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../assets/example.txt");

    #[test]
    fn wrapped_contains_preserves_regular_behaviour() {
        let s = "S.\n#.";
        let puzzle: Puzzle = s.parse().unwrap();
        assert!(puzzle.contains_wrapped(&(0, 0)));
        assert!(!puzzle.contains_wrapped(&(1, 0)));
        assert!(puzzle.contains_wrapped(&(0, 1)));
        assert!(puzzle.contains_wrapped(&(1, 1)));
    }

    #[test]
    fn wrapped_contains_wraps_next() {
        let s = "S.\n#.";
        let puzzle: Puzzle = s.parse().unwrap();
        assert!(puzzle.contains_wrapped(&(2, 0)));
        assert!(!puzzle.contains_wrapped(&(3, 0)));
        assert!(puzzle.contains_wrapped(&(2, 1)));
        assert!(puzzle.contains_wrapped(&(3, 1)));
    }

    #[test]
    fn wrapped_contains_wraps_previous() {
        let s = "S.\n#.";
        let puzzle: Puzzle = s.parse().unwrap();
        assert!(puzzle.contains_wrapped(&(-2, 0)));
        assert!(!puzzle.contains_wrapped(&(-1, 0)));
        assert!(puzzle.contains_wrapped(&(-2, 1)));
        assert!(puzzle.contains_wrapped(&(-1, 1)));
    }

    mod part1 {
        use super::*;

        #[test]
        fn example1() {
            let puzzle: Puzzle = EXAMPLE.parse().unwrap();
            assert_eq!(puzzle.reachable(1), 2);
        }

        #[test]
        fn example2() {
            let puzzle: Puzzle = EXAMPLE.parse().unwrap();
            assert_eq!(puzzle.reachable(2), 4);
        }

        #[test]
        fn example3() {
            let puzzle: Puzzle = EXAMPLE.parse().unwrap();
            assert_eq!(puzzle.reachable(3), 6);
        }

        #[test]
        fn example4() {
            let puzzle: Puzzle = EXAMPLE.parse().unwrap();
            assert_eq!(puzzle.reachable(6), 16);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example1() {
            let puzzle: Puzzle = EXAMPLE.parse().unwrap();
            assert_eq!(puzzle.reachable_wrapped(6), 16);
        }

        #[test]
        fn example2() {
            let puzzle: Puzzle = EXAMPLE.parse().unwrap();
            assert_eq!(puzzle.reachable_wrapped(10), 50);
        }

        #[test]
        fn example3() {
            let puzzle: Puzzle = EXAMPLE.parse().unwrap();
            assert_eq!(puzzle.reachable_wrapped(50), 1594);
        }

        #[test]
        fn example4() {
            let puzzle: Puzzle = EXAMPLE.parse().unwrap();
            assert_eq!(puzzle.reachable_wrapped(100), 6536);
        }

        #[test]
        fn example5() {
            let puzzle: Puzzle = EXAMPLE.parse().unwrap();
            assert_eq!(puzzle.reachable_wrapped(500), 167004);
        }

        #[test]
        fn example6() {
            let puzzle: Puzzle = EXAMPLE.parse().unwrap();
            assert_eq!(puzzle.reachable_wrapped(1000), 668697);
        }

        #[ignore = "slow"]
        #[test]
        fn example7() {
            let puzzle: Puzzle = EXAMPLE.parse().unwrap();
            assert_eq!(puzzle.reachable_wrapped(5000), 16733044);
        }
    }
}
