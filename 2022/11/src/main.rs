use std::fs;
use std::iter;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct BadInput(String);

impl From<ParseIntError> for BadInput {
    fn from(err: ParseIntError) -> BadInput {
        BadInput(err.to_string())
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    action: Expr,
    check: Check,
    true_target: usize,
    false_target: usize,
}

impl Monkey {
    fn get_target(&self, val: &u64) -> &usize {
        if self.check.eval(val) {
            &self.true_target
        } else {
            &self.false_target
        }
    }
}

impl FromStr for Monkey {
    type Err = BadInput;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sp = s.lines();
        sp.next().ok_or(BadInput(s.to_string()))?;
        let items = sp
            .next()
            .ok_or(BadInput(s.to_string()))?
            .trim()
            .strip_prefix("Starting items:")
            .ok_or(BadInput(s.to_string()))?
            .split(",")
            .map(|s| s.trim())
            .map(|s| s.parse::<u64>().map_err(|_| BadInput(s.to_string())))
            .collect::<Result<_, _>>()?;
        let action = sp
            .next()
            .ok_or(BadInput(s.to_string()))?
            .trim()
            .strip_prefix("Operation: new =")
            .ok_or(BadInput(s.to_string()))?
            .trim()
            .parse::<Expr>()?;
        let check = sp
            .next()
            .ok_or(BadInput(s.to_string()))?
            .trim()
            .strip_prefix("Test: divisible by")
            .ok_or(BadInput(s.to_string()))?
            .trim()
            .parse::<Check>()?;
        let true_target = sp
            .next()
            .ok_or(BadInput(s.to_string()))?
            .trim()
            .strip_prefix("If true: throw to monkey")
            .ok_or(BadInput(s.to_string()))?
            .trim()
            .parse::<usize>()?;
        let false_target = sp
            .next()
            .ok_or(BadInput(s.to_string()))?
            .trim()
            .strip_prefix("If false: throw to monkey")
            .ok_or(BadInput(s.to_string()))?
            .trim()
            .parse::<usize>()?;
        Ok(Monkey {
            items,
            action,
            check,
            true_target,
            false_target,
        })
    }
}

#[derive(Debug, Clone)]
enum Expr {
    Var,
    Literal(u64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

impl Expr {
    fn eval(&self, v: u64) -> u64 {
        match self {
            Expr::Var => v,
            Expr::Literal(l) => l.clone(),
            Expr::Add(l, r) => l.eval(v) + r.eval(v),
            Expr::Mul(l, r) => l.eval(v) * r.eval(v),
        }
    }
}

impl FromStr for Expr {
    type Err = BadInput;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("+") {
            let mut sp = s.split("+");
            let l = Box::new(
                sp.next()
                    .ok_or(BadInput(s.to_string()))?
                    .trim()
                    .parse::<Expr>()?,
            );
            let r = Box::new(
                sp.next()
                    .ok_or(BadInput(s.to_string()))?
                    .trim()
                    .parse::<Expr>()?,
            );
            Ok(Expr::Add(l, r))
        } else if s.contains("*") {
            let mut sp = s.split("*");
            let l = Box::new(
                sp.next()
                    .ok_or(BadInput(s.to_string()))?
                    .trim()
                    .parse::<Expr>()?,
            );
            let r = Box::new(
                sp.next()
                    .ok_or(BadInput(s.to_string()))?
                    .trim()
                    .parse::<Expr>()?,
            );
            Ok(Expr::Mul(l, r))
        } else if s == "old" {
            Ok(Expr::Var)
        } else {
            let n = s.parse::<u64>()?;
            Ok(Expr::Literal(n))
        }
    }
}

#[derive(Debug, Clone)]
struct Check(u64);

impl Check {
    fn eval(&self, val: &u64) -> bool {
        val % self.0 == 0
    }
}

impl FromStr for Check {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u64>().map(|v| Check(v))
    }
}

fn worry_level<F>(mut monkeys: Vec<Monkey>, rounds: usize, mut relax_fn: F) -> u64
where
    F: FnMut(u64) -> u64,
{
    let mut throws = iter::repeat(0).take(monkeys.len()).collect::<Vec<_>>();
    let modulus = monkeys.iter().map(|m| m.check.0).product::<u64>();
    for _ in 0..rounds {
        for idx in 0..monkeys.len() {
            let items = monkeys
                .get_mut(idx)
                .unwrap()
                .items
                .drain(..)
                .collect::<Vec<_>>();
            for item in items {
                *throws.get_mut(idx).unwrap() += 1;
                let new = relax_fn(monkeys.get(idx).unwrap().action.eval(item)) % modulus;
                let target = monkeys.get(idx).unwrap().get_target(&new).clone();
                monkeys.get_mut(target).expect("Target").items.push(new);
            }
        }
    }
    throws.sort();
    throws
        .iter()
        .rev()
        .take(2)
        .cloned()
        .product()
}

fn main() {
    let monkeys = fs::read_to_string("assets/input.txt")
        .expect("File")
        .split("\n\n")
        .map(|ls| ls.parse::<Monkey>())
        .collect::<Result<Vec<_>, _>>()
        .expect("Parsed Monkeys");
    println!(
        "Level of buisness: {}",
        worry_level(monkeys.clone(), 20, |x| x / 3)
    );
    println!(
        "New level of buisness: {}",
        worry_level(monkeys, 10000, |x| x)
    );
}
