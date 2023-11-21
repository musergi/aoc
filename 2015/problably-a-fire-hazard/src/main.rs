use std::{fmt::Display, num::ParseIntError, str::FromStr};

fn main() {
    aoc::aoc_main(aoc::input!(), part1, part2)
}

fn part1(input: &str) -> impl Display {
    match input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Instruction>, _>>()
        .map(|instrs| {
            (0..1000000)
                .filter(|idx| get_state_part1(&instrs, *idx))
                .count()
        })
        .map(|v| v.to_string())
    {
        Ok(v) => v,
        Err(err) => err,
    }
}

fn part2(input: &str) -> impl Display {
    match input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Instruction>, _>>()
        .map(|instrs| {
            (0..1000000)
                .map(|idx| get_state_part2(&instrs, idx))
                .sum()
        })
        .map(|v: u32| v.to_string())
    {
        Ok(v) => v,
        Err(err) => err,
    }
}

struct Instruction {
    src: (usize, usize),
    dst: (usize, usize),
    action: Action,
}

impl Instruction {
    fn contains(&self, idx: usize) -> bool {
        let p = (idx % 1000, idx / 1000);
        self.src.0 <= p.0 && p.0 <= self.dst.0 && self.src.1 <= p.1 && p.1 <= self.dst.1
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (action, suffix) = prefix(s).ok_or_else(|| format!("unknown prefix: {s}"))?;
        let (src_s, dst_s) = suffix
            .split_once(" through ")
            .ok_or_else(|| format!("missing split word: {suffix}"))?;
        let src = parse_position(src_s)?;
        let dst = parse_position(dst_s)?;
        Ok(Instruction { src, dst, action })
    }
}

fn parse_position(s: &str) -> Result<(usize, usize), String> {
    let (p1, p2) = s
        .split_once(",")
        .ok_or_else(|| format!("missing position split: {s}"))?;
    let p1 = p1.parse().map_err(|err: ParseIntError| err.to_string())?;
    let p2 = p2.parse().map_err(|err: ParseIntError| err.to_string())?;
    Ok((p1, p2))
}

fn prefix<'a>(s: &'a str) -> Option<(Action, &'a str)> {
    s.strip_prefix("turn on ")
        .map(|s| (Action::On, s))
        .or_else(|| s.strip_prefix("turn off ").map(|s| (Action::Off, s)))
        .or_else(|| s.strip_prefix("toggle ").map(|s| (Action::Toggle, s)))
}

#[derive(Debug, PartialEq, Eq)]
enum Action {
    On,
    Off,
    Toggle,
}

fn get_state_part1(chain: &[Instruction], idx: usize) -> bool {
    let mut toggle = false;
    for instr in chain.into_iter().rev().filter(|instr| instr.contains(idx)) {
        match instr.action {
            Action::On => return if toggle { false } else { true },
            Action::Off => return if toggle { true } else { false },
            Action::Toggle => {
                toggle = !toggle;
            }
        }
    }
    if toggle {
        true
    } else {
        false
    }
}

fn get_state_part2(chain: &[Instruction], idx: usize) -> u32 {
    let mut value: u32 = 0;
    for instr in chain.into_iter().filter(|instr| instr.contains(idx)) {
        match instr.action {
            Action::On => {
                value += 1;
            }
            Action::Off => {
                value = value.saturating_sub(1);
            }
            Action::Toggle => {
                value += 2;
            }
        }
    }
    value
}

#[cfg(test)]
mod tests {
    use crate::{get_state_part1, get_state_part2, Action, Instruction};

    #[test]
    fn parse_on_instruction() {
        let instruction: Instruction = "turn on 0,0 through 999,999".parse().unwrap();
        assert_eq!(instruction.src, (0, 0));
        assert_eq!(instruction.dst, (999, 999));
        assert_eq!(instruction.action, Action::On);
    }

    #[test]
    fn parse_off_instruction() {
        let instruction: Instruction = "turn off 0,0 through 100,100".parse().unwrap();
        assert_eq!(instruction.src, (0, 0));
        assert_eq!(instruction.dst, (100, 100));
        assert_eq!(instruction.action, Action::Off);
    }

    #[test]
    fn parse_toggle_instruction() {
        let instruction: Instruction = "toggle 0,1 through 0,2".parse().unwrap();
        assert_eq!(instruction.src, (0, 1));
        assert_eq!(instruction.dst, (0, 2));
        assert_eq!(instruction.action, Action::Toggle);
    }

    #[test]
    fn simple_state_deduction() {
        let chain = vec![
            Instruction {
                src: (0, 0),
                dst: (0, 0),
                action: Action::On,
            },
            Instruction {
                src: (0, 0),
                dst: (0, 0),
                action: Action::On,
            },
            Instruction {
                src: (0, 0),
                dst: (0, 0),
                action: Action::On,
            },
        ];
        assert!(get_state_part1(&chain, 0));
    }

    #[test]
    fn single_toggle_state_deduction() {
        let chain = vec![
            Instruction {
                src: (0, 0),
                dst: (0, 0),
                action: Action::On,
            },
            Instruction {
                src: (0, 0),
                dst: (0, 0),
                action: Action::On,
            },
            Instruction {
                src: (0, 0),
                dst: (0, 0),
                action: Action::Toggle,
            },
        ];
        assert!(!get_state_part1(&chain, 0));
    }

    #[test]
    fn double_toggle_state_deduction() {
        let chain = vec![
            Instruction {
                src: (0, 0),
                dst: (0, 0),
                action: Action::On,
            },
            Instruction {
                src: (0, 0),
                dst: (0, 0),
                action: Action::Toggle,
            },
            Instruction {
                src: (0, 0),
                dst: (0, 0),
                action: Action::Toggle,
            },
        ];
        assert!(get_state_part1(&chain, 0));
    }

    #[test]
    fn part2_increment_only() {
        let chain = vec![
            Instruction {
                src: (0, 0),
                dst: (0, 0),
                action: Action::On,
            },
            Instruction {
                src: (0, 0),
                dst: (0, 0),
                action: Action::Toggle,
            },
            Instruction {
                src: (0, 0),
                dst: (0, 0),
                action: Action::Toggle,
            },
        ];
        assert_eq!(get_state_part2(&chain, 0), 5);
    }
}
