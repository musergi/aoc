use crate::expression::{Expression, ParseExpressionError};
use std::{error::Error, fmt::Display, str::FromStr};

#[derive(Debug)]
pub struct Statement {
    pub identifier: String,
    pub expression: Expression,
}

#[derive(Debug)]
pub enum ParseStatementError {
    MissingAssignment(String),
    InvalidExpression(String, ParseExpressionError),
}

impl Display for ParseStatementError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseStatementError::MissingAssignment(statement) => {
                write!(f, "assignment not found in statement `{statement}`")
            }
            ParseStatementError::InvalidExpression(statement, err) => {
                write!(f, "while parsing `{statement}` encountered: {err}")
            }
        }
    }
}

impl Error for ParseStatementError {}

impl FromStr for Statement {
    type Err = ParseStatementError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (expression, identifier) = s
            .split_once(" -> ")
            .ok_or_else(|| ParseStatementError::MissingAssignment(s.to_string()))?;
        let expression = expression
            .parse()
            .map_err(|err| ParseStatementError::InvalidExpression(s.to_string(), err))?;
        let identifier = identifier.to_string();
        Ok(Statement {
            identifier,
            expression,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Statement;
    use crate::expression::{Expression, Value};

    #[test]
    fn parse() {
        let stmt: Statement = "456 -> y".parse().unwrap();
        assert_eq!(stmt.identifier, "y");
        assert_eq!(stmt.expression, Expression::Value(Value::Literal(456)));
    }

    #[test]
    fn parse_err() {
        let _err = "456 <- y".parse::<Statement>().unwrap_err();
    }
}
