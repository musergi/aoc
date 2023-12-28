use crate::pattern::Pattern;
use std::{collections::HashSet, str::FromStr};

pub struct Patterns {
    patterns: Vec<Pattern>,
}

impl Patterns {
    pub fn summarize(&self) -> Result<usize, &'static str> {
        self.summarize_with(Pattern::summarize)
    }

    pub fn one_off_summarize(&self) -> Result<usize, &'static str> {
        self.summarize_with(Pattern::one_off_summarize)
    }

    fn summarize_with(
        &self,
        summarization_func: fn(&Pattern) -> Result<usize, &'static str>,
    ) -> Result<usize, &'static str> {
        self.patterns.iter().map(summarization_func).sum()
    }
}

impl FromStr for Patterns {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut patterns = Vec::new();
        let mut rocks = HashSet::new();
        let mut row = 0;
        let mut max_column = 0;
        for line in s.lines() {
            if line.is_empty() {
                patterns.push(Pattern::new(rocks.clone(), max_column + 1, row));
                rocks.clear();
                row = 0;
                max_column = 0;
            } else {
                for (column, char) in line.chars().enumerate() {
                    if char == '#' {
                        rocks.insert((row, column));
                    }
                    if column > max_column {
                        max_column = column
                    }
                }
                row += 1
            }
        }
        patterns.push(Pattern::new(rocks.clone(), max_column + 1, row));
        Ok(Self { patterns })
    }
}

#[cfg(test)]
mod tests {
    use super::Patterns;

    const EXAMPLE: &str = include_str!("../assets/example.txt");

    #[test]
    fn example_part1() {
        let patterns: Patterns = EXAMPLE.parse().unwrap();
        assert_eq!(patterns.summarize().unwrap(), 405);
    }

    #[test]
    fn first_example_summary() {
        let patterns: Patterns = EXAMPLE.parse().unwrap();
        assert_eq!(patterns.patterns.get(0).unwrap().summarize().unwrap(), 5);
    }

    #[test]
    fn second_example_summary() {
        let patterns: Patterns = EXAMPLE.parse().unwrap();
        assert_eq!(patterns.patterns.get(1).unwrap().summarize().unwrap(), 400);
    }

    #[test]
    fn example_part2() {
        let patterns: Patterns = EXAMPLE.parse().unwrap();
        assert_eq!(patterns.one_off_summarize().unwrap(), 400);
    }

    #[test]
    fn first_example_one_off_summary() {
        let patterns: Patterns = EXAMPLE.parse().unwrap();
        assert_eq!(
            patterns
                .patterns
                .get(0)
                .unwrap()
                .one_off_summarize()
                .unwrap(),
            300
        );
    }

    #[test]
    fn second_example_one_off_summary() {
        let patterns: Patterns = EXAMPLE.parse().unwrap();
        assert_eq!(
            patterns
                .patterns
                .get(1)
                .unwrap()
                .one_off_summarize()
                .unwrap(),
            100
        );
    }
}
