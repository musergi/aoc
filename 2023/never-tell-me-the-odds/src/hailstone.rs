use crate::{
    area::Area,
    vec::{Vec2f, Vec3i},
};
use core::f64;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hailstone {
    pub position: Vec3i,
    pub velocity: Vec3i,
}

impl Hailstone {
    pub fn xy_intersect_in(&self, other: &Hailstone, area: &Area) -> bool {
        self.xy_intersect(other)
            .map(|intersection| {
                area.min <= intersection.x
                    && area.max >= intersection.x
                    && area.min <= intersection.y
                    && area.max >= intersection.y
            })
            .unwrap_or(false)
    }

    fn xy_intersect(&self, other: &Hailstone) -> Option<Vec2f> {
        let p1 = self.position.xy();
        let p2 = other.position.xy();
        let v1 = self.velocity.xy();
        let v2 = other.velocity.xy();
        let l1_num = (p1.x - p2.x) * v2.y + (p2.y - p1.y) * v2.x;
        let l1_den = v2.x * v1.y - v1.x * v2.y;
        let l2_num = (p2.x - p1.x) * v1.y - (p2.y - p1.y) * v1.x;
        let l2_den = v1.x * v2.y - v2.x * v1.y;
        if l1_den == 0
            || l2_den == 0
            || l1_num.signum() * l1_den.signum() < 0
            || l2_num.signum() * l2_den.signum() < 0
        {
            None
        } else {
            let l1 = l1_num as f64 / l1_den as f64;
            let x = p1.x as f64 + v1.x as f64 * l1;
            let y = p1.y as f64 + v1.y as f64 * l1;
            Some(Vec2f { x, y })
        }
    }
}

impl FromStr for Hailstone {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (position, velocity) = s.split_once(" @ ").ok_or("missing component")?;
        let position = position.parse().map_err(|_| "missing position")?;
        let velocity = velocity.parse().map_err(|_| "missing velocity")?;
        Ok(Hailstone { position, velocity })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const RELATIVE_ERROR: f64 = 1e-2;

    #[test]
    fn parse_hailstone() {
        let s = "1, 2, 3 @ 4, 5, 6";
        let hailstone: Hailstone = s.parse().unwrap();
        assert_eq!(
            hailstone,
            Hailstone {
                position: (1, 2, 3).into(),
                velocity: (4, 5, 6).into()
            }
        );
    }

    #[test]
    fn fail_if_bad_position() {
        let s = "1, aa, 3 @ 4, 5, 6";
        assert!(s.parse::<Hailstone>().is_err());
    }

    #[test]
    fn fail_if_bad_velocity() {
        let s = "1, 2, 3 @ aa, 5, 6";
        assert!(s.parse::<Hailstone>().is_err());
    }

    #[test]
    fn fail_if_missing_component() {
        let s = "1, 2, 3";
        assert!(s.parse::<Hailstone>().is_err());
    }

    #[test]
    fn xy_intersect() {
        let a = Hailstone {
            position: (19, 13, 30).into(),
            velocity: (-2, 1, -2).into(),
        };
        let b = Hailstone {
            position: (18, 19, 30).into(),
            velocity: (-1, -1, -2).into(),
        };
        let intersection = a.xy_intersect(&b).unwrap();
        let expected: Vec2f = (14.33333, 15.33333).into();
        relative_error(expected.x, intersection.x);
        relative_error(expected.y, intersection.y);
    }

    #[test]
    fn parallel_xy_no_intersect() {
        let a = Hailstone {
            position: (18, 19, 22).into(),
            velocity: (-1, -1, -2).into(),
        };
        let b = Hailstone {
            position: (20, 25, 34).into(),
            velocity: (-2, -2, -4).into(),
        };
        assert!(a.xy_intersect(&b).is_none());
    }

    #[test]
    fn past_crossed_xy_no_intersect() {
        let a = Hailstone {
            position: (19, 13, 30).into(),
            velocity: (-2, 1, -2).into(),
        };
        let b = Hailstone {
            position: (20, 19, 15).into(),
            velocity: (1, -5, -3).into(),
        };
        assert!(a.xy_intersect(&b).is_none());
    }

    #[test]
    fn intersect_out_of_area_false() {
        let a = Hailstone {
            position: (19, 13, 30).into(),
            velocity: (-2, 1, -2).into(),
        };
        let b = Hailstone {
            position: (12, 31, 28).into(),
            velocity: (-1, -2, -1).into(),
        };
        let area = Area {
            min: 7.0,
            max: 27.0,
        };
        assert!(!a.xy_intersect_in(&b, &area));
    }

    fn relative_error(expected: f64, actual: f64) {
        assert!(((expected - actual) / expected).abs() < RELATIVE_ERROR);
    }
}
