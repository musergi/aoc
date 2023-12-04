use std::{fmt::Display, str::FromStr};

fn main() {
    aoc::aoc_main(aoc::input!(), part1, part2);
}

fn part1(input: &str) -> impl Display {
    get_value(input, |engine| engine.part_number_sum())
}

fn part2(input: &str) -> impl Display {
    get_value(input, |engine| engine.gear_ratios())
}

fn get_value(input: &str, func: fn(Engine) -> u32) -> impl Display {
    input
        .parse::<Engine>()
        .map(|engine| func(engine))
        .map(|num| num.to_string())
        .unwrap_or_else(|err| err.to_string())
}

struct Engine {
    symbols: Vec<(usize, usize)>,
    parts: Vec<PartNumber>,
}

impl Engine {
    fn part_number_sum(&self) -> u32 {
        self.parts
            .iter()
            .filter(|part| part.has_symbol(&self.symbols))
            .map(|part| part.value)
            .sum()
    }

    fn gear_ratios(&self) -> u32 {
        self.symbols
            .iter()
            .filter_map(|symbol| {
                let parts: Vec<_> = self
                    .parts
                    .iter()
                    .filter(|part| part.has_symbol(&[*symbol]))
                    .collect();
                if parts.len() == 2 {
                    let ratio = parts.get(0).unwrap().value * parts.get(1).unwrap().value;
                    Some(ratio)
                } else {
                    None
                }
            })
            .sum()
    }
}

impl FromStr for Engine {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut symbols = Vec::new();
        let mut parts = Vec::new();
        let mut builder = PartBuilder::default();
        for (line_number, line) in s.lines().enumerate() {
            for (c_number, c) in line.chars().enumerate() {
                if c.is_numeric() {
                    builder.add(c, c_number);
                } else if let Some(part) = builder.build(line_number)? {
                    parts.push(part);
                }
                if !c.is_alphanumeric() && c != '.' {
                    symbols.push((line_number, c_number));
                }
            }
            if let Some(part) = builder.build(line_number)? {
                parts.push(part);
            }
        }
        Ok(Engine { symbols, parts })
    }
}

#[derive(Default)]
struct PartBuilder {
    buf: Vec<char>,
    start: Option<usize>,
}

impl PartBuilder {
    fn add(&mut self, c: char, index: usize) {
        if self.start.is_none() {
            self.start = Some(index);
        }
        self.buf.push(c)
    }

    fn build(&mut self, row: usize) -> Result<Option<PartNumber>, &'static str> {
        Ok(if let Some(start) = self.start {
            let end = self.buf.len() - 1 + start;
            let bounds = (start, end);
            let value_str: String = self.buf.iter().collect();
            let value = value_str.parse().map_err(|_| "could not parse number")?;
            self.start = None;
            self.buf.clear();
            Some(PartNumber { row, bounds, value })
        } else {
            None
        })
    }
}

struct PartNumber {
    row: usize,
    bounds: (usize, usize),
    value: u32,
}
impl PartNumber {
    fn has_symbol(&self, symbols: &[(usize, usize)]) -> bool {
        for symbol in symbols {
            if symbol.0 >= self.row.saturating_sub(1)
                && symbol.0 <= self.row.saturating_add(1)
                && symbol.1 >= self.bounds.0.saturating_sub(1)
                && symbol.1 <= self.bounds.1.saturating_add(1)
            {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, Engine};

    #[test]
    fn parse_engine() {
        let s = include_str!("../assets/example.txt");
        let engine: Engine = s.parse().unwrap();
        assert!(engine.symbols.contains(&(1, 3)));
        assert!(engine.symbols.contains(&(3, 6)));
        assert!(engine.symbols.contains(&(4, 3)));
        assert!(engine.symbols.contains(&(5, 5)));
        assert!(engine.symbols.contains(&(8, 3)));
        assert!(engine.symbols.contains(&(8, 5)));

        let part = engine.parts.get(0).unwrap();
        assert_eq!(part.row, 0);
        assert_eq!(part.bounds, (0, 2));
        assert_eq!(part.value, 467);

        let part = engine.parts.get(1).unwrap();
        assert_eq!(part.row, 0);
        assert_eq!(part.bounds, (5, 7));
        assert_eq!(part.value, 114);
    }

    #[test]
    fn example_part1() {
        let s = include_str!("../assets/example.txt");
        assert_eq!(format!("{}", part1(s)), "4361");
    }

    #[test]
    fn example_part2() {
        let s = include_str!("../assets/example.txt");
        assert_eq!(format!("{}", part2(s)), "467835");
    }
}
