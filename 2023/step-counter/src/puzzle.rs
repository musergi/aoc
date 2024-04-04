use crate::{disance_iter::DistanceIter, vec::Vec2i};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    i64, usize,
};

pub struct Puzzle {
    start: Vec2i,
    garden: HashSet<Vec2i>,
    rows: i64,
    columns: i64,
}

const PART1_STEPS: usize = 64;
const PART2_STEPS: usize = 26501365;

impl Puzzle {
    fn new(start: Vec2i, rows: i64, columns: i64, garden: HashSet<Vec2i>) -> Puzzle {
        Puzzle {
            start,
            garden,
            rows,
            columns,
        }
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

    fn fast_reachable(&self, steps: usize) -> usize {
        let unstable_area = 4;
        let distances = self.distances(usize::MAX, |position| {
            let chunks = self.chunk(position);
            chunks.row.abs() + chunks.column.abs() <= unstable_area
                && self.contains_wrapped(position)
        });
        let chunks = self.chunks(&distances);
        let edge = (1..unstable_area)
            .map(|idx| chunks.get(&Vec2i::new(idx, unstable_area - idx)).unwrap())
            .collect::<Vec<_>>();
        for (f, s) in edge[..(edge.len() - 1)].iter().zip(edge[1..].iter()) {
            assert_eq!(f, s)
        }
        let offset = (
            self.offset(
                &chunks
                    .get(&(unstable_area - 1, 0).into())
                    .unwrap()
                    .distances,
                &chunks.get(&(unstable_area, 0).into()).unwrap().distances,
            ),
            self.offset(
                &chunks
                    .get(&(0, unstable_area - 1).into())
                    .unwrap()
                    .distances,
                &chunks.get(&(0, unstable_area).into()).unwrap().distances,
            ),
        );
        assert_eq!(offset.0, offset.1);
        let offset = offset.0;
        let even_count = chunks
            .get(&(0, 0).into())
            .unwrap()
            .distances
            .values()
            .filter(|value| *value % 2 == steps % 2)
            .count();
        let odd_count = chunks.get(&(0, 0).into()).unwrap().distances.len() - even_count;
        let max = chunks.values().map(|chunk| chunk.max).max().unwrap();
        let (mut distance, mut count) = steps
            .checked_sub(max)
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
        let unstable_max = DistanceIter::from(unstable_area)
            .into_iter()
            .map(|chunk| chunks.get(&chunk).unwrap().max)
            .max()
            .unwrap();
        loop {
            let initial_count = count;
            let chunk_parity = distance % 2;

            if distance <= unstable_area {
                for chunk in DistanceIter::from(distance) {
                    let distances = &chunks.get(&chunk).unwrap().distances;
                    let chunk_max = chunks.get(&chunk).unwrap().max;
                    if chunk_max <= steps {
                        count += if chunk_parity == 0 {
                            even_count
                        } else {
                            odd_count
                        };
                    } else {
                        count += distances
                            .values()
                            .filter(|&distance| *distance <= steps)
                            .filter(|&distance| (distance) % 2 == steps % 2)
                            .count();
                    }
                }
            } else {
                let delta = (distance - unstable_area) as usize * offset;
                if delta + unstable_max <= steps {
                    count += if chunk_parity == 0 {
                        even_count * 4 * distance as usize
                    } else {
                        odd_count * 4 * distance as usize
                    };
                } else {
                    // Square corners
                    count += [
                        Vec2i::new(0, unstable_area),
                        Vec2i::new(unstable_area, 0),
                        Vec2i::new(0, -unstable_area),
                        Vec2i::new(-unstable_area, 0),
                    ]
                    .map(|chunk| chunks.get(&chunk).unwrap())
                    .map(|chunk| {
                        chunk
                            .distances
                            .values()
                            .map(|distance| distance + delta)
                            .filter(|&distance| distance <= steps)
                            .filter(|&distance| distance % 2 == steps % 2)
                            .count()
                    })
                    .into_iter()
                    .sum::<usize>();
                    // edges
                    count += [
                        Vec2i::new(1, unstable_area - 1),
                        Vec2i::new(-1, unstable_area - 1),
                        Vec2i::new(1, -(unstable_area - 1)),
                        Vec2i::new(-1, -(unstable_area - 1)),
                    ]
                    .into_iter()
                    .map(|chunk| {
                        chunks
                            .get(&chunk)
                            .unwrap()
                            .distances
                            .values()
                            .map(|distance| distance + delta)
                            .filter(|&distance| distance <= steps)
                            .filter(|&distance| distance % 2 == steps % 2)
                            .count()
                    })
                    .sum::<usize>()
                        * (distance - 1) as usize;
                }
            }
            if initial_count == count {
                break;
            }
            distance += 1;
        }
        count
    }

    fn offset(&self, near: &HashMap<Vec2i, usize>, far: &HashMap<Vec2i, usize>) -> usize {
        let offsets: HashSet<_> = (0..self.rows)
            .flat_map(|row| (0..self.columns).map(move |column| (row, column)))
            .filter_map(|position| {
                let near = near.get(&position.into());
                let far = far.get(&position.into());
                match (far, near) {
                    (Some(far), Some(near)) => Some(far - near),
                    _ => None,
                }
            })
            .collect();
        assert_eq!(offsets.len(), 1);
        offsets.into_iter().next().unwrap()
    }

    fn chunks(&self, distances: &HashMap<Vec2i, usize>) -> HashMap<Vec2i, Chunk> {
        distances
            .iter()
            .fold(HashMap::new(), |mut chunks, (vec, distance)| {
                let chunk = self.chunk(vec);
                let chunk_position = vec.rem_euclid(self.rows, self.columns);
                chunks
                    .entry(chunk)
                    .or_default()
                    .update(chunk_position, *distance);
                chunks
            })
    }

    fn chunk(&self, position: &Vec2i) -> Vec2i {
        position.div_euclid(self.rows, self.columns)
    }

    fn filtered_reachable<F>(&self, steps: usize, func: F) -> usize
    where
        F: Fn(&Vec2i) -> bool,
    {
        let distances = self.distances(steps, &func);
        let parity = steps % 2;
        distances
            .values()
            .into_iter()
            .filter(|&distance| distance % 2 == parity)
            .count()
    }

    fn distances<F>(&self, steps: usize, func: F) -> HashMap<Vec2i, usize>
    where
        F: Fn(&Vec2i) -> bool,
    {
        let mut queue = VecDeque::new();
        queue.push_back(self.start.clone());
        let mut distances = HashMap::new();
        distances.insert(self.start.clone(), 0);
        while let Some(position) = queue.pop_front() {
            let new_distance = distances.get(&position).unwrap() + 1;
            if new_distance <= steps {
                for adjacent in position.adjacent().into_iter().filter(&func) {
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

    fn contains_wrapped(&self, position: &Vec2i) -> bool {
        let row = position.row.rem_euclid(self.rows);
        let column = position.column.rem_euclid(self.columns);
        self.garden.contains(&Vec2i::new(row, column))
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
                        garden.insert((row as i64, column as i64).into());
                    }
                    'S' => {
                        garden.insert((row as i64, column as i64).into());
                        start = Some((row as i64, column as i64).into());
                    }
                    _ => (),
                }
            }
        }
        let start = start.ok_or("start not found")?;
        let rows = s.lines().count() as i64;
        let columns = s.lines().map(|line| line.len()).max().ok_or("no rows")? as i64;
        Ok(Puzzle::new(start, rows, columns, garden))
    }
}

#[derive(Debug, Default, PartialEq)]
struct Chunk {
    distances: HashMap<Vec2i, usize>,
    max: usize,
}

impl Chunk {
    fn update(&mut self, vec: Vec2i, distance: usize) {
        if distance > self.max {
            self.max = distance
        }
        self.distances.insert(vec, distance);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../assets/example.txt");

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
