use std::str::FromStr;

use crate::vec::Vec3i;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Line {
    pub start: Vec3i,
    pub end: Vec3i,
}

impl Line {
    pub fn bottom(&self) -> i64 {
        self.start.z.min(self.end.z)
    }

    pub fn top(&self) -> i64 {
        self.start.z.max(self.end.z)
    }

    pub fn fall(&mut self, distance: i64) {
        self.start.z -= distance;
        self.end.z -= distance;
    }

    pub fn z_intersect(&self, other: &Self) -> bool {
        self.start.x <= other.end.x
            && self.end.x >= other.start.x
            && self.start.y <= other.end.y
            && self.end.y >= other.start.y
    }
}

impl FromStr for Line {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('~').ok_or("no separator")?;
        let start = start.parse()?;
        let end = end.parse()?;
        Ok(Line { start, end })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line() {
        let s = "0,0,0~1,1,1";
        let parsed: Line = s.parse().unwrap();
        let expected = Line {
            start: (0, 0, 0).into(),
            end: (1, 1, 1).into(),
        };
        assert_eq!(parsed, expected);
    }

    #[test]
    fn fail_parse_no_separator() {
        let s = "0,0,0";
        assert!(s.parse::<Line>().is_err());
    }

    #[test]
    fn fail_parse_vec_err() {
        let s = "0,0,b~1,1,1";
        assert!(s.parse::<Line>().is_err());
    }

    #[test]
    fn z_intersect_same_axis_overlap() {
        let l1: Line = "0,0,0~3,0,0".parse().unwrap();
        let l2: Line = "3,0,0~4,0,0".parse().unwrap();
        assert!(l1.z_intersect(&l2));
    }

    #[test]
    fn z_intersect_cross() {
        let l1: Line = "1,2,0~3,2,0".parse().unwrap();
        let l2: Line = "2,1,0~2,3,0".parse().unwrap();
        assert!(l1.z_intersect(&l2));
    }

    #[test]
    fn z_intersect_same_axis_contained() {
        let l1: Line = "0,0,0~3,0,0".parse().unwrap();
        let l2: Line = "2,0,0~2,0,0".parse().unwrap();
        assert!(l1.z_intersect(&l2));
    }

    #[test]
    fn z_intersect_not_crossing() {
        let l1: Line = "0,0,0~3,0,0".parse().unwrap();
        let l2: Line = "1,1,0~1,3,0".parse().unwrap();
        assert!(!l1.z_intersect(&l2));
    }
}
