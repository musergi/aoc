use super::Value;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    Not(Value),
    Lshift(Value, Value),
    Rshift(Value, Value),
    And(Value, Value),
    Or(Value, Value),
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_not(s)
            .or_else(|| parse_binary(s, " LSHIFT ", Op::Lshift))
            .or_else(|| parse_binary(s, " RSHIFT ", Op::Rshift))
            .or_else(|| parse_binary(s, " AND ", Op::And))
            .or_else(|| parse_binary(s, " OR ", Op::Or))
            .ok_or(())
    }
}

fn parse_not(s: &str) -> Option<Op> {
    s.strip_prefix("NOT ")
        .and_then(|s| s.parse().ok())
        .map(Op::Not)
}

fn parse_binary(s: &str, split: &str, constructor: fn(Value, Value) -> Op) -> Option<Op> {
    let (left, right) = s.split_once(split)?;
    let left = left.parse().ok()?;
    let right = right.parse().ok()?;
    Some(constructor(left, right))
}
