use std::{fmt, fs, ops::Add, str::FromStr};

#[derive(Debug)]
struct Tree {
    height: u8,
}

#[derive(Debug)]
struct InvalidTreeChar(char);

impl TryFrom<char> for Tree {
    type Error = InvalidTreeChar;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let height = value
            .to_digit(10)
            .ok_or(InvalidTreeChar(value))?
            .try_into()
            .map_err(|_| InvalidTreeChar(value))?;
        Ok(Tree { height })
    }
}

#[derive(Debug)]
struct Forest {
    width: usize,
    trees: Vec<Tree>,
}

#[derive(Debug)]
struct ForestParseError;

impl FromStr for Forest {
    type Err = ForestParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trees: Vec<Tree> = s
            .lines()
            .flat_map(|l| l.chars().map(|c| -> Tree { Tree::try_from(c).unwrap() }))
            .collect::<Vec<_>>();
        let width = s.lines().next().unwrap().len();
        Ok(Forest { width, trees })
    }
}

impl fmt::Display for Forest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self
            .trees
            .windows(self.width)
            .enumerate()
            .filter_map(|(i, c)| if i % self.width == 0 { Some(c) } else { None })
        {
            for t in line {
                write!(f, "{}", t.height)?
            }
            writeln!(f, "")?
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Coords {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct Delta {
    x: i64,
    y: i64,
}

impl Delta {
    fn directions() -> Vec<Delta> {
        vec![
            Delta { x: 1, y: 0 },
            Delta { x: -1, y: 0 },
            Delta { x: 0, y: 1 },
            Delta { x: 0, y: -1 },
        ]
    }
}

impl Add for Delta {
    type Output = Delta;

    fn add(self, rhs: Self) -> Self::Output {
        Delta {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Forest {
    fn get(&self, coords: &Coords) -> &Tree {
        self.trees.get(coords.y * self.width + coords.x).unwrap()
    }

    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.trees.len() / self.width
    }

    fn get_score(&self, coords: &Coords) -> u32 {
        Delta::directions()
            .into_iter()
            .map(|d| self.get_score_from(coords, d))
            .reduce(|a, b| a * b)
            .unwrap()
    }

    fn get_score_from(&self, coords: &Coords, delta: Delta) -> u32 {
        let mut acc = (None::<u8>, None::<u8>);
        CoordIterator {
            coord: Some(coords.clone()),
            delta: delta.clone(),
            forest: self,
        }
        .map(|other| self.get(&other).height)
        .map(|v| {
            acc = (acc.1, Some(v));
            acc.clone()
        })
        .take_while(|(p, _)| p.is_none() || p.map(|h| h < self.get(coords).height).unwrap_or(true))
        .filter_map(|(_, c)| c)
        .count() as u32
    }

    fn map_index(&self, i: usize) -> Coords {
        let x = i % self.get_width();
        let y = i / self.get_height();
        Coords { x, y }
    }

    fn is_visible(&self, coords: &Coords) -> bool {
        Delta::directions()
            .into_iter()
            .any(|d| self.is_visible_from(coords, d))
    }

    fn is_visible_from(&self, coords: &Coords, delta: Delta) -> bool {
        CoordIterator {
            coord: Some(coords.clone()),
            delta: delta.clone(),
            forest: self,
        }
        .all(|other| self.get(coords).height > self.get(&other).height)
    }

    fn apply(&self, coords: Coords, delta: &Delta) -> Option<Coords> {
        let x = coords.x as i64 + delta.x;
        let x = if x >= 0 && x < self.get_width() as i64 {
            Some(x)
        } else {
            None
        }? as usize;
        let y = coords.y as i64 + delta.y;
        let y = if y >= 0 && y < self.get_height() as i64 {
            Some(y)
        } else {
            None
        }? as usize;
        Some(Coords { x, y })
    }
}

struct CoordIterator<'a> {
    coord: Option<Coords>,
    delta: Delta,
    forest: &'a Forest,
}

impl<'a> Iterator for CoordIterator<'a> {
    type Item = Coords;

    fn next(&mut self) -> Option<Self::Item> {
        self.coord = self.forest.apply(self.coord.clone().unwrap(), &self.delta);
        return self.coord.clone();
    }
}

fn main() {
    let forest = fs::read_to_string("assets/input.txt")
        .expect("File")
        .parse::<Forest>()
        .unwrap();
    let visible = forest
        .trees
        .iter()
        .enumerate()
        .map(|(i, _)| forest.map_index(i))
        .filter(|c| forest.is_visible(c))
        .count();
    println!("Visible trees: {}", visible);
    let highest = forest
        .trees
        .iter()
        .enumerate()
        .map(|(i, _)| forest.map_index(i))
        .map(|c| forest.get_score(&c))
        .max()
        .expect("Max");
    println!("Max score: {}", highest);
}
