use std::{fs, num::ParseIntError, str::FromStr};

enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn get_xs(&self, x: i32) -> Vec<i32> {
        match self {
            Instruction::Noop => vec![x],
            Instruction::Addx(_) => vec![x, x],
        }
    }

    fn update(&self, x: i32) -> i32 {
        match self {
            Instruction::Noop => x,
            Instruction::Addx(val) => x + val,
        }
    }
}

impl FromStr for Instruction {
    type Err = BadInput;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_whitespace();
        match it.next().ok_or(BadInput(s.to_string()))? {
            "addx" => Ok(Instruction::Addx(
                it.next().ok_or(BadInput(s.to_string()))?.parse()?,
            )),
            "noop" => Ok(Instruction::Noop),
            _ => Err(BadInput(s.to_string())),
        }
    }
}

#[derive(Debug)]
struct BadInput(String);

impl From<ParseIntError> for BadInput {
    fn from(err: ParseIntError) -> Self {
        BadInput(err.to_string())
    }
}

fn main() {
    let boundries = [20, 60, 100, 140, 180, 220];
    let instrs = fs::read_to_string("assets/input.txt")
        .expect("File")
        .lines()
        .map(|l| l.parse::<Instruction>().expect("Parsed Instruction"))
        .collect::<Vec<_>>();
    let mut states = vec![1];
    let mut x = 1;
    for instr in instrs.iter() {
        states.extend(instr.get_xs(x.clone()));
        x = instr.update(x);
    }
    let total = boundries
        .iter()
        .map(|b| *b as i32 * states.get(*b).unwrap())
        .sum::<i32>();
    println!("Sum: {}", total);
    for idx in 0..40 * 6 {
        if idx % 40 == 0 {
            println!()
        }
        let state = *states.get(idx + 1).expect("State");
        let idx = (idx % 40) as i32 ;
        if state - 1 == idx || state + 1 == idx || state == idx {
            print!("#")
        } else {
            print!(" ")
        }
    }
}
