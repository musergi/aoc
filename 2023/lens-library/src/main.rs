use hash::hash;
use hashmap::HashMap;
use instruction::Instruction;
use std::fmt::Display;

mod hash;
mod hashmap;
mod instruction;

fn main() {
    aoc::aoc_main(aoc::input!(), part1, part2)
}

fn part1(input: &str) -> impl Display {
    input
        .trim()
        .split(",")
        .map(|string| hash(string) as u64)
        .sum::<u64>()
}

fn part2(input: &str) -> impl Display {
    try_part2(input)
        .map(|v| v.to_string())
        .unwrap_or_else(|err| err.to_string())
}

fn try_part2(input: &str) -> Result<impl Display, &'static str> {
    let mut map = HashMap::default();
    for instruction_string in input.trim().split(",") {
        let instruction = Instruction::parse(instruction_string)?;
        match instruction {
            Instruction::Insert {
                label,
                focal_length,
            } => map.insert(label, focal_length),
            Instruction::Remove { label } => map.remove(label),
        }
    }
    Ok(map.focusing_power())
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn example_part1() {
        let string = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part1(string).to_string(), "1320");
    }

    #[test]
    fn example_part2() {
        let string = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part2(string).to_string(), "145");
    }
}
