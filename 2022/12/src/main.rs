use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct Tile(u8);

impl Tile {
    fn to_height(c: char) -> u8 {
        c as u8 - 'a' as u8 + 1
    }

    fn is_start(&self) -> bool {
        self.0 == Tile::to_height('a') - 1
    }

    fn is_end(&self) -> bool {
        self.0 == Tile::to_height('z') + 1
    }

    fn can_go(&self, other: &Tile) -> bool {
        self.0 + 1 == other.0
    }
}

impl TryFrom<char> for Tile {
    type Error = InvalidTileChar;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Tile(Tile::to_height('a') - 1)),
            'E' => Ok(Tile(Tile::to_height('z') + 1)),
            o => {
                if o >= 'a' && o <= 'z' {
                    Ok(Tile(Tile::to_height(o)))
                } else {
                    Err(InvalidTileChar(o))
                }
            }
        }
    }
}

#[derive(Debug)]
struct InvalidTileChar(char);

#[derive(Debug)]
struct Map {
    width: usize,
    tiles: Vec<Tile>,
}

impl Map {
    fn get_start(&self) -> Option<Position> {
        self.tiles
            .iter()
            .enumerate()
            .find(|(_, t)| t.is_start())
            .map(|(i, _)| Position {
                x: i % self.width,
                y: i / self.width,
            })
    }

    fn get_next(&self, pos: &Position) -> Vec<Position> {
        [pos.up(), pos.down(), pos.left(), pos.right()]
            .into_iter()
            .filter_map(|p| p)
            .filter(|p| self.in_bounds(p))
            .filter(|p| {
                self.get_tile(pos)
                    .unwrap()
                    .can_go(self.get_tile(p).unwrap())
            })
            .collect()
    }

    fn get_tile(&self, pos: &Position) -> Option<&Tile> {
        self.tiles.get(pos.x + pos.y * self.width)
    }

    fn in_bounds(&self, pos: &Position) -> bool {
        pos.x < self.width && pos.y < self.get_height()
    }

    fn get_height(&self) -> usize {
        self.tiles.len() / self.width
    }
}

impl FromStr for Map {
    type Err = ParseMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().ok_or(ParseMapError::EmptyInput)?.len();
        let tiles = s
            .lines()
            .flat_map(|line| line.chars().map(|c| Tile::try_from(c)))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Map { width, tiles })
    }
}

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn up(&self) -> Option<Position> {
        let x = self.x;
        let y = self.y.checked_sub(1)?;
        Some(Position { x, y })
    }

    fn down(&self) -> Option<Position> {
        let x = self.x;
        let y = self.y.checked_add(1)?;
        Some(Position { x, y })
    }

    fn right(&self) -> Option<Position> {
        let x = self.x.checked_add(1)?;
        let y = self.y;
        Some(Position { x, y })
    }

    fn left(&self) -> Option<Position> {
        let x = self.x.checked_sub(1)?;
        let y = self.y;
        Some(Position { x, y })
    }
}

#[derive(Debug)]
enum ParseMapError {
    EmptyInput,
    InvalidTileChar(InvalidTileChar),
}

impl From<InvalidTileChar> for ParseMapError {
    fn from(err: InvalidTileChar) -> ParseMapError {
        ParseMapError::InvalidTileChar(err)
    }
}

fn a_star<T, F>(start: T, end: T, cost: F) -> Vec<Position>
where
    F: FnMut(T) -> u32,
{
}

fn main() {
    let map = fs::read_to_string("assets/example.txt")
        .expect("Read file")
        .parse::<Map>()
        .expect("Parsed Map");
    let start = map.get_start().expect("Starting position");
    println!("{:?} {:?}", start, map.get_next(&start));
}
