use crate::position::Position;

#[derive(Debug, PartialEq, Eq)]
pub enum Ice {
    North,
    South,
    East,
    West,
}

impl Ice {
    pub fn delta(&self) -> Position {
        match self {
            Ice::North => Position::new(-1, 0),
            Ice::South => Position::new(1, 0),
            Ice::East => Position::new(0, 1),
            Ice::West => Position::new(0, -1),
        }
    }

    pub fn from_char(c: char) -> Option<Ice> {
        match c {
            '>' => Some(Ice::East),
            '<' => Some(Ice::West),
            '^' => Some(Ice::North),
            'v' => Some(Ice::South),
            _ => None,
        }
    }

    pub fn all() -> [Ice; 4] {
        [Ice::North, Ice::South, Ice::East, Ice::West]
    }
}
