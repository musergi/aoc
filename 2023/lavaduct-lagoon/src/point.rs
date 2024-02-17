use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }

    pub fn cross_product(&self, b: &Self, c: &Self) -> i64 {
        (b.y - self.y) * (c.x - self.x) - (b.x - self.x) * (c.y - self.y)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[cfg(test)]
mod tests {
    use crate::point::Point;

    #[test]
    fn cross_product_linear() {
        let point = Point::new(0, 0);
        assert_eq!(
            point.cross_product(&Point::new(0, -1), &Point::new(0, 1)),
            0
        );
    }

    #[test]
    fn cross_product_left() {
        let point = Point::new(0, 0);
        assert!(point.cross_product(&Point::new(0, -1), &Point::new(-1, 0)) > 0);
    }

    #[test]
    fn cross_product_right() {
        let point = Point::new(0, 0);
        assert!(point.cross_product(&Point::new(0, -1), &Point::new(1, 0)) < 0);
    }
}
