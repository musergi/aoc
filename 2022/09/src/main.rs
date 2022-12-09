use std::ops::{Add, Sub, AddAssign};

#[derive(Debug, Clone, PartialEq, Default)]
struct Coordinates {
    x: i32,
    y: i32,
}

impl Add for Coordinates {
    type Output = Coordinates;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        Coordinates { x, y }
    }
}

impl AddAssign for Coordinates {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Coordinates {
    type Output = Coordinates;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        Coordinates { x, y }
    }
}

#[derive(Debug, Default)]
struct Bridge {
    head: Coordinates,
    tail: Coordinates,
}

impl Bridge {
    fn updated(mut self, delta: Coordinates) -> Bridge {
        self.head += delta;
        if self.head - self.tail {
            
        }
    }
}

fn main() {}
