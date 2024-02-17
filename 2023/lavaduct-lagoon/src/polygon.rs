use crate::point::Point;

pub struct Polygon {
    pub points: Vec<Point>,
}

impl Polygon {
    pub fn is_in(&self, point: &Point) -> bool {
        let mut winding_number = 0;
        for i in 0..self.points.len() {
            let current_point = &self.points[i];
            let next_point = &self.points[(i + 1) % self.points.len()];

            if current_point.y <= point.y {
                if next_point.y > point.y
                    && Angle::from_points(current_point, next_point, point) == Angle::Left
                {
                    winding_number += 1;
                }
            } else if next_point.y <= point.y
                && Angle::from_points(current_point, next_point, point) == Angle::Right
            {
                winding_number -= 1;
            }

            if current_point.x == point.x
                && next_point.x == point.x
                && point.y >= current_point.y.min(next_point.y)
                && point.y <= current_point.y.max(next_point.y)
            {
                return true;
            }

            if current_point.y == point.y
                && next_point.y == point.y
                && point.x >= current_point.x.min(next_point.x)
                && point.x <= current_point.x.max(next_point.x)
            {
                return true;
            }
        }

        winding_number != 0
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Angle {
    Left,
    Right,
    Colinear,
}

impl Angle {
    fn from_points(a: &Point, b: &Point, c: &Point) -> Angle {
        let cross_product_value = a.cross_product(b, c);

        if cross_product_value > 0 {
            Angle::Left
        } else if cross_product_value < 0 {
            Angle::Right
        } else {
            Angle::Colinear
        }
    }
}
