use crate::{part::Part, part_combination::PartCombination, statement::Statement, value::Value};

pub struct Workflow {
    pub name: String,
    statements: Vec<Statement>,
}
impl Workflow {
    pub fn eval(&self, part: &Part) -> Value {
        self.statements
            .iter()
            .find(|statement| statement.applicable(part))
            .unwrap()
            .value()
    }

    pub fn split(&self, mut combination: PartCombination) -> Vec<(PartCombination, Value)> {
        let mut splits = Vec::new();
        for statement in self.statements.iter() {
            let (split, remainder) = statement.split(combination);
            if let Some(split) = split {
                splits.push(split);
            }
            if let Some(remainder) = remainder {
                combination = remainder;
            } else {
                break;
            }
        }
        splits
    }
}

impl std::str::FromStr for Workflow {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_suffix('}').ok_or("missing closing bracket")?;
        let (name, definition) = s.split_once('{').ok_or("missing opening bracket")?;
        let name = name.to_string();
        let mut statements = Vec::new();
        for statement_definition in definition.split(",") {
            statements.push(statement_definition.parse()?);
        }
        Ok(Workflow { name, statements })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        let s = "px{a<2006:qkq,m>2090:A,rfg}";
        let workflow: Workflow = s.parse().unwrap();
        assert_eq!(workflow.name, "px");
        assert_eq!(workflow.statements.len(), 3);
    }

    #[test]
    fn missing_postfix_error() {
        let s = "px{a<2006:qkq,m>2090:A,rfg";
        assert!(s.parse::<Workflow>().is_err());
    }

    #[test]
    fn missing_separator_error() {
        let s = "pxa<2006:qkq,m>2090:A,rfg}";
        assert!(s.parse::<Workflow>().is_err());
    }
}
