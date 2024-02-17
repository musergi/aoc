use crate::{direction::Direction, point::Point};

pub struct Instruction {
    direction: Direction,
    length: i64,
}

impl Instruction {
    pub fn apply(&self, source: &Point) -> Point {
        let Self {
            direction, length, ..
        } = self;
        match direction {
            Direction::North => *source + Point::new(0, *length),
            Direction::South => *source + Point::new(0, -length),
            Direction::East => *source + Point::new(*length, 0),
            Direction::West => *source + Point::new(-length, 0),
        }
    }

    pub fn parse_v1(s: &str) -> Result<Self, &'static str> {
        let mut it = s.split_whitespace();
        let direction = parse_direction(it.next().ok_or("missing trench direction")?)?;
        let length = it
            .next()
            .ok_or("missing trench length")?
            .parse()
            .map_err(|_| "could not parse trench length")?;
        Ok(Instruction { direction, length })
    }

    pub fn parse_v2(s: &str) -> Result<Self, &'static str> {
        let mut it = s.split_whitespace();
        it.next();
        it.next();
        let final_section = it
            .next()
            .ok_or("missing final section")?
            .strip_prefix("(#")
            .ok_or("missing prefix")?
            .strip_suffix(")")
            .ok_or("missing suffix")?;
        let length = i64::from_str_radix(&final_section[..5], 16).map_err(|_| "invalid length")?;
        let chars = final_section.chars();
        let direction = match chars.last().ok_or("missing direction digit")? {
            '0' => Ok(Direction::East),
            '1' => Ok(Direction::South),
            '2' => Ok(Direction::West),
            '3' => Ok(Direction::North),
            _ => Err("unkown direction"),
        }?;
        Ok(Instruction { direction, length })
    }
}

fn parse_direction(s: &str) -> Result<Direction, &'static str> {
    if s.len() != 1 {
        Err("invalid length for direction")
    } else {
        s.chars().next().unwrap().try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::Instruction;
    use crate::{direction::Direction, point::Point};

    #[test]
    fn parse_example_first_line() {
        let instruction = Instruction::parse_v1("R 6 (#70c710)").unwrap();
        assert_eq!(instruction.direction, Direction::East);
        assert_eq!(instruction.length, 6);
    }

    #[test]
    fn parse_fail_on_missing_segment() {
        assert!(Instruction::parse_v1("R").is_err());
    }

    #[test]
    fn apply_example1() {
        let start = Point::new(0, 0);
        let instruction = Instruction::parse_v1("R 6 (#70c710)").unwrap();
        assert_eq!(instruction.apply(&start), Point::new(6, 0));
    }

    #[test]
    fn apply_example2() {
        let start = Point::new(0, 0);
        let instruction = Instruction::parse_v1("D 8 (#70c710)").unwrap();
        assert_eq!(instruction.apply(&start), Point::new(0, -8));
    }

    #[test]
    fn parse_v2_first_line() {
        let instruction = Instruction::parse_v2("R 6 (#70c710)").unwrap();
        assert_eq!(instruction.direction, Direction::East);
        assert_eq!(instruction.length, 461937);
    }
}
