use core::panic;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    i64, usize,
};

use crate::disance_iter::DistanceIter;

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
        self.fast_reachable(PART2_STEPS)
    }

    fn reachable(self, steps: usize) -> usize {
        self.filtered_reachable(steps, |p| self.garden.contains(p))
    }

    fn reachable_wrapped(&self, steps: usize) -> usize {
        self.filtered_reachable(steps, |p| self.contains_wrapped(p))
    }

    fn fast_reachable(&self, steps: usize) -> usize {
        let unstable_area = 4;
        let distances = self.distances(usize::MAX, |position| {
            let chunks = self.chunk(position);
            chunks.0.abs() + chunks.1.abs() <= unstable_area && self.contains_wrapped(position)
        });
        let mut chunked_distances: HashMap<(i64, i64), HashMap<(i64, i64), usize>> = HashMap::new();
        let mut chunk_maxs: HashMap<(i64, i64), usize> = HashMap::new();
        for chunk_distance in 0..=unstable_area {
            for chunk in DistanceIter::from(chunk_distance) {
                let chunked: HashMap<_, _> = distances
                    .iter()
                    .filter(|(position, _)| self.chunk(&position) == chunk)
                    .map(|(position, distance)| {
                        (
                            (
                                position.0.rem_euclid(self.rows),
                                position.1.rem_euclid(self.columns),
                            ),
                            *distance,
                        )
                    })
                    .collect();
                chunk_maxs.insert(
                    chunk,
                    distances
                        .iter()
                        .filter(|(position, _)| self.chunk(&position) == chunk)
                        .map(|(_, distance)| *distance)
                        .max()
                        .unwrap(),
                );
                chunked_distances.insert(chunk, chunked);
            }
        }
        let offset = (
            self.offset(
                chunked_distances.get(&(unstable_area - 1, 0)).unwrap(),
                chunked_distances.get(&(unstable_area, 0)).unwrap(),
            ),
            self.offset(
                chunked_distances.get(&(0, unstable_area - 1)).unwrap(),
                chunked_distances.get(&(0, unstable_area)).unwrap(),
            ),
        );
        assert_eq!(offset.0, offset.1);
        let offset = offset.0;
        let even_count = chunked_distances
            .get(&(0, 0))
            .unwrap()
            .values()
            .filter(|value| *value % 2 == steps % 2)
            .count();
        let odd_count = self.garden.len() - even_count;
        let max = chunk_maxs.values().max().unwrap();
        let (mut distance, mut count) = steps
            .checked_sub(*max)
            .map(|remaining_steps| {
                let distance_diff = remaining_steps / offset;
                let distance = unstable_area + distance_diff as i64;
                let count = (0..distance)
                    .map(|distance| (if distance == 0 { 1 } else { distance * 4 }, distance % 2))
                    .map(|(chunk_count, parity)| {
                        chunk_count as usize * if parity == 0 { even_count } else { odd_count }
                    })
                    .sum();
                (distance, count)
            })
            .unwrap_or((0, 0));
        loop {
            let initial_count = count;
            let chunk_parity = distance % 2;
            for mut chunk in DistanceIter::from(distance) {
                let mut delta = 0;
                while chunked_distances.get(&chunk).is_none() {
                    if chunk.0 < -1 {
                        chunk.0 += 1;
                        delta += offset;
                    } else if chunk.0 > 1 {
                        chunk.0 -= 1;
                        delta += offset;
                    } else if chunk.1 < -1 {
                        chunk.1 += 1;
                        delta += offset;
                    } else if chunk.1 > 1 {
                        chunk.1 -= 1;
                        delta += offset;
                    } else {
                        panic!("invalid state");
                    }
                }
                let distances = chunked_distances.get(&chunk).unwrap();
                let chunk_max = chunk_maxs.get(&chunk).unwrap();
                if delta + chunk_max <= steps {
                    count += if chunk_parity == 0 {
                        even_count
                    } else {
                        odd_count
                    };
                } else {
                    count += distances
                        .values()
                        .filter(|&distance| distance + delta <= steps)
                        .filter(|&distance| (distance + delta) % 2 == steps % 2)
                        .count();
                }
            }
            if initial_count == count {
                break;
            }
            distance += 1;
        }
        count
    }

    fn offset(&self, near: &HashMap<(i64, i64), usize>, far: &HashMap<(i64, i64), usize>) -> usize {
        let offsets: HashSet<_> = (0..self.rows)
            .flat_map(|row| (0..self.columns).map(move |column| (row, column)))
            .filter_map(|position| {
                let near = near.get(&position);
                let far = far.get(&position);
                match (far, near) {
                    (Some(far), Some(near)) => Some(far - near),
                    _ => None,
                }
            })
            .collect();
        assert_eq!(offsets.len(), 1);
        offsets.into_iter().next().unwrap()
    }

    fn chunk(&self, position: &(i64, i64)) -> (i64, i64) {
        let &(row, column) = position;
        (
            if row >= 0 {
                row / self.rows
            } else {
                (row - self.rows + 1) / self.rows
            },
            if column >= 0 {
                column / self.columns
            } else {
                (column - self.columns + 1) / self.columns
            },
        )
    }

    fn filtered_reachable<F>(&self, steps: usize, func: F) -> usize
    where
        F: Fn(&(i64, i64)) -> bool,
    {
        let distances = self.distances(steps, &func);
        let parity = steps % 2;
        distances
            .values()
            .into_iter()
            .filter(|&distance| distance % 2 == parity)
            .count()
    }

    fn distances<F>(&self, steps: usize, func: F) -> HashMap<(i64, i64), usize>
    where
        F: Fn(&(i64, i64)) -> bool,
    {
        let mut queue = VecDeque::new();
        queue.push_back(self.start.clone());
        let mut distances = HashMap::new();
        distances.insert(self.start.clone(), 0);
        while let Some(position) = queue.pop_front() {
            let new_distance = distances.get(&position).unwrap() + 1;
            let (row, column) = position;
            if new_distance <= steps {
                for adjacent in [
                    (row + 1, column),
                    (row - 1, column),
                    (row, column + 1),
                    (row, column - 1),
                ]
                .into_iter()
                .filter(&func)
                {
                    if distances
                        .get(&adjacent)
                        .map(|&old_distance| new_distance < old_distance)
                        .unwrap_or(true)
                    {
                        distances.insert(adjacent, new_distance);
                        queue.push_back(adjacent)
                    }
                }
            }
        }
        distances
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

    #[test]
    fn chunk_calculations() {
        let s = "...\n.S.\n...";
        let puzzle: Puzzle = s.parse().unwrap();
        assert_eq!(puzzle.chunk(&(-6, 0)).0, -2);
        assert_eq!(puzzle.chunk(&(-5, 0)).0, -2);
        assert_eq!(puzzle.chunk(&(-4, 0)).0, -2);
        assert_eq!(puzzle.chunk(&(-3, 0)).0, -1);
        assert_eq!(puzzle.chunk(&(-2, 0)).0, -1);
        assert_eq!(puzzle.chunk(&(-1, 0)).0, -1);
        assert_eq!(puzzle.chunk(&(0, 0)).0, 0);
        assert_eq!(puzzle.chunk(&(1, 0)).0, 0);
        assert_eq!(puzzle.chunk(&(2, 0)).0, 0);
        assert_eq!(puzzle.chunk(&(3, 0)).0, 1);
        assert_eq!(puzzle.chunk(&(4, 0)).0, 1);
        assert_eq!(puzzle.chunk(&(5, 0)).0, 1);
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

        #[ignore = "slow"]
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

        mod fast {
            use super::*;

            #[test]
            fn example1() {
                let puzzle: Puzzle = EXAMPLE.parse().unwrap();
                assert_eq!(puzzle.fast_reachable(6), 16);
            }

            #[test]
            fn example2() {
                let puzzle: Puzzle = EXAMPLE.parse().unwrap();
                assert_eq!(puzzle.fast_reachable(10), 50);
            }

            #[test]
            fn example3() {
                let puzzle: Puzzle = EXAMPLE.parse().unwrap();
                assert_eq!(puzzle.fast_reachable(50), 1594);
            }

            #[test]
            fn example4() {
                let puzzle: Puzzle = EXAMPLE.parse().unwrap();
                assert_eq!(puzzle.fast_reachable(100), 6536);
            }

            #[test]
            fn example5() {
                let puzzle: Puzzle = EXAMPLE.parse().unwrap();
                assert_eq!(puzzle.fast_reachable(500), 167004);
            }

            #[test]
            fn example6() {
                let puzzle: Puzzle = EXAMPLE.parse().unwrap();
                assert_eq!(puzzle.fast_reachable(1000), 668697);
            }

            #[test]
            fn example7() {
                let puzzle: Puzzle = EXAMPLE.parse().unwrap();
                assert_eq!(puzzle.fast_reachable(5000), 16733044);
            }
        }
    }
}
