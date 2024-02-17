#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl TryFrom<char> for Direction {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'R' => Ok(Self::East),
            'L' => Ok(Self::West),
            'U' => Ok(Self::North),
            'D' => Ok(Self::South),
            _ => Err("invalid char for direction"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::direction::Direction;

    #[test]
    fn parse_north() {
        let direction: Direction = 'U'.try_into().unwrap();
        assert_eq!(direction, Direction::North);
    }

    #[test]
    fn parse_south() {
        let direction: Direction = 'D'.try_into().unwrap();
        assert_eq!(direction, Direction::South);
    }

    #[test]
    fn parse_east() {
        let direction: Direction = 'R'.try_into().unwrap();
        assert_eq!(direction, Direction::East);
    }

    #[test]
    fn parse_west() {
        let direction: Direction = 'L'.try_into().unwrap();
        assert_eq!(direction, Direction::West);
    }

    #[test]
    fn parse_err() {
        assert!(Direction::try_from('S').is_err());
    }
}
