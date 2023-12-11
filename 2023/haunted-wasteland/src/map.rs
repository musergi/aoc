use crate::{instruction::Instruction, line::Line};
use aoc::lcm::Lcm;
use std::str::FromStr;

pub struct Map {
    instructions: Vec<Instruction>,
    lines: Vec<Line>,
}

impl Map {
    pub fn steps(&self) -> usize {
        self.steps_end_condition("AAA", |position| position == "ZZZ")
    }

    pub fn steps_all(&self) -> usize {
        self.lines
            .iter()
            .map(|line| &line.source)
            .filter(|source| source.chars().last().unwrap() == 'A')
            .map(|source| {
                self.steps_end_condition(source, |postition| {
                    postition.chars().last().unwrap() == 'Z'
                })
            })
            .reduce(|a, b| a.lcm(b).unwrap())
            .unwrap()
    }

    fn steps_end_condition(&self, start: &str, condition: fn(&str) -> bool) -> usize {
        let mut position = start;
        std::iter::repeat(self.instructions.iter())
            .flat_map(|iter| iter)
            .take_while(|instr| {
                if condition(position) {
                    false
                } else {
                    let line = self
                        .lines
                        .iter()
                        .find(|line| &line.source == position)
                        .unwrap();
                    position = match instr {
                        Instruction::Left => &line.left,
                        Instruction::Right => &line.right,
                    };
                    true
                }
            })
            .count()
    }
}

impl FromStr for Map {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.lines();
        let instructions = it
            .next()
            .ok_or("instructions lines missing")?
            .chars()
            .map(|c| Instruction::try_from(c))
            .collect::<Result<_, _>>()?;
        if !it.next().ok_or("missing empty line")?.is_empty() {
            return Err("missing required white line");
        }
        let lines = it
            .map(|line| line.parse::<Line>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Map {
            instructions,
            lines,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Map;

    #[test]
    fn example1() {
        let string = include_str!("../assets/example1.txt");
        let map: Map = string.parse().unwrap();
        assert_eq!(map.steps(), 2);
    }

    #[test]
    fn example2() {
        let string = include_str!("../assets/example2.txt");
        let map: Map = string.parse().unwrap();
        assert_eq!(map.steps(), 6);
    }

    #[test]
    fn example3() {
        let string = include_str!("../assets/example3.txt");
        let map: Map = string.parse().unwrap();
        assert_eq!(map.steps_all(), 6);
    }
}
