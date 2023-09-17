use std::{collections::HashSet, convert::Infallible, fs, str::FromStr};

#[derive(Debug)]
struct Problem {
    board: Board,
    path: Vec<Step>,
}

#[derive(Debug)]
struct State {
    position: (i32, i32),
    facing: Facing,
}

impl State {
    fn new(position: (i32, i32)) -> Self {
        Self {
            position,
            facing: Facing::Right,
        }
    }

    fn from_delta(&self, delta: (i32, i32)) -> Self {
        Self {
            position: (self.position.0 + delta.0, self.position.1 + delta.1),
            facing: self.facing,
        }
    }

    fn score(&self) -> i32 {
        let State { position, facing } = self;
        position.0 * 1000 + position.1 * 4 + facing.score()
    }
}

trait WrapHandler {
    fn wrap_position(
        &self,
        state: &State,
        empty: &HashSet<(i32, i32)>,
        wall: &HashSet<(i32, i32)>,
    ) -> State;
}

struct Part1;

impl WrapHandler for Part1 {
    fn wrap_position(
        &self,
        state: &State,
        empty: &HashSet<(i32, i32)>,
        wall: &HashSet<(i32, i32)>,
    ) -> State {
        let State { position, facing } = state;
        let tiles = empty.union(wall);
        let position = match facing {
            Facing::Right => tiles
                .filter(|(row, _)| *row == position.0)
                .min_by(|left, right| left.1.cmp(&right.1))
                .unwrap(),
            Facing::Left => tiles
                .filter(|(row, _)| *row == position.0)
                .max_by(|left, right| left.1.cmp(&right.1))
                .unwrap(),
            Facing::Down => tiles
                .filter(|(_, col)| *col == position.1)
                .min_by(|left, right| left.0.cmp(&right.0))
                .unwrap(),
            Facing::Up => tiles
                .filter(|(_, col)| *col == position.1)
                .max_by(|left, right| left.0.cmp(&right.0))
                .unwrap(),
        }
        .clone();
        State {
            position,
            facing: state.facing,
        }
    }
}

impl Problem {
    fn password(&self, wrap: impl WrapHandler) -> i32 {
        let starting_position = self.board.starting();
        let mut state = State::new(starting_position);
        for step in self.path.iter() {
            self.apply(&mut state, step, &wrap);
        }
        state.score()
    }

    fn apply(&self, state: &mut State, step: &Step, wrap: &impl WrapHandler) {
        match step {
            Step::Advance(n) => self.apply_advance(state, *n, wrap),
            Step::Rotate(rotation) => {
                state.facing = state.facing.rotate(rotation);
            }
        }
    }

    fn apply_advance(&self, state: &mut State, count: i32, wrap: &impl WrapHandler) {
        let delta = state.facing.delta();
        for _ in 0..count {
            let mut next_state = state.from_delta(delta);
            if !self.exists(&next_state.position) {
                next_state = wrap.wrap_position(&state, &self.board.empty, &self.board.wall);
            }
            if self.board.empty.contains(&next_state.position) {
                *state = next_state;
            } else if self.board.wall.contains(&next_state.position) {
                break;
            } else {
                panic!("error");
            }
        }
    }

    fn exists(&self, position: &(i32, i32)) -> bool {
        self.board.empty.contains(position) || self.board.wall.contains(position)
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
        path.push(Step::Advance(path_string[last_letter..].parse().unwrap()));

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

#[derive(Debug, Clone, Copy)]
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
            Facing::Up => (-1, 0),
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
    let part1 = problem.password(Part1);
    println!("Part 1: {}", part1);
}
