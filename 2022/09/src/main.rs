use std::{
    collections::HashSet,
    fs, iter,
    ops::{Add, AddAssign, Sub},
    str::FromStr,
};

#[derive(Debug, Clone, Eq, PartialEq, Default, Hash)]
struct Coordinates {
    x: i32,
    y: i32,
}

impl Coordinates {
    fn touching(&self, other: &Coordinates) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }

    fn normalized(mut self) -> Self {
        if self.x.abs() > 1 {
            self.x = self.x / self.x.abs();
        }
        if self.y.abs() > 1 {
            self.y = self.y / self.y.abs();
        }
        self
    }

    fn follow(&mut self, target: &Coordinates) {
        if !self.touching(target) {
            *self += (target.clone() - self.clone()).normalized();
        }
    }
}

impl Add for Coordinates {
    type Output = Coordinates;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        Coordinates { x, y }
    }
}

impl AddAssign for Coordinates {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Coordinates {
    type Output = Coordinates;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        Coordinates { x, y }
    }
}

#[derive(Debug)]
struct Bridge {
    segments: Vec<Coordinates>,
}

impl Bridge {
    fn new(n: usize) -> Bridge {
        let segments = iter::repeat(Coordinates::default()).take(n).collect();
        Bridge { segments }
    }

    fn updated(mut self, delta: Coordinates) -> Bridge {
        let mut it = self.segments.iter_mut();
        let mut head = it.next().unwrap();
        *head += delta;
        for el in it {
            el.follow(head);
            head = el;
        }
        self
    }

    fn tail(&self) -> &Coordinates {
        self.segments.last().unwrap()
    }
}

#[derive(Clone)]
enum Instruction {
    R,
    U,
    L,
    D,
}

impl From<Instruction> for Coordinates {
    fn from(val: Instruction) -> Self {
        match val {
            Instruction::R => Coordinates { x: 1, y: 0 },
            Instruction::U => Coordinates { x: 0, y: 1 },
            Instruction::L => Coordinates { x: -1, y: 0 },
            Instruction::D => Coordinates { x: 0, y: -1 },
        }
    }
}

impl FromStr for Instruction {
    type Err = InvalidInput;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Instruction::R),
            "U" => Ok(Instruction::U),
            "L" => Ok(Instruction::L),
            "D" => Ok(Instruction::D),
            _ => Err(InvalidInput(s.to_string())),
        }
    }
}

#[derive(Debug)]
struct InvalidInput(String);

fn main() {
    let deltas = fs::read_to_string("assets/input.txt")
        .expect("File")
        .lines()
        .flat_map(|line| {
            let mut sp = line.split_whitespace();
            let instr = sp
                .next()
                .expect("Instr")
                .parse::<Instruction>()
                .expect("Parsed Instr");
            let n = sp
                .next()
                .expect("Count")
                .parse::<usize>()
                .expect("Parsed Count");
            iter::repeat(instr).take(n)
        })
        .map(|i| Coordinates::from(i))
        .collect::<Vec<_>>();

    let mut bridge = Bridge::new(2);
    let mut visited = HashSet::new();
    visited.insert(bridge.tail().clone());
    for delta in deltas.iter() {
        bridge = bridge.updated(delta.clone());
        visited.insert(bridge.tail().clone());
    }
    println!("Visited 2 rope: {}", visited.len());

    let mut bridge = Bridge::new(10);
    let mut visited = HashSet::new();
    visited.insert(bridge.tail().clone());
    for delta in deltas.iter() {
        bridge = bridge.updated(delta.clone());
        visited.insert(bridge.tail().clone());
    }
    println!("Visited 10 rope: {}", visited.len())
}
