use std::{fs, str::FromStr};

#[derive(Clone)]
struct Stack {
    crates: Vec<char>,
}

impl Stack {
    fn new() -> Stack {
        Self { crates: Vec::new() }
    }
}

#[derive(Clone)]
struct Shipyard {
    stacks: Vec<Stack>,
}

#[derive(Debug)]
enum InvalidInstructionError {
    EmptyStack,
    InvalidStart(usize),
    InvalidEnd(usize),
}

impl Shipyard {
    fn new(n: usize) -> Shipyard {
        let mut stacks = Vec::with_capacity(n);
        for _ in 0..n {
            stacks.push(Stack::new())
        }
        Shipyard { stacks }
    }

    fn apply(&mut self, inst: &Instruction) -> Result<(), InvalidInstructionError> {
        for _ in 0..inst.count {
            let value = self
                .stacks
                .get_mut(inst.start)
                .ok_or(InvalidInstructionError::InvalidStart(inst.start))?
                .crates
                .pop()
                .ok_or(InvalidInstructionError::EmptyStack)?;
            self.stacks
                .get_mut(inst.end)
                .ok_or(InvalidInstructionError::InvalidEnd(inst.end))?
                .crates
                .push(value)
        }
        Ok(())
    }

    fn apply_grouped(&mut self, inst: &Instruction) -> Result<(), InvalidInstructionError> {
        let mut temp = Vec::new();
        for _ in 0..inst.count {
            let value = self
                .stacks
                .get_mut(inst.start)
                .ok_or(InvalidInstructionError::InvalidStart(inst.start))?
                .crates
                .pop()
                .ok_or(InvalidInstructionError::EmptyStack)?;
            temp.push(value)
        }
        for _ in 0..inst.count {
            self.stacks
                .get_mut(inst.end)
                .ok_or(InvalidInstructionError::InvalidEnd(inst.end))?
                .crates
                .push(temp.pop().unwrap())
        }
        Ok(())
    }

    fn top(&self) -> String {
        self.stacks
            .iter()
            .map(|s| s.crates.last())
            .filter(|o| o.is_some())
            .map(|o| o.unwrap())
            .collect()
    }
}

#[derive(Debug)]
struct ShipyardParseError;

impl FromStr for Shipyard {
    type Err = ShipyardParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.split("\n").collect::<Vec<_>>();
        let stack_count = lines.pop().unwrap().trim().split_whitespace().count();
        let mut shipyard = Self::new(stack_count);
        for line in lines.iter().rev() {
            let stack_elements = line
                .chars()
                .into_iter()
                .enumerate()
                .filter(|(i, _)| i % 4 == 1)
                .map(|(_, v)| v)
                .enumerate();
            for (stack, element) in stack_elements {
                if element != ' ' {
                    shipyard
                        .stacks
                        .get_mut(stack)
                        .ok_or(ShipyardParseError)?
                        .crates
                        .push(element.clone());
                }
            }
        }
        Ok(shipyard)
    }
}

struct Instruction {
    count: usize,
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct InstructionParseError(String);

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits = s.split_whitespace().collect::<Vec<_>>();
        let count = splits
            .get(1)
            .ok_or(InstructionParseError(s.to_string()))?
            .parse()
            .map_err(|_| InstructionParseError(s.to_string()))?;
        let start = splits
            .get(3)
            .ok_or(InstructionParseError(s.to_string()))?
            .parse::<usize>()
            .map(|v| v - 1)
            .map_err(|_| InstructionParseError(s.to_string()))?;
        let end = splits
            .get(5)
            .ok_or(InstructionParseError(s.to_string()))?
            .parse::<usize>()
            .map(|v| v - 1)
            .map_err(|_| InstructionParseError(s.to_string()))?;
        Ok(Instruction { count, start, end })
    }
}

fn main() {
    let s = fs::read_to_string("assets/input.txt").expect("File");
    let parts = s.trim().split("\n\n").collect::<Vec<_>>();
    let shipyard = parts
        .get(0)
        .expect("Shipyard")
        .parse::<Shipyard>()
        .expect("Parsed shipyard");
    let mut first = shipyard.clone();
    for line in parts.get(1).expect("Instructions").split("\n") {
        let instruction = line.parse::<Instruction>().expect("Parsed instruction");
        first.apply(&instruction).expect("Applied instruction");
    }
    println!("Top crates: {}", first.top());
    let mut second = shipyard.clone();
    for line in parts.get(1).expect("Instructions").split("\n") {
        let instruction = line.parse::<Instruction>().expect("Parsed instruction");
        second
            .apply_grouped(&instruction)
            .expect("Applied instruction");
    }
    println!("Top crates grouped: {}", second.top());
}
