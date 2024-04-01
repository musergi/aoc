#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2i {
    pub row: i64,
    pub column: i64,
}

impl Vec2i {
    pub fn new(row: i64, column: i64) -> Vec2i {
        Vec2i { row, column }
    }

    pub fn div_euclid(&self, rows: i64, column: i64) -> Vec2i {
        Vec2i::new(self.row.div_euclid(rows), self.column.div_euclid(column))
    }

    pub fn rem_euclid(&self, rows: i64, column: i64) -> Vec2i {
        Vec2i::new(self.row.rem_euclid(rows), self.column.rem_euclid(column))
    }

    pub fn adjacent(&self) -> [Vec2i; 4] {
        [
            (self.row - 1, self.column).into(),
            (self.row + 1, self.column).into(),
            (self.row, self.column - 1).into(),
            (self.row, self.column + 1).into(),
        ]
    }
}

impl From<(i64, i64)> for Vec2i {
    fn from((row, column): (i64, i64)) -> Self {
        Vec2i { row, column }
    }
}
