use std::str::FromStr;

use crate::counter_iter::IteratorExt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpringRow {
    conditions: Vec<Option<SpringCondition>>,
    validation: Vec<usize>,
}

impl SpringRow {
    pub fn combinations(&self) -> usize {
        let mut copy = self.clone();
        copy.inner_combinations()
    }

    fn inner_combinations(&mut self) -> usize {
        match self.is_valid() {
            Some(true) => 1,
            Some(false) => 0,
            None => self.sum_both_options(),
        }
    }

    fn sum_both_options(&mut self) -> usize {
        let empty_idx = self
            .conditions
            .iter()
            .position(|condition| condition.is_none())
            .unwrap();
        self.conditions[empty_idx] = Some(SpringCondition::Operational);
        let first = self.inner_combinations();
        self.conditions[empty_idx] = Some(SpringCondition::Damaged);
        let second = self.inner_combinations();
        self.conditions[empty_idx] = None;
        first + second
    }

    fn is_valid(&self) -> Option<bool> {
        if self.conditions.iter().any(|condition| condition.is_none()) {
            None
        } else {
            let mut idx = 0;
            for (count, value) in self.conditions.iter().counts() {
                let value = value.as_ref().unwrap();
                if *value == SpringCondition::Damaged {
                    if let Some(expected) = self.validation.get(idx) {
                        if *expected != count {
                            return Some(false);
                        }
                        idx += 1;
                    } else {
                        return Some(false);
                    }
                }
            }
            Some(idx == self.validation.len())
        }
    }

    fn simplified(&self) -> Option<Self> {
        let strip_prefix = self.strip_prefix()?;
        Some(Self {
            conditions: self.conditions[strip_prefix..].iter().cloned().collect(),
            validation: self.validation.clone(),
        })
    }

    fn strip_prefix(&self) -> Option<usize> {
        let mut it = self.conditions.iter().counts();
        let first = it.next()?;
        match first.1 {
            Some(SpringCondition::Operational) => Some(first.0),
            _ => None,
        }
    }
}

impl FromStr for SpringRow {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (conditions, validation) = s
            .split_once(' ')
            .ok_or("failed to split conditions and validation")?;
        let conditions = conditions
            .chars()
            .into_iter()
            .map(|char| SpringCondition::new(char))
            .collect::<Result<_, _>>()?;
        let validation = validation
            .split(',')
            .map(|value| value.parse::<usize>().map_err(|_| "invalid number"))
            .collect::<Result<_, _>>()?;
        Ok(SpringRow {
            conditions,
            validation,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SpringCondition {
    Operational,
    Damaged,
}

impl SpringCondition {
    fn new(value: char) -> Result<Option<SpringCondition>, &'static str> {
        match value {
            '.' => Ok(Some(SpringCondition::Operational)),
            '#' => Ok(Some(SpringCondition::Damaged)),
            '?' => Ok(None),
            _ => Err("invalid condition char"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SpringRow;

    #[test]
    fn validation_no_missing() {
        let row: SpringRow = ".#.###.#.###### 1,3,1,6".parse().unwrap();
        assert!(row.is_valid().unwrap());
    }

    #[test]
    fn validation_missing() {
        let row: SpringRow = ".#.#?#?#?#?#?#? 1,3,1,6".parse().unwrap();
        assert!(row.is_valid().is_none());
    }

    #[test]
    fn example_prefix_simplification() {
        let row: SpringRow = "..????????????? 1,1,3".parse().unwrap();
        assert_eq!(
            row.simplified().unwrap(),
            "????????????? 1,1,3".parse().unwrap()
        );
    }

    #[test]
    fn example_hard_simplification() {
        let row: SpringRow = "#.?.### 1,1,3".parse().unwrap();
        assert_eq!(row.simplified().unwrap(), "?.### 1,3".parse().unwrap());
    }

    #[test]
    fn example_line1() {
        let row: SpringRow = "???.### 1,1,3".parse().unwrap();
        assert_eq!(row.combinations(), 1);
    }

    #[test]
    fn example_line2() {
        let row: SpringRow = ".??..??...?##. 1,1,3".parse().unwrap();
        assert_eq!(row.combinations(), 4);
    }

    #[test]
    fn example_line3() {
        let row: SpringRow = "?#?#?#?#?#?#?#? 1,3,1,6".parse().unwrap();
        assert_eq!(row.combinations(), 1);
    }

    #[test]
    fn example_line4() {
        let row: SpringRow = "????.#...#... 4,1,1".parse().unwrap();
        assert_eq!(row.combinations(), 1);
    }

    #[test]
    fn example_line5() {
        let row: SpringRow = "????.######..#####. 1,6,5".parse().unwrap();
        assert_eq!(row.combinations(), 4);
    }

    #[test]
    fn example_line6() {
        let row: SpringRow = "?###???????? 3,2,1".parse().unwrap();
        assert_eq!(row.combinations(), 10);
    }
}
