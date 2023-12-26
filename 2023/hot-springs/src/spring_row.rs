use crate::{counter_iter::IteratorExt, spring_condition::SpringCondition};
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpringRow {
    conditions: Vec<Option<SpringCondition>>,
    validation: Vec<usize>,
}

impl SpringRow {
    pub fn combinations(&self) -> usize {
        let mut copy = self.clone();
        let mut cache = Cache::default();
        let mut row_ref = SpringRowRef {
            reference: &mut copy,
            conditions_offset: 0,
            validation_offset: 0,
        };
        row_ref.inner_combinations(&mut cache)
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

struct SpringRowRef<'a> {
    reference: &'a mut SpringRow,
    conditions_offset: usize,
    validation_offset: usize,
}

impl<'a> SpringRowRef<'a> {
    fn conditions(&self) -> &[Option<SpringCondition>] {
        &self.reference.conditions[self.conditions_offset..]
    }

    fn validations(&self) -> &[usize] {
        &self.reference.validation[self.validation_offset..]
    }

    fn conditions_mut(&mut self) -> &mut [Option<SpringCondition>] {
        &mut self.reference.conditions[self.conditions_offset..]
    }

    fn counts(&self) -> impl Iterator<Item = (usize, &Option<SpringCondition>)> {
        self.conditions().iter().counts()
    }

    fn inner_combinations(&mut self, cache: &mut Cache) -> usize {
        let initial_condition_offset = self.conditions_offset;
        let initial_validation_offset = self.validation_offset;
        self.simplified();
        let combinations = cache.cache_or_else_calc(self, |value, cache| match value.is_valid() {
            Some(true) => 1,
            Some(false) => 0,
            None => value.sum_both_options(cache),
        });
        self.conditions_offset = initial_condition_offset;
        self.validation_offset = initial_validation_offset;
        combinations
    }

    fn simplified(&mut self) {
        while let Some((strip_prefix, validation_strips)) = self.strip_prefix() {
            self.conditions_offset += strip_prefix;
            self.validation_offset += validation_strips;
        }
    }

    fn strip_prefix(&self) -> Option<(usize, usize)> {
        let mut it = self.counts();
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
                    && first_length == *self.validations().get(0)?
                {
                    Some((first_length, 1))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn is_valid(&self) -> Option<bool> {
        let mut idx = 0;
        let mut previous_damaged = None;
        for (count, value) in self.conditions().iter().counts() {
            match value {
                Some(SpringCondition::Damaged) => previous_damaged = Some(count),
                Some(SpringCondition::Operational) => {
                    if let Some(damaged) = previous_damaged {
                        if let Some(expected) = self.validations().get(idx) {
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
            if let Some(expected) = self.validations().get(idx) {
                if *expected != damaged {
                    return Some(false);
                }
                idx += 1;
            } else {
                return Some(false);
            }
        }
        Some(idx == self.validations().len())
    }

    fn sum_both_options(&mut self, cache: &mut Cache) -> usize {
        let empty_idx = self
            .conditions()
            .iter()
            .position(|condition| condition.is_none())
            .unwrap();
        self.conditions_mut()[empty_idx] = Some(SpringCondition::Operational);
        let first = self.inner_combinations(cache);
        self.conditions_mut()[empty_idx] = Some(SpringCondition::Damaged);
        let second = self.inner_combinations(cache);
        self.conditions_mut()[empty_idx] = None;
        first + second
    }

    fn to_row(&self) -> SpringRow {
        let conditions = self.conditions().iter().cloned().collect();
        let validation = self.validations().iter().cloned().collect();
        SpringRow {
            conditions,
            validation,
        }
    }
}

#[derive(Default)]
struct Cache {
    table: HashMap<Vec<Option<SpringCondition>>, HashMap<Vec<usize>, usize>>,
}

impl Cache {
    fn cache_or_else_calc(
        &mut self,
        curr: &mut SpringRowRef,
        calc: fn(&mut SpringRowRef, &mut Self) -> usize,
    ) -> usize {
        self.table
            .get(curr.conditions())
            .and_then(|condition| condition.get(curr.validations()))
            .cloned()
            .unwrap_or_else(|| {
                let value = calc(curr, self);
                let SpringRow {
                    conditions,
                    validation,
                } = curr.to_row();
                self.table
                    .entry(conditions)
                    .or_default()
                    .insert(validation, value);
                value
            })
    }
}

#[cfg(test)]
mod tests {
    use super::SpringRow;

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
