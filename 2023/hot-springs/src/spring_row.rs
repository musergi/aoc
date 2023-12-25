use crate::counter_iter::IteratorExt;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpringRow {
    conditions: Vec<Option<SpringCondition>>,
    validation: Vec<usize>,
}

impl SpringRow {
    pub fn combinations(&self) -> usize {
        let mut copy = self.clone();
        let mut cache = Cache::default();
        copy.inner_combinations(&mut cache)
    }

    pub fn expand(self) -> Self {
        let Self {
            conditions,
            validation,
        } = self;
        let conditions = (0..(conditions.len() * 5 + 4))
            .map(|idx| {
                let mod_idx = idx % (conditions.len() + 1);
                if mod_idx == conditions.len() {
                    None
                } else {
                    conditions[mod_idx].clone()
                }
            })
            .collect();
        let validation = std::iter::repeat(validation.iter())
            .take(5)
            .flatten()
            .cloned()
            .collect();
        Self {
            conditions,
            validation,
        }
    }

    fn inner_combinations(&mut self, cache: &mut Cache) -> usize {
        let mut simplified = self.simplified();
        let mut anaylzed = simplified.as_mut().unwrap_or(self);
        cache.cache_or_else_calc(&mut anaylzed, |value, cache| match value.is_valid() {
            Some(true) => 1,
            Some(false) => 0,
            None => value.sum_both_options(cache),
        })
    }

    fn sum_both_options(&mut self, cache: &mut Cache) -> usize {
        let empty_idx = self
            .conditions
            .iter()
            .position(|condition| condition.is_none())
            .unwrap();
        self.conditions[empty_idx] = Some(SpringCondition::Operational);
        let first = self.inner_combinations(cache);
        self.conditions[empty_idx] = Some(SpringCondition::Damaged);
        let second = self.inner_combinations(cache);
        self.conditions[empty_idx] = None;
        first + second
    }

    fn is_valid(&self) -> Option<bool> {
        let mut idx = 0;
        let mut previous_damaged = None;
        for (count, value) in self.conditions.iter().counts() {
            match value {
                Some(SpringCondition::Damaged) => previous_damaged = Some(count),
                Some(SpringCondition::Operational) => {
                    if let Some(damaged) = previous_damaged {
                        if let Some(expected) = self.validation.get(idx) {
                            if *expected != damaged {
                                return Some(false);
                            }
                            idx += 1;
                        } else {
                            return Some(false);
                        }
                    }
                }
                None => return None,
            }
        }
        if let Some(damaged) = previous_damaged {
            if let Some(expected) = self.validation.get(idx) {
                if *expected != damaged {
                    return Some(false);
                }
                idx += 1;
            } else {
                return Some(false);
            }
        }
        Some(idx == self.validation.len())
    }

    fn simplified(&self) -> Option<Self> {
        let mut new = None;
        while let Some((strip_prefix, validation_strips)) =
            new.as_ref().unwrap_or(self).strip_prefix()
        {
            let source = new.as_ref().unwrap_or(self);
            new = Some(Self {
                conditions: source.conditions[strip_prefix..].iter().cloned().collect(),
                validation: source.validation[validation_strips..]
                    .iter()
                    .cloned()
                    .collect(),
            });
        }
        new
    }

    fn strip_prefix(&self) -> Option<(usize, usize)> {
        let mut it = self.conditions.iter().counts();
        let (first_length, first_condition) = it.next()?;
        match first_condition {
            Some(SpringCondition::Operational) => Some((first_length, 0)),
            Some(SpringCondition::Damaged) => {
                if it
                    .next()
                    .map(|(_, second_condition)| {
                        second_condition
                            .as_ref()
                            .map(|condition| *condition == SpringCondition::Operational)
                            .unwrap_or(false)
                    })
                    .unwrap_or(true)
                    && first_length == *self.validation.get(0)?
                {
                    Some((first_length, 1))
                } else {
                    None
                }
            }
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Default)]
struct Cache {
    table: HashMap<SpringRow, usize>,
    hit: usize,
    miss: usize,
}

impl Cache {
    fn cache_or_else_calc(
        &mut self,
        curr: &mut SpringRow,
        calc: fn(&mut SpringRow, &mut Self) -> usize,
    ) -> usize {
        match self.table.get(curr) {
            Some(value) => {
                self.hit += 1;
                *value
            }
            None => {
                self.miss += 1;
                let value = calc(curr, self);
                self.table.insert(curr.clone(), value);
                value
            }
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
    fn example_empty_prefix_simplification() {
        let row: SpringRow = "..????????????? 1,1,3".parse().unwrap();
        assert_eq!(
            row.simplified().unwrap(),
            "????????????? 1,1,3".parse().unwrap()
        );
    }

    #[test]
    fn example_closed_group_prefix_simplification() {
        let row: SpringRow = "#.????????????? 1,1,3".parse().unwrap();
        assert_eq!(
            row.simplified().unwrap(),
            "????????????? 1,3".parse().unwrap()
        );
    }

    #[test]
    fn example_hard_simplification() {
        let row: SpringRow = "#.?.### 1,1,3".parse().unwrap();
        assert_eq!(row.simplified().unwrap(), "?.### 1,3".parse().unwrap());
    }

    #[test]
    fn test_expansion() {
        let row: SpringRow = ".#? 1".parse().unwrap();
        assert_eq!(
            row.expand(),
            ".#??.#??.#??.#??.#? 1,1,1,1,1".parse().unwrap()
        );
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

    #[test]
    fn test_expanded_line() {
        let row: SpringRow = "?###???????? 3,2,1".parse().unwrap();
        assert_eq!(row.expand().combinations(), 506250);
    }
}
