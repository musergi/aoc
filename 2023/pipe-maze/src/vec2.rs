#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    pub const fn new(x: i32, y: i32) -> Vec2 {
        Vec2 { x, y }
    }

    pub const fn east() -> Vec2 {
        Vec2 { x: 1, y: 0 }
    }

    pub const fn west() -> Vec2 {
        Vec2 { x: -1, y: 0 }
    }

    pub const fn north() -> Vec2 {
        Vec2 { x: 0, y: -1 }
    }

    pub const fn south() -> Vec2 {
        Vec2 { x: 0, y: 1 }
    }
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vec2::Vec2;

    #[test]
    fn horizontal_addition() {
        assert_eq!(Vec2::new(1, 1) + Vec2::new(1, 0), Vec2::new(2, 1));
    }

    #[test]
    fn vertical_addition() {
        assert_eq!(Vec2::new(1, 1) + Vec2::new(0, 1), Vec2::new(1, 2));
    }

    #[test]
    fn horizontal_substraction() {
        assert_eq!(Vec2::new(1, 1) - Vec2::new(1, 0), Vec2::new(0, 1));
    }

    #[test]
    fn vertical_substraction() {
        assert_eq!(Vec2::new(1, 1) - Vec2::new(0, 1), Vec2::new(1, 0));
    }
}
