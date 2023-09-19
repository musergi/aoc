use std::collections::HashSet;

fn main() {
    let s = std::fs::read_to_string("assets/input.txt").unwrap();
    let valley: Valley = s.parse().unwrap();
    println!("Part 1: {}", valley.faster_crossing());
    println!("Part 2: {}", valley.faster_roundtrip());
}

const DELTAS: [Vector2; 5] = [
    Vector2::up(),
    Vector2::down(),
    Vector2::right(),
    Vector2::left(),
    Vector2::zero(),
];

#[derive(Debug, Clone)]
struct Valley {
    positions: HashSet<Vector2>,
    blizzards: Vec<Blizzard>,
}

impl Valley {
    fn faster_crossing(&self) -> u32 {
        let start = self.start();
        let end = self.end();
        self.go(start, &end).0
    }

    fn faster_roundtrip(&self) -> u32 {
        let start = self.start();
        let end = self.end();
        let mut total = 0;
        let (t, valley) = self.go(start.clone(), &end);
        total += t;
        let (t, valley) = valley.go(end.clone(), &start);
        total += t;
        let (t, _) = valley.go(start, &end);
        total + t
    }

    fn go(&self, start: Vector2, end: &Vector2) -> (u32, Self) {
        let mut moves = 0;
        let mut positions = HashSet::new();
        positions.insert(start);
        let mut valley: Valley = self.clone();
        while !positions.contains(end) {
            valley = valley.next();
            let banned: HashSet<_> = valley
                .blizzards
                .iter()
                .map(|blizzard| blizzard.position.clone())
                .collect();
            let new_positions = positions
                .iter()
                .flat_map(|position| {
                    DELTAS
                        .iter()
                        .map(|delta| position.clone() + delta.clone())
                        .filter(|new_position| {
                            valley.positions.contains(new_position)
                                && !banned.contains(new_position)
                        })
                })
                .collect();
            positions = new_positions;
            moves += 1;
        }
        (moves, valley)
    }

    fn start(&self) -> Vector2 {
        self.positions
            .iter()
            .min_by_key(|position| position.row)
            .unwrap()
            .clone()
    }

    fn end(&self) -> Vector2 {
        self.positions
            .iter()
            .max_by_key(|position| position.row)
            .unwrap()
            .clone()
    }

    fn next(&self) -> Self {
        let positions = self.positions.clone();
        let north_west = self.north_west();
        let south_east = self.south_east();
        let blizzards = self
            .blizzards
            .iter()
            .map(|blizzard| blizzard.advance(&north_west, &south_east))
            .collect();
        Self {
            positions,
            blizzards,
        }
    }

    fn north_west(&self) -> Vector2 {
        let north = self
            .positions
            .iter()
            .map(|position| position.row)
            .min()
            .unwrap()
            + 1;
        let west = self
            .positions
            .iter()
            .map(|position| position.col)
            .min()
            .unwrap();
        Vector2::new(north, west)
    }

    fn south_east(&self) -> Vector2 {
        let south = self
            .positions
            .iter()
            .map(|position| position.row)
            .max()
            .unwrap()
            - 1;
        let east = self
            .positions
            .iter()
            .map(|position| position.col)
            .max()
            .unwrap();
        Vector2::new(south, east)
    }
}

impl std::str::FromStr for Valley {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut positions = HashSet::new();
        let mut blizzards = Vec::new();

        let mut position = Vector2::zero();
        for line in s.lines() {
            for tile in line.chars() {
                parse_tile(&mut positions, &mut blizzards, &position, tile)?;
                position.col += 1;
            }
            position.row += 1;
            position.col = 0;
        }

        Ok(Self {
            positions,
            blizzards,
        })
    }
}

fn parse_tile(
    positions: &mut HashSet<Vector2>,
    blizzards: &mut Vec<Blizzard>,
    position: &Vector2,
    tile: char,
) -> Result<(), Error> {
    match tile {
        '.' => {
            positions.insert(position.clone());
            Ok(())
        }
        '>' => {
            positions.insert(position.clone());
            blizzards.push(Blizzard {
                position: position.clone(),
                delta: Vector2::right(),
            });
            Ok(())
        }
        '<' => {
            positions.insert(position.clone());
            blizzards.push(Blizzard {
                position: position.clone(),
                delta: Vector2::left(),
            });
            Ok(())
        }
        '^' => {
            positions.insert(position.clone());
            blizzards.push(Blizzard {
                position: position.clone(),
                delta: Vector2::up(),
            });
            Ok(())
        }
        'v' => {
            positions.insert(position.clone());
            blizzards.push(Blizzard {
                position: position.clone(),
                delta: Vector2::down(),
            });
            Ok(())
        }
        '#' => Ok(()),
        _ => Err("unexpected symbol".into()),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Blizzard {
    position: Vector2,
    delta: Vector2,
}
impl Blizzard {
    fn advance(&self, north_west: &Vector2, south_east: &Vector2) -> Blizzard {
        let mut position = self.position.clone() + self.delta.clone();
        let delta = self.delta.clone();
        if position.row > south_east.row {
            position.row = north_west.row;
        } else if position.row < north_west.row {
            position.row = south_east.row;
        }
        if position.col > south_east.col {
            position.col = north_west.col;
        } else if position.col < north_west.col {
            position.col = south_east.col;
        }
        Blizzard { position, delta }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Vector2 {
    row: i32,
    col: i32,
}

impl Vector2 {
    const fn new(row: i32, col: i32) -> Vector2 {
        Vector2 { row, col }
    }

    const fn zero() -> Vector2 {
        Vector2::new(0, 0)
    }

    const fn right() -> Vector2 {
        Vector2::new(0, 1)
    }

    const fn left() -> Vector2 {
        Vector2::new(0, -1)
    }

    const fn up() -> Vector2 {
        Vector2::new(-1, 0)
    }

    const fn down() -> Vector2 {
        Vector2::new(1, 0)
    }
}

impl std::ops::Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2::new(self.row + rhs.row, self.col + rhs.col)
    }
}

#[derive(Debug)]
struct Error {
    message: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        let message = value.to_string();
        Error { message }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_example_parsing() {
        let s = include_str!("../assets/simple_example.txt");
        let valley: Valley = s.parse().expect("should parse valley");
        assert!(valley.positions.contains(&Vector2::new(0, 1)));
        for i in 1..=5 {
            for j in 1..=5 {
                assert!(valley.positions.contains(&Vector2::new(i, j)))
            }
        }
        assert!(valley.positions.contains(&Vector2::new(6, 5)));
        assert_eq!(valley.blizzards.len(), 2);
        assert!(valley.blizzards.contains(&Blizzard {
            position: Vector2::new(2, 1),
            delta: Vector2::right()
        }));
        assert!(valley.blizzards.contains(&Blizzard {
            position: Vector2::new(4, 4),
            delta: Vector2::down()
        }));
    }

    #[test]
    fn regular_move() {
        let s = include_str!("../assets/simple_example.txt");
        let valley: Valley = s.parse().expect("should parse valley");
        let new_valley = valley.next();
        assert!(new_valley.blizzards.contains(&Blizzard {
            position: Vector2::new(2, 2),
            delta: Vector2::right()
        }));
        assert!(new_valley.blizzards.contains(&Blizzard {
            position: Vector2::new(5, 4),
            delta: Vector2::down()
        }));
    }

    #[test]
    fn bottom_wrapping_move() {
        let s = include_str!("../assets/simple_example.txt");
        let mut valley: Valley = s.parse().expect("should parse valley");
        for _ in 0..2 {
            valley = valley.next();
        }
        assert!(valley.blizzards.contains(&Blizzard {
            position: Vector2::new(2, 3),
            delta: Vector2::right()
        }));
        assert!(valley.blizzards.contains(&Blizzard {
            position: Vector2::new(1, 4),
            delta: Vector2::down()
        }));
    }

    #[test]
    fn superposition_move() {
        let s = include_str!("../assets/simple_example.txt");
        let mut valley: Valley = s.parse().expect("should parse valley");
        for _ in 0..3 {
            valley = valley.next();
        }
        assert!(valley.blizzards.contains(&Blizzard {
            position: Vector2::new(2, 4),
            delta: Vector2::right()
        }));
        assert!(valley.blizzards.contains(&Blizzard {
            position: Vector2::new(2, 4),
            delta: Vector2::down()
        }));
    }

    #[test]
    fn after_superposition() {
        let s = include_str!("../assets/simple_example.txt");
        let mut valley: Valley = s.parse().expect("should parse valley");
        for _ in 0..4 {
            valley = valley.next();
        }
        assert!(valley.blizzards.contains(&Blizzard {
            position: Vector2::new(2, 5),
            delta: Vector2::right()
        }));
        assert!(valley.blizzards.contains(&Blizzard {
            position: Vector2::new(3, 4),
            delta: Vector2::down()
        }));
    }

    #[test]
    fn right_wrapping() {
        let s = include_str!("../assets/simple_example.txt");
        let mut valley: Valley = s.parse().expect("should parse valley");
        for _ in 0..5 {
            valley = valley.next();
        }
        assert!(valley.blizzards.contains(&Blizzard {
            position: Vector2::new(2, 1),
            delta: Vector2::right()
        }));
        assert!(valley.blizzards.contains(&Blizzard {
            position: Vector2::new(4, 4),
            delta: Vector2::down()
        }));
    }

    #[test]
    fn fastest_route() {
        let s = include_str!("../assets/example.txt");
        let valley: Valley = s.parse().expect("should parse valley");
        assert_eq!(valley.faster_crossing(), 18);
    }

    #[test]
    fn round_trip() {
        let s = include_str!("../assets/example.txt");
        let valley: Valley = s.parse().expect("should parse valley");
        assert_eq!(valley.faster_roundtrip(), 54);
    }
}
