use std::{collections::HashMap, convert::Infallible, fs, str::FromStr};

struct Problem {
    monkeys: HashMap<String, Monkey>,
}

impl FromStr for Problem {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let monkeys = s
            .lines()
            .map(|line| line.parse().unwrap())
            .map(|monkey: Monkey| (monkey.name.clone(), monkey))
            .collect();
        Ok(Self { monkeys })
    }
}

impl Problem {
    fn value(&self, name: &str) -> &MonkeyValue {
        &self.monkeys.get(name).unwrap().value
    }

    fn evaluate(&self, name: &str) -> i64 {
        match self.value(name) {
            MonkeyValue::Literal(value) => *value,
            MonkeyValue::Operation {
                operator,
                left,
                right,
            } => {
                let left = self.evaluate(left);
                let right = self.evaluate(right);
                operator.apply(left, right)
            }
        }
    }

    fn equality(&self, equality: &str, unkown: &str) -> i64 {
        match self.value(equality) {
            MonkeyValue::Literal(_) => panic!(),
            MonkeyValue::Operation { left, right, .. } => {
                match (
                    self.has_unkown(left, unkown),
                    self.has_unkown(right, unkown),
                ) {
                    (true, false) => self.solve(self.evaluate(right), left, unkown),
                    (false, true) => self.solve(self.evaluate(left), right, unkown),
                    (_, _) => panic!(),
                }
            }
        }
    }

    fn has_unkown(&self, name: &str, unkown: &str) -> bool {
        match (name == unkown, self.value(name)) {
            (true, _) => true,
            (false, MonkeyValue::Literal(_)) => false,
            (false, MonkeyValue::Operation { left, right, .. }) => {
                self.has_unkown(left, unkown) || self.has_unkown(right, unkown)
            }
        }
    }

    fn solve(&self, result: i64, name: &str, unkown: &str) -> i64 {
        if name == unkown {
            result
        } else {
            if let MonkeyValue::Operation {
                operator,
                left,
                right,
            } = self.value(name)
            {
                match (
                    self.has_unkown(left, unkown),
                    self.has_unkown(right, unkown),
                ) {
                    (true, false) => self.solve(
                        operator.missing_left(result, self.evaluate(right)),
                        left,
                        unkown,
                    ),
                    (false, true) => self.solve(
                        operator.missing_right(result, self.evaluate(left)),
                        right,
                        unkown,
                    ),
                    (_, _) => panic!(),
                }
            } else {
                panic!()
            }
        }
    }
}

#[derive(Debug)]
struct Monkey {
    name: String,
    value: MonkeyValue,
}

impl FromStr for Monkey {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(": ");
        let name = it.next().unwrap().to_string();
        let value = it.next().unwrap().parse().unwrap();
        Ok(Self { name, value })
    }
}

#[derive(Debug)]
enum MonkeyValue {
    Literal(i64),
    Operation {
        operator: Operator,
        left: String,
        right: String,
    },
}

impl FromStr for MonkeyValue {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(literal) = s.parse() {
            Ok(MonkeyValue::Literal(literal))
        } else {
            let mut it = s.split_whitespace();
            let left = it.next().unwrap().to_string();
            let operator = it.next().unwrap().parse().unwrap();
            let right = it.next().unwrap().to_string();
            Ok(Self::Operation {
                operator,
                left,
                right,
            })
        }
    }
}

#[derive(Debug)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}
impl Operator {
    fn apply(&self, left: i64, right: i64) -> i64 {
        match self {
            Operator::Add => left + right,
            Operator::Sub => left - right,
            Operator::Mul => left * right,
            Operator::Div => left / right,
        }
    }

    fn missing_left(&self, result: i64, right: i64) -> i64 {
        match self {
            Operator::Add => result - right,
            Operator::Sub => result + right,
            Operator::Mul => result / right,
            Operator::Div => result * right,
        }
    }

    fn missing_right(&self, result: i64, left: i64) -> i64 {
        match self {
            Operator::Add => result - left,
            Operator::Sub => left - result,
            Operator::Mul => result / left,
            Operator::Div => left / result,
        }
    }
}

impl FromStr for Operator {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => Operator::Add,
            "-" => Operator::Sub,
            "*" => Operator::Mul,
            "/" => Operator::Div,
            _ => panic!("Unknown symbol"),
        })
    }
}

fn main() {
    let s = fs::read_to_string("assets/input.txt").unwrap();
    let problem: Problem = s.parse().unwrap();
    let part1 = problem.evaluate("root");
    println!("Part 1: {}", part1);
    let part2 = problem.equality("root", "humn");
    println!("Part 2: {}", part2);
}
