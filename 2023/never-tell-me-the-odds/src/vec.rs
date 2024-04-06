use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vec3i {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Vec3i {
    pub fn xy(&self) -> Vec2i {
        let Vec3i { x, y, .. } = self;
        Vec2i { x: *x, y: *y }
    }
}

impl FromStr for Vec3i {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(", ");
        let x = parse_component(&mut it)?;
        let y = parse_component(&mut it)?;
        let z = parse_component(&mut it)?;
        Ok(Vec3i { x, y, z })
    }
}

fn parse_component<'a>(mut it: impl Iterator<Item = &'a str>) -> Result<i64, &'static str> {
    it.next()
        .ok_or("missing component")?
        .trim()
        .parse()
        .map_err(|_| "not a number")
}

impl From<(i64, i64, i64)> for Vec3i {
    fn from((x, y, z): (i64, i64, i64)) -> Self {
        Vec3i { x, y, z }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vec2i {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vec2f {
    pub x: f64,
    pub y: f64,
}

impl From<(f64, f64)> for Vec2f {
    fn from((x, y): (f64, f64)) -> Self {
        Vec2f { x, y }
    }
}

impl From<Vec2i> for Vec2f {
    fn from(value: Vec2i) -> Self {
        let Vec2i { x, y } = value;
        let x = x as f64;
        let y = y as f64;
        Vec2f { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_zero_vector() {
        let s = "0, 0, 0";
        let vec: Vec3i = s.parse().unwrap();
        assert_eq!(vec, (0, 0, 0).into());
    }

    #[test]
    fn parse_non_zero_vector() {
        let s = "1, 2, 3";
        let vec: Vec3i = s.parse().unwrap();
        assert_eq!(vec, (1, 2, 3).into());
    }

    #[test]
    fn fail_on_missing_component() {
        let s = "1, 2";
        assert!(s.parse::<Vec3i>().is_err());
    }

    #[test]
    fn fail_on_non_digit() {
        let s = "1, aa, 3";
        assert!(s.parse::<Vec3i>().is_err());
    }
}
