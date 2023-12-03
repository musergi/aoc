use std::{iter::Sum, ops::Add, str::FromStr};

#[derive(Default)]
pub struct CubeSet {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl CubeSet {
    pub fn max(self, other: Self) -> Self {
        CubeSet {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue)
        }
    }
}

impl From<(u32, &str)> for CubeSet {
    fn from((count, color): (u32, &str)) -> Self {
        match color {
            "red" => CubeSet {
                red: count,
                green: 0,
                blue: 0,
            },
            "green" => CubeSet {
                red: 0,
                green: count,
                blue: 0,
            },
            "blue" => CubeSet {
                red: 0,
                green: 0,
                blue: count,
            },
            _ => panic!("invalid color"),
        }
    }
}

impl Add for CubeSet {
    type Output = CubeSet;

    fn add(self, rhs: Self) -> Self::Output {
        CubeSet {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Sum for CubeSet {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(CubeSet::default(), |l, r| l + r)
    }
}

impl FromStr for CubeSet {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.split(", ")
            .map(|s| parse_color(s))
            .map(CubeSet::from)
            .sum::<CubeSet>())
    }
}

fn parse_color<'a>(s: &'a str) -> (u32, &'a str) {
    let mut it = s.split(" ");
    let count = it.next().unwrap().parse().unwrap();
    (count, it.next().unwrap())
}

#[cfg(test)]
mod tests {
    use super::CubeSet;

    #[test]
    fn parse_three_colors() {
        let grab: CubeSet = "3 green, 4 blue, 1 red".parse().unwrap();
        assert_eq!(grab.red, 1);
        assert_eq!(grab.green, 3);
        assert_eq!(grab.blue, 4);
    }

    #[test]
    fn parse_other_three_colors() {
        let grab: CubeSet = "8 green, 6 blue, 20 red".parse().unwrap();
        assert_eq!(grab.red, 20);
        assert_eq!(grab.green, 8);
        assert_eq!(grab.blue, 6);
    }

    #[test]
    fn parse_missing_color() {
        let grab: CubeSet = "1 green, 1 blue".parse().unwrap();
        assert_eq!(grab.red, 0);
        assert_eq!(grab.green, 1);
        assert_eq!(grab.blue, 1);
    }

    #[test]
    fn parse_single_color() {
        let grab: CubeSet = "2 green".parse().unwrap();
        assert_eq!(grab.red, 0);
        assert_eq!(grab.green, 2);
        assert_eq!(grab.blue, 0);
    }
}
