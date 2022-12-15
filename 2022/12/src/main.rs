use std::cmp::Ord;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::BinaryHeap;
use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Clone)]
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
        self.0 + 1 >= other.0
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

#[derive(Debug, Clone)]
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

    fn get_end(&self) -> Option<Position> {
        self.tiles
            .iter()
            .enumerate()
            .find(|(_, t)| t.is_end())
            .map(|(i, _)| Position {
                x: i % self.width,
                y: i / self.width,
            })
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

impl Traversor<Position> for Map {
    fn get_neightbors(&self, el: &Position) -> Vec<Position> {
        self.get_next(el)
    }

    fn dist(&self, l: &Position, r: &Position) -> u32 {
        (l.x.abs_diff(r.x) + l.y.abs_diff(r.y)) as u32
    }

    fn cost(&self, l: &Position, r: &Position) -> u32 {
        (l.x.abs_diff(r.y) + l.y.abs_diff(l.y)) as u32
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
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

#[derive(Debug, Eq)]
struct StarItem<T> {
    cost: u32,
    content: T,
}

impl<T> StarItem<T> {
    fn new(content: T) -> StarItem<T> {
        StarItem { cost: 0, content }
    }
}

impl<T> PartialEq for StarItem<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &StarItem<T>) -> bool {
        self.cost == other.cost && self.content == other.content
    }
}

impl<T> PartialOrd for StarItem<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &StarItem<T>) -> Option<Ordering> {
        Some(
            self.cost
                .partial_cmp(&other.cost)?
                .reverse()
                .then(self.content.partial_cmp(&other.content)?),
        )
    }
}

impl<T> Ord for StarItem<T>
where
    T: Ord,
{
    fn cmp(&self, other: &StarItem<T>) -> Ordering {
        self.cost
            .cmp(&other.cost)
            .reverse()
            .then(self.content.cmp(&other.content))
    }
}

trait Traversor<T>
where
    Self: Sized,
{
    fn get_neightbors(&self, el: &T) -> Vec<T>;
    fn dist(&self, l: &T, r: &T) -> u32;
    fn cost(&self, l: &T, r: &T) -> u32;
}

fn a_star<T, F>(start: T, goal: T, traversor: F) -> Vec<T>
where
    T: Ord + Clone,
    F: Traversor<T>,
{
    let mut open = BinaryHeap::new();
    open.push(StarItem::new(start.clone()));

    let mut came_from = BTreeMap::new();

    let mut g_score = BTreeMap::new();
    g_score.insert(start.clone(), 0);
    let mut f_score = BTreeMap::new();
    f_score.insert(start.clone(), traversor.cost(&start, &goal));

    while let Some(current) = open.pop() {
        if current.content == goal {
            return rebuild_path(came_from, &current.content);
        }
        for neighbor in traversor.get_neightbors(&current.content) {
            let tentative = g_score.get(&current.content).unwrap()
                + traversor.dist(&current.content, &neighbor);
            match g_score.get_mut(&neighbor) {
                Some(score) => {
                    if tentative < *score {
                        came_from.insert(neighbor.clone(), current.content.clone());
                        *score = tentative;
                        let f_score_neighbor = tentative + traversor.cost(&neighbor, &goal);
                        f_score.insert(neighbor.clone(), f_score_neighbor.clone());
                        if !open.iter().any(|o| o.content.eq(&neighbor)) {
                            open.push(StarItem {
                                cost: f_score_neighbor,
                                content: neighbor,
                            })
                        }
                    }
                }
                None => {
                    came_from.insert(neighbor.clone(), current.content.clone());
                    g_score.insert(neighbor.clone(), tentative);
                    let f_score_neighbor = tentative + traversor.cost(&neighbor, &goal);
                    f_score.insert(neighbor.clone(), f_score_neighbor.clone());
                    if !open.iter().any(|o| o.content.eq(&neighbor)) {
                        open.push(StarItem {
                            cost: f_score_neighbor,
                            content: neighbor,
                        })
                    }
                }
            }
        }
    }
    Vec::new()
}

fn rebuild_path<T>(came_from: BTreeMap<T, T>, start: &T) -> Vec<T>
where
    T: Ord + Clone,
{
    let mut path = Vec::new();
    path.push(start.clone());
    while let Some(current) = came_from.get(path.last().unwrap()) {
        path.push(current.clone())
    }
    path
}

fn main() {
    let map = fs::read_to_string("assets/input.txt")
        .expect("Read file")
        .parse::<Map>()
        .expect("Parsed Map");
    let start = map.get_start().expect("Starting position");
    let end = map.get_end().expect("Ending position");
    let path = a_star(start, end, map.clone());
    println!("{:?}", path);
    for i in 0..map.tiles.len() {
        if i % map.width == 0 {
            println!()
        }
        if path.contains(&Position { x: i % map.width, y: i / map.width }) {
            print!("#")
        } else {
            print!(" ")
        }
        
    }
    println!();
    println!("Number of steps: {}", path.len().checked_sub(1).unwrap_or(0));
}
