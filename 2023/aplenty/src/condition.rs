use crate::{part::Part, part_combination::PartCombination};

pub struct Condition {
    operator: Operator,
    variable: Variable,
    constant: u64,
}
impl Condition {
    pub fn apply(&self, part: &Part) -> bool {
        match self.operator {
            Operator::Less => self.eval(part) < self.constant,
            Operator::Greater => self.eval(part) > self.constant,
        }
    }

    fn eval(&self, part: &Part) -> u64 {
        match self.variable {
            Variable::X => part.x,
            Variable::M => part.m,
            Variable::A => part.a,
            Variable::S => part.s,
        }
    }

    pub fn split(
        &self,
        combination: PartCombination,
    ) -> (Option<PartCombination>, Option<PartCombination>) {
        match self.operator {
            Operator::Less => {
                let true_combination = self.split_with_tranform(combination.clone(), |range| {
                    range.1 = self.constant - 1;
                });
                let false_combination = self.split_with_tranform(combination, |range| {
                    range.0 = self.constant;
                });
                (true_combination, false_combination)
            }
            Operator::Greater => {
                let true_combination = self.split_with_tranform(combination.clone(), |range| {
                    range.0 = self.constant + 1;
                });
                let false_combination = self.split_with_tranform(combination, |range| {
                    range.1 = self.constant;
                });
                (true_combination, false_combination)
            }
        }
    }

    fn split_with_tranform<F>(
        &self,
        mut combination: PartCombination,
        mutator: F,
    ) -> Option<PartCombination>
    where
        F: FnOnce(&mut (u64, u64)) -> (),
    {
        let mut range = self.range(&mut combination);
        mutator(&mut range);
        if range.0 <= range.1 {
            Some(combination)
        } else {
            None
        }
    }

    fn range<'a>(&self, combination: &'a mut PartCombination) -> &'a mut (u64, u64) {
        match self.variable {
            Variable::X => &mut combination.x,
            Variable::M => &mut combination.m,
            Variable::A => &mut combination.a,
            Variable::S => &mut combination.s,
        }
    }
}

impl std::str::FromStr for Condition {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let condition = s
            .split_once('<')
            .map(|(left, right)| parse_with_op(Operator::Less, left, right))
            .or_else(|| {
                s.split_once('>')
                    .map(|(left, right)| parse_with_op(Operator::Greater, left, right))
            })
            .ok_or("invalid operator")??;
        Ok(condition)
    }
}

fn parse_with_op(operator: Operator, left: &str, right: &str) -> Result<Condition, &'static str> {
    let variable = match left {
        "x" => Ok(Variable::X),
        "m" => Ok(Variable::M),
        "a" => Ok(Variable::A),
        "s" => Ok(Variable::S),
        _ => Err("invalid variable in condition"),
    }?;
    let constant = right.parse().map_err(|_| "invalid constant in condition")?;
    Ok(Condition {
        operator,
        variable,
        constant,
    })
}

enum Variable {
    X,
    M,
    A,
    S,
}

enum Operator {
    Less,
    Greater,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        let s = "a<2006";
        let condition: Condition = s.parse().unwrap();
        assert!(matches!(condition.operator, Operator::Less));
        assert!(matches!(condition.variable, Variable::A));
        assert_eq!(condition.constant, 2006);
    }

    #[test]
    fn fail_on_unkown_op() {
        let s = "a=1231";
        assert!(s.parse::<Condition>().is_err())
    }

    #[test]
    fn fail_on_nan() {
        let s = "a<bbb";
        assert!(s.parse::<Condition>().is_err())
    }

    #[test]
    fn fail_on_invalid_left() {
        let s = "b<2131";
        assert!(s.parse::<Condition>().is_err())
    }
}
