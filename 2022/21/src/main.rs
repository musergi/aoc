use std::{collections::HashMap, convert::Infallible, fs, str::FromStr};

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
