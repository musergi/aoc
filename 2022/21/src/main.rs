use std::{collections::HashMap, convert::Infallible, fs, str::FromStr};

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
    let monkeys: HashMap<_, _> = s
        .lines()
        .map(|line| line.parse().unwrap())
        .map(|Monkey { name, value }: Monkey| (name, value))
        .collect();
    let part1 = resolve(&monkeys, monkeys.get("root").unwrap());
    println!("Part 1: {}", part1);
    if let MonkeyValue::Operation {
        left,
        right,
        ..
    } = monkeys.get("root").unwrap() {
        let left_v = try_resolve(&monkeys, &left, monkeys.get(left).unwrap());
        let right_v = try_resolve(&monkeys, &right, monkeys.get(right).unwrap());
        let part2 = match (left_v, right_v) {
            (Some(v), None) => backprop(&monkeys, v, right, monkeys.get(right).unwrap()),
            (None, Some(v)) => backprop(&monkeys, v, left, monkeys.get(left).unwrap()),
            (_, _) => panic!("invalid input"),
        }.unwrap();
        println!("Part 2: {}", part2);
    }
}

fn resolve(expressions: &HashMap<String, MonkeyValue>, value: &MonkeyValue) -> i64 {
    match value {
        MonkeyValue::Literal(v) => *v,
        MonkeyValue::Operation {
            operator,
            left,
            right,
        } => match operator {
            Operator::Add => {
                resolve(expressions, expressions.get(left).unwrap())
                    + resolve(expressions, expressions.get(right).unwrap())
            }
            Operator::Sub => {
                resolve(expressions, expressions.get(left).unwrap())
                    - resolve(expressions, expressions.get(right).unwrap())
            }
            Operator::Mul => {
                resolve(expressions, expressions.get(left).unwrap())
                    * resolve(expressions, expressions.get(right).unwrap())
            }
            Operator::Div => {
                resolve(expressions, expressions.get(left).unwrap())
                    / resolve(expressions, expressions.get(right).unwrap())
            }
        },
    }
}

fn try_resolve(
    expressions: &HashMap<String, MonkeyValue>,
    key: &str,
    value: &MonkeyValue,
) -> Option<i64> {
    if key == "humn" {
        None
    } else {
        Some(match value {
            MonkeyValue::Literal(v) => *v,
            MonkeyValue::Operation {
                operator,
                left,
                right,
            } => match operator {
                Operator::Add => {
                    try_resolve(expressions, &left, expressions.get(left).unwrap())?
                        + try_resolve(expressions, &right, expressions.get(right).unwrap())?
                }
                Operator::Sub => {
                    try_resolve(expressions, &left, expressions.get(left).unwrap())?
                        - try_resolve(expressions, &right, expressions.get(right).unwrap())?
                }
                Operator::Mul => {
                    try_resolve(expressions, &left, expressions.get(left).unwrap())?
                        * try_resolve(expressions, &right, expressions.get(right).unwrap())?
                }
                Operator::Div => {
                    try_resolve(expressions, &left, expressions.get(left).unwrap())?
                        / try_resolve(expressions, &right, expressions.get(right).unwrap())?
                }
            },
        })
    }
}

fn backprop(
    expressions: &HashMap<String, MonkeyValue>,
    expected_value: i64,
    name: &str,
    value: &MonkeyValue,
) -> Option<i64> {
    if name == "humn" {
        Some(expected_value)
    } else {
        match value {
            MonkeyValue::Literal(v) => Some(*v),
            MonkeyValue::Operation {
                operator,
                left,
                right,
            } => {
                let left_value = try_resolve(expressions, left, expressions.get(left).unwrap());
                let right_value = try_resolve(expressions, right, expressions.get(right).unwrap());
                match (operator, left_value, right_value) {
                    (Operator::Add, Some(v), None) => backprop(
                        expressions,
                        expected_value - v,
                        right,
                        expressions.get(right).unwrap(),
                    ),
                    (Operator::Add, None, Some(v)) => backprop(
                        expressions,
                        expected_value - v,
                        left,
                        expressions.get(left).unwrap(),
                    ),
                    (Operator::Sub, Some(v), None) => backprop(
                        expressions,
                        v - expected_value,
                        right,
                        expressions.get(right).unwrap(),
                    ),
                    (Operator::Sub, None, Some(v)) => backprop(
                        expressions,
                        expected_value + v,
                        left,
                        expressions.get(left).unwrap(),
                    ),
                    (Operator::Mul, Some(v), None) => backprop(
                        expressions,
                        expected_value / v,
                        right,
                        expressions.get(right).unwrap(),
                    ),
                    (Operator::Mul, None, Some(v)) => backprop(
                        expressions,
                        expected_value / v,
                        left,
                        expressions.get(left).unwrap(),
                    ),
                    (Operator::Div, Some(v), None) => backprop(
                        expressions,
                        v / expected_value,
                        right,
                        expressions.get(right).unwrap(),
                    ),
                    (Operator::Div, None, Some(v)) => backprop(
                        expressions,
                        expected_value * v,
                        left,
                        expressions.get(left).unwrap(),
                    ),
                    (_, _, _) => panic!("unsolvable"),
                }
            }
        }
    }
}
