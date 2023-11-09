use std::{
    collections::HashSet,
    fmt::Display,
    hash::Hash,
    ops::{Add, AddAssign},
};

fn main() {
    aoc::aoc_main(aoc::input!(), part1, part2);
}

fn part1(input: &str) -> impl Display {
    let mut s = HashSet::new();
    let mut position = Vec::default();
    s.insert(position);
    for c in input.chars() {
        match c.try_into() {
            Ok(offset) => {
                position += offset;
                s.insert(position);
            }
            Err(err) => {
                return err;
            }
        }
    }
    s.len().to_string()
}

fn part2(input: &str) -> impl Display {
    let mut s = HashSet::new();
    let mut selector = 0;
    let mut positions = [Vec::default(), Vec::default()];
    s.insert(positions[selector]);
    for c in input.chars() {
        match c.try_into() {
            Ok(offset) => {
                positions[selector] += offset;
                s.insert(positions[selector]);
                selector = 1 - selector;
            }
            Err(err) => {
                return err;
            }
        }
    }
    s.len().to_string()
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec {
    v: [i32; 2],
}

impl AddAssign for Vec {
    fn add_assign(&mut self, rhs: Self) {
        self.v[0] += rhs.v[0];
        self.v[1] += rhs.v[1];
    }
}

impl TryFrom<char> for Vec {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Vec { v: [0, 1] }),
            'v' => Ok(Vec { v: [0, -1] }),
            '>' => Ok(Vec { v: [1, 0] }),
            '<' => Ok(Vec { v: [-1, 0] }),
            c => Err(format!("invalid char `{c}`")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn houses_example1() {
        assert_eq!(part1(">").to_string(), "2");
    }

    #[test]
    fn houses_example2() {
        assert_eq!(part1("^>v<").to_string(), "4");
    }

    #[test]
    fn houses_example3() {
        assert_eq!(part1("^v^v^v^v^v").to_string(), "2");
    }

    #[test]
    fn robo_santa_example1() {
        assert_eq!(part2("^v").to_string(), "3");
    }

    #[test]
    fn robo_santa_example2() {
        assert_eq!(part2("^>v<").to_string(), "3");
    }

    #[test]
    fn robo_santa_example3() {
        assert_eq!(part2("^v^v^v^v^v").to_string(), "11");
    }
}
