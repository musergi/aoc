use std::str::FromStr;

struct Stack {
    crates: Vec<char>,
}

impl Stack {
    fn new() -> Stack {
        Self { crates: Vec::new() }
    }
}

struct Shipyard {
    stacks: Vec<Stack>
}

struct ShipyardParseError;

impl FromStr for Shipyard {
    type Err = ShipyardParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.split("\n").collect::<Vec<_>>();
        let stack_count = lines.get(lines.len() - 1).unwrap().trim().split_whitespace().count();
        let mut stacks = Vec::with_capacity(stack_count);
        
        Ok(Shipyard { stacks })
    }
}

fn main() {
    println!("Hello, world!");
}
