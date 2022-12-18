use std::{collections::HashSet, fs, num::ParseIntError, ops::Add, str::FromStr};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn down() -> Position {
        Position { x: 0, y: 1 }
    }

    fn down_left() -> Position {
        Position { x: -1, y: 1 }
    }

    fn down_right() -> Position {
        Position { x: 1, y: 1 }
    }
}

impl Add for &Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Position {
    fn of_line(start: &Position, end: &Position) -> Vec<Position> {
        if start.x == end.x {
            let min = start.y.min(end.y);
            let max = start.y.max(end.y);
            (min..=max).map(|y| Position { x: start.x, y }).collect()
        } else {
            let min = start.x.min(end.x);
            let max = start.x.max(end.x);
            (min..=max).map(|x| Position { x, y: start.y }).collect()
        }
    }
}

#[derive(Debug)]
enum PositionParseError {
    MissingComponent(String),
    ParseIntError(ParseIntError),
}

impl From<ParseIntError> for PositionParseError {
    fn from(err: ParseIntError) -> Self {
        PositionParseError::ParseIntError(err)
    }
}

impl FromStr for Position {
    type Err = PositionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.trim().split(",").map(|n| n.parse::<i32>());
        let x = it
            .next()
            .ok_or(PositionParseError::MissingComponent(s.to_string()))??;
        let y = it
            .next()
            .ok_or(PositionParseError::MissingComponent(s.to_string()))??;
        Ok(Position { x, y })
    }
}

#[derive(Debug, Clone)]
struct Map {
    rock_tiles: HashSet<Position>,
    sand_tiles: HashSet<Position>,
}

impl Map {
    fn add_sand_tile(&mut self, mut starting: Position, void_y: i32) -> bool {
        let mut stopped = false;
        let moves = [
            Position::down(),
            Position::down_left(),
            Position::down_right(),
        ];
        while !stopped {
            match moves.iter().find_map(|m| {
                let new_p = m + &starting;
                if self.contains(&new_p) {
                    None
                } else {
                    Some(new_p)
                }
            }) {
                Some(new_p) => starting = new_p,
                None => stopped = true,
            }
            if starting.y > void_y {
                return false;
            }
        }
        self.sand_tiles.insert(starting);
        true
    }

    fn add_sand_tile_ground(&mut self, mut starting: Position, ground: i32) -> bool {
        let mut stopped = false;
        let mut first = true;
        let moves = [
            Position::down(),
            Position::down_left(),
            Position::down_right(),
        ];
        while !stopped {
            match moves.iter().find_map(|m| {
                let new_p = m + &starting;
                if self.contains(&new_p) || new_p.y == ground {
                    None
                } else {
                    Some(new_p)
                }
            }) {
                Some(new_p) => {
                    starting = new_p;
                    first = false;
                },
                None => stopped = true,
            }
        }
        self.sand_tiles.insert(starting);
        !first
    }

    fn contains(&self, p: &Position) -> bool {
        self.rock_tiles.contains(p) || self.sand_tiles.contains(p)
    }
}

fn main() {
    let rock_tiles = fs::read_to_string("assets/input.txt")
        .expect("Read file")
        .lines()
        .flat_map(|line| {
            let positions = line
                .trim()
                .split("->")
                .map(|p| p.parse::<Position>())
                .collect::<Result<Vec<_>, _>>()
                .expect("Parsed positions");
            positions
                .windows(2)
                .flat_map(|p| Position::of_line(p.get(0).unwrap(), p.get(1).unwrap()))
                .collect::<HashSet<_>>()
        })
        .collect::<HashSet<_>>();
    let mut map = Map {
        rock_tiles,
        sand_tiles: HashSet::new(),
    };
    let void_y = map.rock_tiles.iter().map(|t| t.y).max().expect("Max depth");
    let mut count = 0;
    let mut f_map = map.clone();
    while f_map.add_sand_tile(Position { x: 500, y: 0 }, void_y.clone()) {
        count += 1;
    }
    println!("Fallen: {}", count);
    let mut count = 1;
    while map.add_sand_tile_ground(Position { x: 500, y: 0 }, void_y + 2) {
        count += 1;
    }
    println!("Fallen with gound: {}", count);
}
