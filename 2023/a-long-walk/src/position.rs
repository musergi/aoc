use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    row: i64,
    column: i64,
}

impl Position {
    pub fn new(row: i64, column: i64) -> Self {
        Position { row, column }
    }

    pub fn row(&self) -> i64 {
        self.row
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            row: self.row + rhs.row,
            column: self.column + rhs.column,
        }
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            row: self.row - rhs.row,
            column: self.column - rhs.column,
        }
    }
}
