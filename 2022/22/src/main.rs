use std::{collections::HashSet, convert::Infallible, fs, str::FromStr};

#[derive(Debug)]
struct Problem {
    board: Board,
    path: Vec<Step>,
}

impl Problem {
    fn password(&self) -> i32 {
        let mut position = self.board.starting();
        let mut facing = Facing::Right;
        for step in self.path.iter() {
            println!("{:?}", step);
            self.apply(&mut position, &mut facing, step);
            println!("{:?}", position);
        }
        println!("{:?}", position);
        position.0 * 1000 + position.1 * 4 + facing.score()
    }

    fn apply(&self, position: &mut (i32, i32), facing: &mut Facing, step: &Step) {
        match step {
            Step::Advance(n) => self.apply_advance(position, facing, *n),
            Step::Rotate(rotation) => {
                *facing = facing.rotate(rotation);
            }
        }
    }

    fn apply_advance(&self, position: &mut (i32, i32), facing: &Facing, count: i32) {
        let delta = facing.delta();
        for _ in 0..count {
            let next_position = (position.0 + delta.0, position.1 + delta.1);
            if self.board.empty.contains(&next_position) {
                *position = next_position;
            } else if self.board.wall.contains(&next_position) {
                break;
            } else {
                let next_position = match facing {
                    Facing::Right => self
                        .board
                        .empty
                        .union(&self.board.wall)
                        .filter(|(row, _)| *row == position.0)
                        .min_by(|left, right| left.1.cmp(&right.1))
                        .unwrap(),
                    Facing::Left => self
                        .board
                        .empty
                        .union(&self.board.wall)
                        .filter(|(row, _)| *row == position.0)
                        .max_by(|left, right| left.1.cmp(&right.1))
                        .unwrap(),
                    Facing::Down => self
                        .board
                        .empty
                        .union(&self.board.wall)
                        .filter(|(_, col)| *col == position.1)
                        .min_by(|left, right| left.0.cmp(&right.0))
                        .unwrap(),
                    Facing::Up => self
                        .board
                        .empty
                        .union(&self.board.wall)
                        .filter(|(_, col)| *col == position.1)
                        .max_by(|left, right| left.0.cmp(&right.0))
                        .unwrap(),
                }
                .clone();
                if self.board.empty.contains(&next_position) {
                    *position = next_position;
                } else if self.board.wall.contains(&next_position) {
                    break;
                }
            }
        }
    }
}

impl FromStr for Problem {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.lines();
        let mut empty = HashSet::new();
        let mut wall = HashSet::new();
        let mut row = 1;
        while let Some(line) = it.next() {
            if line.is_empty() {
                break;
            }
            let mut col = 1;
            for c in line.chars() {
                match c {
                    '.' => empty.insert((row, col)),
                    '#' => wall.insert((row, col)),
                    _ => false,
                };
                col += 1;
            }
            row += 1;
        }
        let path_string = it.next().unwrap();
        let mut last_letter = 0;
        let mut path = Vec::new();
        for (idx, c) in path_string.char_indices() {
            if c == 'R' {
                path.push(Step::Advance(
                    path_string[last_letter..idx].parse().unwrap(),
                ));
                path.push(Step::Rotate(Rotation::Right));
                last_letter = idx + 1;
            } else if c == 'L' {
                path.push(Step::Advance(
                    path_string[last_letter..idx].parse().unwrap(),
                ));
                path.push(Step::Rotate(Rotation::Left));
                last_letter = idx + 1;
            }
        }

        Ok(Self {
            board: Board { empty, wall },
            path,
        })
    }
}

#[derive(Debug)]
struct Board {
    empty: HashSet<(i32, i32)>,
    wall: HashSet<(i32, i32)>,
}

impl Board {
    fn starting(&self) -> (i32, i32) {
        self.empty.iter().min().unwrap().clone()
    }
}

#[derive(Debug)]
enum Facing {
    Right,
    Down,
    Left,
    Up,
}

impl Facing {
    fn rotate(&self, rotation: &Rotation) -> Self {
        match (self, rotation) {
            (Facing::Right, Rotation::Right) => Facing::Down,
            (Facing::Right, Rotation::Left) => Facing::Up,
            (Facing::Down, Rotation::Right) => Facing::Left,
            (Facing::Down, Rotation::Left) => Facing::Right,
            (Facing::Left, Rotation::Right) => Facing::Up,
            (Facing::Left, Rotation::Left) => Facing::Down,
            (Facing::Up, Rotation::Right) => Facing::Right,
            (Facing::Up, Rotation::Left) => Facing::Left,
        }
    }

    fn delta(&self) -> (i32, i32) {
        match self {
            Facing::Right => (0, 1),
            Facing::Down => (1, 0),
            Facing::Left => (0, -1),
            Facing::Up => (-1, 1),
        }
    }

    fn score(&self) -> i32 {
        match self {
            Facing::Right => 0,
            Facing::Down => 1,
            Facing::Left => 2,
            Facing::Up => 3,
        }
    }
}

#[derive(Debug)]
enum Step {
    Advance(i32),
    Rotate(Rotation),
}

#[derive(Debug)]
enum Rotation {
    Right,
    Left,
}

fn main() {
    let s = fs::read_to_string("assets/input.txt").unwrap();
    let problem: Problem = s.parse().unwrap();
    let part1 = problem.password();
    println!("Part 1: {}", part1);
}
