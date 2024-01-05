use crate::dijkstra::{dijkstra, State};
use std::str::FromStr;

pub struct Map {
    blocks: Vec<usize>,
    columns: usize,
}

impl Map {
    pub fn heat_loss<P: Path>(&self) -> Option<usize> {
        dijkstra(
            |path| path.next(self),
            P::default(),
            |path| path.last() == self.blocks.len() - 1,
        )
    }

    fn rows(&self) -> usize {
        self.blocks.len() / self.columns
    }

    fn get(&self, row: usize, column: usize) -> usize {
        *self.blocks.get(self.to_index(row, column)).unwrap()
    }

    fn to_coordinates(&self, index: usize) -> (usize, usize) {
        (index / self.columns, index % self.columns)
    }

    fn to_index(&self, row: usize, column: usize) -> usize {
        self.columns * row + column
    }

    fn in_bounds(&self, row: usize, column: usize) -> bool {
        column < self.columns && row < self.rows()
    }

    fn delta_matches(
        &self,
        first: Option<usize>,
        second: Option<usize>,
        direction: Direction,
    ) -> bool {
        first
            .map(|first| self.to_coordinates(first))
            .and_then(|first| {
                second
                    .map(|second| self.to_coordinates(second))
                    .map(|second| Direction::from_delta(first, second) == direction)
            })
            .unwrap_or(false)
    }
}

impl FromStr for Map {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blocks = Vec::new();
        let mut columns = 0;
        for line in s.lines() {
            columns = line.len();
            for c in line.chars() {
                let block = c.to_digit(10).ok_or("invalid digit character")? as usize;
                blocks.push(block);
            }
        }
        Ok(Map { blocks, columns })
    }
}

pub trait Path: Clone + PartialEq + Eq + PartialOrd + Ord + Default {
    fn next(&self, map: &Map) -> Vec<State<Self>>;
    fn last(&self) -> usize;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ShortPath {
    path: [Option<usize>; 4],
}

impl ShortPath {
    fn build_next(&self, map: &Map, direction: Direction) -> Option<State<Self>> {
        let (last_row, last_column) = map.to_coordinates(self.last());
        if self.is_same_than_last(map, direction) || self.over_max_consecutives(map, direction) {
            return None;
        }
        let (row, column) = direction.apply(last_row, last_column)?;
        if !map.in_bounds(row, column) {
            return None;
        }
        let index = map.to_index(row, column);
        let path = [self.path[1], self.path[2], self.path[3], Some(index)];
        let node = ShortPath { path };
        let cost = map.get(row, column);
        Some(State { cost, node })
    }

    fn is_same_than_last(&self, map: &Map, direction: Direction) -> bool {
        map.delta_matches(self.path[2], self.path[3], direction.opposite())
    }

    fn over_max_consecutives(&self, map: &Map, direction: Direction) -> bool {
        self.path
            .windows(2)
            .all(|positions| map.delta_matches(positions[0], positions[1], direction))
    }
}

impl Default for ShortPath {
    fn default() -> Self {
        let path = [None, None, None, Some(0)];
        ShortPath { path }
    }
}

impl Path for ShortPath {
    fn next(&self, map: &Map) -> Vec<State<Self>> {
        Direction::all()
            .into_iter()
            .filter_map(|direction| self.build_next(map, direction))
            .collect()
    }

    fn last(&self) -> usize {
        self.path[3].unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LongPath {
    path: [Option<usize>; 11],
}

impl LongPath {
    fn get_advance_path(&self, map: &Map, direction: Direction) -> Option<State<Self>> {
        let mut new: [Option<usize>; 4] = [None; 4];
        let mut cost = 0;
        for index in 0..4 {
            let (previous_row, previous_column) = match index {
                0 => map.to_coordinates(self.last()),
                index => map.to_coordinates(new[index - 1].unwrap()),
            };
            let (row, column) = direction
                .apply(previous_row, previous_column)
                .filter(|(row, column)| map.in_bounds(*row, *column))?;
            cost += map.get(row, column);
            new[index] = Some(map.to_index(row, column));
        }
        let mut path = [None; 11];
        path[..(11 - 4)].clone_from_slice(&self.path[11 - (11 - 4)..]);
        path[(11 - 4)..].clone_from_slice(&new);
        let node = LongPath { path };
        Some(State { cost, node })
    }

    fn last_direction(&self, map: &Map) -> Option<Direction> {
        let first = map.to_coordinates(self.path[9]?);
        let second = map.to_coordinates(self.path[10].unwrap());
        Some(Direction::from_delta(first, second))
    }

    fn continue_direction(&self, map: &Map, direction: Direction) -> Option<State<LongPath>> {
        if self
            .path
            .windows(2)
            .all(|positions| map.delta_matches(positions[0], positions[1], direction))
        {
            None
        } else {
            let mut path = self.path.clone();
            for index in 0..10 {
                path.swap(index, index + 1);
            }
            let (last_row, last_column) = map.to_coordinates(self.last());
            let (row, column) = direction
                .apply(last_row, last_column)
                .filter(|(row, column)| map.in_bounds(*row, *column))?;
            path[10] = Some(map.to_index(row, column));
            let node = LongPath { path };
            let cost = map.get(row, column);
            Some(State { cost, node })
        }
    }
}

impl Default for LongPath {
    fn default() -> Self {
        let path = [
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(0),
        ];
        LongPath { path }
    }
}

impl Path for LongPath {
    fn next(&self, map: &Map) -> Vec<State<Self>> {
        if let Some(last_direction) = self.last_direction(map) {
            let mut paths = Vec::with_capacity(3);
            if let Some(path) = self.continue_direction(map, last_direction) {
                paths.push(path)
            }
            if let Some(path) = self.get_advance_path(map, last_direction.left()) {
                paths.push(path)
            }
            if let Some(path) = self.get_advance_path(map, last_direction.right()) {
                paths.push(path)
            }
            paths
        } else {
            // Starting tile
            Direction::all()
                .into_iter()
                .filter_map(|direction| self.get_advance_path(map, direction))
                .collect()
        }
    }

    fn last(&self) -> usize {
        self.path[10].unwrap()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn all() -> [Direction; 4] {
        [Self::North, Self::South, Self::West, Self::East]
    }

    fn from_delta(
        (first_row, first_column): (usize, usize),
        (second_row, second_column): (usize, usize),
    ) -> Direction {
        if first_row == second_row {
            if first_column < second_column {
                return Direction::East;
            } else if first_column > second_column {
                return Direction::West;
            }
        } else if first_column == second_column {
            if first_row < second_row {
                return Direction::South;
            } else if first_row > second_row {
                return Direction::North;
            }
        }
        panic!("invalid delta");
    }

    fn apply(&self, row: usize, column: usize) -> Option<(usize, usize)> {
        match self {
            Direction::North => row.checked_sub(1).map(|row| (row, column)),
            Direction::South => Some((row + 1, column)),
            Direction::West => column.checked_sub(1).map(|column| (row, column)),
            Direction::East => Some((row, column + 1)),
        }
    }

    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }

    fn left(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::East => Direction::South,
        }
    }
    fn right(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::East => Direction::North,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::map::{LongPath, Path};

    use super::{Map, ShortPath};

    const EXAMPLE: &str = include_str!("../assets/example.txt");

    #[test]
    fn example_short() {
        let map: Map = EXAMPLE.parse().unwrap();
        assert_eq!(map.heat_loss::<ShortPath>().unwrap(), 102);
    }

    #[test]
    fn example_long() {
        let map: Map = EXAMPLE.parse().unwrap();
        assert_eq!(map.heat_loss::<LongPath>().unwrap(), 94);
    }

    #[test]
    fn next_initial() {
        let map: Map = EXAMPLE.parse().unwrap();
        let path = ShortPath::default();
        let next = path.next(&map);
        assert_eq!(next.len(), 2)
    }

    #[test]
    fn next_cannot_reverse() {
        let map: Map = EXAMPLE.parse().unwrap();
        let path = ShortPath {
            path: [None, None, Some(0), Some(1)],
        };
        let next = path.next(&map);
        assert_eq!(next.len(), 2);
    }

    #[test]
    fn next_no_more_than_three_blocks_strait() {
        let map: Map = EXAMPLE.parse().unwrap();
        let path = ShortPath {
            path: [Some(0), Some(1), Some(2), Some(3)],
        };
        let next = path.next(&map);
        assert_eq!(next.len(), 1)
    }

    #[test]
    fn example_path_possible() {
        let map: Map = EXAMPLE.parse().unwrap();
        let positions = [
            0, 1, 2, 15, 16, 17, 18, 5, 6, 7, 8, 21, 34, 35, 36, 49, 62, 63, 76, 89, 102, 103, 116,
            129, 142, 141, 154, 167, 168,
        ];
        let mut path = ShortPath {
            path: [None, None, None, None],
        };
        for position in positions {
            path.path.swap(0, 1);
            path.path.swap(1, 2);
            path.path.swap(2, 3);
            path.path[3] = Some(position);
            assert!(path.next(&map).len() > 0);
        }
    }
}
