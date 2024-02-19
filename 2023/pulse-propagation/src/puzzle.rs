use crate::{
    module::{Module, State},
    module_line::ModuleLine,
    module_type::ModuleType,
    signal::Signal,
};
use std::collections::{HashMap, VecDeque};

pub struct Puzzle {
    modules: HashMap<String, Module>,
}

impl Puzzle {
    pub fn part1(mut self) -> usize {
        let mut low_signal = 0;
        let mut high_signal = 0;
        let mut messages = VecDeque::new();
        for _ in 0..1000 {
            messages.push_front(("button".to_string(), "broadcaster".to_string(), Signal::Low));
            low_signal += 1;
            while let Some((from, target, signal)) = messages.pop_front() {
                if let Some(module) = self.modules.get_mut(&target) {
                    let signals = module.process(&from, signal);
                    low_signal += signals
                        .iter()
                        .filter(|signal| signal.1 == Signal::Low)
                        .count();
                    high_signal += signals
                        .iter()
                        .filter(|signal| signal.1 == Signal::High)
                        .count();
                    messages.extend(
                        signals
                            .into_iter()
                            .map(|(output, signal)| (target.clone(), output, signal))
                            .collect::<Vec<_>>(),
                    );
                }
            }
        }
        low_signal * high_signal
    }
}

impl std::str::FromStr for Puzzle {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut modules_lines = Vec::new();
        for line in s.lines() {
            modules_lines.push(line.parse::<ModuleLine>()?);
        }
        let mut modules = HashMap::new();
        for ModuleLine { name, module_type } in modules_lines.iter() {
            let module = match module_type {
                ModuleType::Broadcaster(targets) => Module::Broadcast(targets.clone()),
                ModuleType::FlipFlop(target) => Module::FlipFlop {
                    target: target.clone(),
                    state: State::Off,
                },
                ModuleType::Conjunction(target) => {
                    let last_signals = modules_lines
                        .iter()
                        .filter(|line| match &line.module_type {
                            ModuleType::Broadcaster(targets) => {
                                targets.iter().any(|target| target == name)
                            }
                            ModuleType::FlipFlop(target) => {
                                target.iter().any(|target| target == name)
                            }
                            ModuleType::Conjunction(target) => {
                                target.iter().any(|target| target == name)
                            }
                        })
                        .map(|line| (line.name.clone(), Signal::Low))
                        .collect();
                    Module::Conjunction {
                        target: target.clone(),
                        last_signals,
                    }
                }
            };
            modules.insert(name.to_string(), module);
        }
        Ok(Puzzle { modules })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("../assets/example1.txt");
    const EXAMPLE2: &str = include_str!("../assets/example2.txt");

    #[test]
    fn parse_single_line() {
        let s = "broadcaster -> a";
        let puzzle: Puzzle = s.parse().unwrap();
        assert_eq!(puzzle.modules.len(), 1);
    }

    #[test]
    fn parse_first_example() {
        let puzzle: Puzzle = EXAMPLE1.parse().unwrap();
        assert_eq!(puzzle.modules.len(), 5);
    }

    #[test]
    fn parse_second_example() {
        let puzzle: Puzzle = EXAMPLE2.parse().unwrap();
        assert_eq!(puzzle.modules.len(), 5);
    }

    #[test]
    fn first_example_part1() {
        let puzzle: Puzzle = EXAMPLE1.parse().unwrap();
        assert_eq!(puzzle.part1(), 32000000);
    }

    #[test]
    fn first_example_part2() {
        let puzzle: Puzzle = EXAMPLE2.parse().unwrap();
        assert_eq!(puzzle.part1(), 11687500);
    }
}
