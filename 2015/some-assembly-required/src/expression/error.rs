use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub struct ParseExpressionError {
    expression: String,
}

impl ParseExpressionError {
    pub fn new(s: impl ToString) -> ParseExpressionError {
        ParseExpressionError {
            expression: s.to_string(),
        }
    }
}

impl Display for ParseExpressionError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "invalid expression `{}`", self.expression)
    }
}

impl Error for ParseExpressionError {}
