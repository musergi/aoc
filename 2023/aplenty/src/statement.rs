use crate::{condition::Condition, part::Part, part_combination::PartCombination, value::Value};

pub struct Statement {
    condition: Option<Condition>,
    value: Value,
}
impl Statement {
    pub fn value(&self) -> Value {
        self.value.clone()
    }

    pub fn applicable(&self, part: &Part) -> bool {
        self.condition
            .as_ref()
            .map(|condition| condition.apply(part))
            .unwrap_or(true)
    }

    pub fn split(
        &self,
        combination: PartCombination,
    ) -> (Option<(PartCombination, Value)>, Option<PartCombination>) {
        match &self.condition {
            Some(condition) => {
                let (true_combination, false_combination) = condition.split(combination);
                (
                    true_combination.map(|combination| (combination, self.value())),
                    false_combination,
                )
            }
            None => (Some((combination, self.value())), None),
        }
    }
}

impl std::str::FromStr for Statement {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let statement = match s.split_once(":") {
            Some((condition, value)) => Statement {
                condition: Some(condition.parse()?),
                value: value.parse()?,
            },
            None => Statement {
                condition: None,
                value: s.parse()?,
            },
        };
        Ok(statement)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_with_condition() {
        let s = "a>1716:R";
        let statement: Statement = s.parse().unwrap();
        assert!(statement.condition.is_some());
    }

    #[test]
    fn parse_without_condition() {
        let s = "R";
        let statement: Statement = s.parse().unwrap();
        assert!(statement.condition.is_none());
    }
}
