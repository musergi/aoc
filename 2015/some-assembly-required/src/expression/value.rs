use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Value {
    Literal(u16),
    Variable(String),
}

impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_literal(s).or_else(|| parse_variable(s)).ok_or(())
    }
}

fn parse_literal(s: &str) -> Option<Value> {
    s.parse().ok().map(Value::Literal)
}

fn parse_variable(s: &str) -> Option<Value> {
    if s.chars().all(|c| c.is_ascii_alphabetic()) {
        Some(Value::Variable(s.to_string()))
    } else {
        None
    }
}
