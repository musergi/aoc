use std::str::FromStr;

mod value;
pub use value::Value;

mod op;
pub use op::Op;

mod error;
pub use error::ParseExpressionError;

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Value(Value),
    Op(Op),
}

impl FromStr for Expression {
    type Err = ParseExpressionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse()
            .map(Self::Value)
            .or(s.parse().map(Self::Op))
            .map_err(|_| ParseExpressionError::new(s))
    }
}

#[cfg(test)]
mod tests {
    use super::Expression;
    use crate::expression::{Op, Value};

    #[test]
    fn parse_literal() {
        let expr: Expression = "132".parse().unwrap();
        assert_eq!(expr, Expression::Value(Value::Literal(132)));
    }

    #[test]
    fn parse_not() {
        let expr: Expression = "NOT 321".parse().unwrap();
        assert_eq!(expr, Expression::Op(Op::Not(Value::Literal(321))));
    }

    #[test]
    fn parse_inner_variable() {
        let expr: Expression = "NOT abc".parse().unwrap();
        assert_eq!(
            expr,
            Expression::Op(Op::Not(Value::Variable("abc".to_string())))
        )
    }

    #[test]
    fn parse_lshift() {
        let expr: Expression = "321 LSHIFT 123".parse().unwrap();
        assert_eq!(
            expr,
            Expression::Op(Op::Lshift(Value::Literal(321), Value::Literal(123)))
        )
    }

    #[test]
    fn parse_rshift() {
        let expr: Expression = "321 RSHIFT 123".parse().unwrap();
        assert_eq!(
            expr,
            Expression::Op(Op::Rshift(Value::Literal(321), Value::Literal(123)))
        )
    }

    #[test]
    fn parse_and() {
        let expr: Expression = "321 AND 123".parse().unwrap();
        assert_eq!(
            expr,
            Expression::Op(Op::And(Value::Literal(321), Value::Literal(123)))
        )
    }

    #[test]
    fn parse_or() {
        let expr: Expression = "321 OR 123".parse().unwrap();
        assert_eq!(
            expr,
            Expression::Op(Op::Or(Value::Literal(321), Value::Literal(123)))
        )
    }

    #[test]
    fn parse_err() {
        let err = "321 XOR 123".parse::<Expression>().unwrap_err();
        assert_eq!(err.to_string(), "invalid expression `321 XOR 123`")
    }
}
