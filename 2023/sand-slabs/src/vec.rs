use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vec3i {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl FromStr for Vec3i {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(',');
        let x = it
            .next()
            .ok_or("missing component")?
            .parse()
            .map_err(|_| "NaN")?;
        let y = it
            .next()
            .ok_or("missing component")?
            .parse()
            .map_err(|_| "NaN")?;
        let z = it
            .next()
            .ok_or("missing component")?
            .parse()
            .map_err(|_| "NaN")?;
        if it.next().is_none() {
            Ok(Vec3i { x, y, z })
        } else {
            Err("only 3 components expected")
        }
    }
}

impl From<(i64, i64, i64)> for Vec3i {
    fn from((x, y, z): (i64, i64, i64)) -> Self {
        Vec3i { x, y, z }
    }
}

#[cfg(test)]
mod tests {
    use crate::vec::Vec3i;

    #[test]
    fn parse_empty() {
        let s = "0,0,0";
        let parsed: Vec3i = s.parse().unwrap();
        assert_eq!(parsed, (0, 0, 0).into());
    }

    #[test]
    fn parse_unit() {
        let s = "1,1,1";
        let parsed: Vec3i = s.parse().unwrap();
        assert_eq!(parsed, (1, 1, 1).into());
    }

    #[test]
    fn parse_distinct() {
        let s = "1,3,2";
        let parsed: Vec3i = s.parse().unwrap();
        assert_eq!(parsed, (1, 3, 2).into());
    }

    #[test]
    fn fail_parse_missing() {
        let s = "1,3";
        assert!(s.parse::<Vec3i>().is_err());
    }

    #[test]
    fn fail_parse_too_long() {
        let s = "1,3,1,2";
        assert!(s.parse::<Vec3i>().is_err());
    }

    #[test]
    fn fail_parse_not_number() {
        let s = "1,3,bb";
        assert!(s.parse::<Vec3i>().is_err());
    }
}
