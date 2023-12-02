use crate::{
    expression::{Expression, Value, Op},
    statement::Statement,
};
use std::collections::{btree_map::Values, HashMap};

pub struct Solver {
    statements: Vec<Statement>,
}

impl Solver {
    pub fn new(statements: Vec<Statement>) -> Solver {
        Solver { statements }
    }

    pub fn solve(&self, identifier: &str) -> u16 {
        let mut state = SolveState::new(identifier);
        while let Some(current_identifier) = state.stack.pop() {
            match &self
                .statements
                .iter()
                .find(|stmt| stmt.identifier == current_identifier)
                .unwrap()
                .expression
            {
                Expression::Value(value) => state.try_update_value(identifier, value),
                Expression::Op(op) => state.try_update_op(identifier, op),
            }
        }
        *state.values.get(identifier).expect("unresolved ref")
    }
}

struct SolveState<'a> {
    values: HashMap<&'a str, u16>,
    stack: Vec<&'a str>,
}

impl<'a> SolveState<'a> {
    fn new(target: &'a str) -> SolveState<'a> {
        SolveState {
            values: HashMap::new(),
            stack: vec![target],
        }
    }

    fn try_update_value(&mut self, identifier: &'a str, value: &'a Value) {
        match value {
            Value::Literal(value) => {
                self.values.insert(identifier, *value);
            }
            Value::Variable(refered) => {
                if let Some(value) = self.values.get(identifier) {
                    self.values.insert(identifier, *value);
                } else {
                    self.stack.push(identifier);
                    self.stack.push(&refered);
                }
            }
        }
    }

    fn try_update_op(&self, identifier: &str, op: &Op) {
        match op {
            Op::And(l, r) => {
                todo!()
            }
            _ => todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solver;
    use crate::{
        expression::{Expression, Op, Value},
        statement::Statement,
    };

    #[test]
    fn solve_base_case() {
        let statements = vec![Statement {
            identifier: "a".to_string(),
            expression: Expression::Value(Value::Literal(123)),
        }];
        let solver = Solver::new(statements);
        assert_eq!(solver.solve("a"), 123);
    }

    #[test]
    fn solve_single_derreference() {
        let statements = vec![
            Statement {
                identifier: "a".to_string(),
                expression: Expression::Value(Value::Variable("b".to_string())),
            },
            Statement {
                identifier: "b".to_string(),
                expression: Expression::Value(Value::Literal(123)),
            },
        ];
        let solver = Solver::new(statements);
        assert_eq!(solver.solve("a"), 123);
    }

    #[test]
    fn solve_double_derreference() {
        let statements = vec![
            Statement {
                identifier: "a".to_string(),
                expression: Expression::Value(Value::Variable("b".to_string())),
            },
            Statement {
                identifier: "b".to_string(),
                expression: Expression::Value(Value::Variable("c".to_string())),
            },
            Statement {
                identifier: "c".to_string(),
                expression: Expression::Value(Value::Literal(321)),
            },
        ];
        let solver = Solver::new(statements);
        assert_eq!(solver.solve("a"), 321);
    }

    #[test]
    fn solve_and() {
        let statements = vec![
            Statement {
                identifier: "a".to_string(),
                expression: Expression::Op(Op::And(
                    Value::Variable("b".to_string()),
                    Value::Literal(123),
                )),
            },
            Statement {
                identifier: "b".to_string(),
                expression: Expression::Value(Value::Literal(321)),
            },
        ];
        let solver = Solver::new(statements);
        assert_eq!(solver.solve("a"), 321 & 123);
    }
}
