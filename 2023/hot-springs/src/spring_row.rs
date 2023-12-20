use std::str::FromStr;

#[derive(Clone)]
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
        if self.conditions.iter().all(|condition| condition.is_some()) {
            if self.is_valid() {
                1
            } else {
                0
            }
        } else {
            self.sum_both_options()
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

    fn is_valid(&self) -> bool {
        let mut idx = 0;
        let mut current_count = None;
        for condition in self.conditions.iter() {
            current_count = match (condition.as_ref().unwrap(), current_count) {
                (SpringCondition::Operational, None) => None,
                (SpringCondition::Damaged, None) => Some(1),
                (SpringCondition::Damaged, Some(n)) => Some(n + 1),
                (SpringCondition::Operational, Some(n)) => {
                    if let Some(expected) = self.validation.get(idx) {
                        if *expected != n {
                            return false;
                        }
                        idx += 1;
                    } else {
                        return false;
                    }
                    None
                }
            }
        }
        if let Some(current_count) = current_count {
            if let Some(expected) = self.validation.get(idx) {
                if *expected != current_count {
                    return false;
                }
                idx += 1;
            } else {
                return false;
            }
        }
        idx == self.validation.len()
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
