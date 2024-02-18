use crate::{part::Part, part_combination::PartCombination, value::Value, workflow::Workflow};

pub struct Puzzle {
    workflows: Vec<Workflow>,
    parts: Vec<Part>,
}

impl Puzzle {
    pub fn part1(self) -> u64 {
        self.parts
            .iter()
            .filter(|part| self.eval(part))
            .map(|part| part.x + part.m + part.a + part.s)
            .sum()
    }

    fn eval(&self, part: &Part) -> bool {
        self.eval_recursive(part, "in")
    }

    fn eval_recursive(&self, part: &Part, ptr: &str) -> bool {
        match self.workflow(ptr).eval(part) {
            Value::Goto(ptr) => self.eval_recursive(part, &ptr),
            Value::Accept => true,
            Value::Reject => false,
        }
    }

    pub fn part2(self) -> u64 {
        self.count_recursive(PartCombination::default(), "in")
    }

    fn count_recursive(&self, combination: PartCombination, ptr: &str) -> u64 {
        self.workflow(ptr)
            .split(combination)
            .into_iter()
            .map(|(combination, value)| match value {
                Value::Accept => combination.combinations(),
                Value::Reject => 0,
                Value::Goto(ptr) => self.count_recursive(combination, &ptr),
            })
            .sum()
    }

    fn workflow(&self, ptr: &str) -> &Workflow {
        self.workflows
            .iter()
            .find(|workflow| workflow.name == ptr)
            .unwrap()
    }
}

impl std::str::FromStr for Puzzle {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut state = ParserState::Workflows;
        let mut workflows = Vec::new();
        let mut parts = Vec::new();
        for line in s.lines() {
            if line.is_empty() {
                state = ParserState::Parts
            } else {
                match state {
                    ParserState::Workflows => workflows.push(line.parse()?),
                    ParserState::Parts => parts.push(line.parse()?),
                }
            }
        }
        Ok(Puzzle { workflows, parts })
    }
}

enum ParserState {
    Workflows,
    Parts,
}

#[cfg(test)]
mod tests {
    use super::Puzzle;

    const EXAMPLE: &str = include_str!("../assets/example.txt");

    #[test]
    fn parse_exampple_puzzle() {
        let puzzle: Puzzle = EXAMPLE.parse().unwrap();
        assert_eq!(puzzle.workflows.len(), 11);
        assert_eq!(puzzle.parts.len(), 5);
    }

    #[test]
    fn example_part1() {
        let puzzle: Puzzle = EXAMPLE.parse().unwrap();
        assert_eq!(puzzle.part1(), 19114);
    }

    #[test]
    fn example_part2() {
        let puzzle: Puzzle = EXAMPLE.parse().unwrap();
        assert_eq!(puzzle.part2(), 167409079868000);
    }
}
