use std::{collections::HashSet, str::FromStr};

pub struct Contraption {
    tiles: Vec<Tile>,
    columns: usize,
}

impl Contraption {
    pub fn energized(&self) -> usize {
        let mut visited = HashSet::new();
        let mut stack = Vec::new();
        stack.push(((0, 0), Direction::East));
        while let Some((position, direction)) = stack.pop() {
            visited.insert((position, direction));
            let index = position.0 * self.columns + position.1;
            let current_tile = self.tiles[index];
            let (next_direction, alternative) = current_tile.next_direction(direction);
            if let Some(in_position) = self.fit_in(next_direction.apply(position)) {
                let new = (in_position, next_direction);
                if !visited.contains(&new) {
                    stack.push(new)
                }
            }
            if let Some(alternative) = alternative {
                if let Some(in_position) = self.fit_in(alternative.apply(position)) {
                    let new = (in_position, alternative);
                    if !visited.contains(&new) {
                        stack.push(new)
                    }
                }
            }
        }
        let unique_tiles: HashSet<_> = visited.into_iter().map(|(position, _)| position).collect();
        unique_tiles.len()
    }

    fn fit_in(&self, position: (i64, i64)) -> Option<(usize, usize)> {
        usize::try_from(position.0)
            .ok()
            .and_then(|row| usize::try_from(position.1).ok().map(|column| (row, column)))
            .filter(|(_, column)| *column < self.columns)
            .filter(|(row, _)| *row < self.tiles.len() / self.columns)
    }
}

impl FromStr for Contraption {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut columns = 0;
        let mut tiles = Vec::new();
        for line in s.lines() {
            columns = line.len();
            for character in line.chars() {
                tiles.push(Tile::try_from(character)?);
            }
        }
        Ok(Self { tiles, columns })
    }
}

#[derive(Clone, Copy)]
enum Tile {
    EmptySpace,
    DiagonalWest,
    DiagonalEast,
    Horizontal,
    Vertical,
}

impl Tile {
    fn next_direction(&self, direction: Direction) -> (Direction, Option<Direction>) {
        match self {
            Tile::EmptySpace => (direction, None),
            Tile::DiagonalWest => match direction {
                Direction::North => (Direction::West, None),
                Direction::South => (Direction::East, None),
                Direction::West => (Direction::North, None),
                Direction::East => (Direction::South, None),
            },
            Tile::DiagonalEast => match direction {
                Direction::North => (Direction::East, None),
                Direction::South => (Direction::West, None),
                Direction::West => (Direction::South, None),
                Direction::East => (Direction::North, None),
            },
            Tile::Horizontal => match direction {
                Direction::North | Direction::South => (Direction::West, Some(Direction::East)),
                other => (other, None),
            },
            Tile::Vertical => match direction {
                Direction::West | Direction::East => (Direction::North, Some(Direction::South)),
                other => (other, None),
            },
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::EmptySpace),
            '\\' => Ok(Self::DiagonalWest),
            '|' => Ok(Self::Vertical),
            '/' => Ok(Self::DiagonalEast),
            '-' => Ok(Self::Horizontal),
            _ => Err("invalid character for tile"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    West,
    East,
}
impl Direction {
    fn apply(&self, position: (usize, usize)) -> (i64, i64) {
        let position = (position.0 as i64, position.1 as i64);
        match self {
            Direction::North => (position.0 - 1, position.1),
            Direction::South => (position.0 + 1, position.1),
            Direction::West => (position.0, position.1 - 1),
            Direction::East => (position.0, position.1 + 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Contraption;

    #[test]
    fn energized() {
        let string = include_str!("../assets/example.txt");
        let contraption: Contraption = string.parse().unwrap();
        assert_eq!(contraption.energized(), 46);
    }
}
