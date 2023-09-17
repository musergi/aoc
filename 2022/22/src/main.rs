use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
    fs,
    str::FromStr,
};

#[derive(Debug)]
struct Problem {
    board: Board,
    path: Vec<Step>,
}

#[derive(Debug, PartialEq)]
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
        (position.0 + 1) * 1000 + (position.1 + 1) * 4 + facing.score()
    }
}

trait WrapHandler {
    fn wrap_position(&self, state: &State, board: &Board) -> State;
}

struct Part1;

impl WrapHandler for Part1 {
    fn wrap_position(&self, state: &State, board: &Board) -> State {
        let State { position, facing } = state;
        let tiles = board.empty.union(&board.wall);
        let position = match facing {
            Facing::Right => tiles
                .filter(|(row, _)| *row == position.0)
                .min_by_key(|val| val.1),
            Facing::Left => tiles
                .filter(|(row, _)| *row == position.0)
                .max_by_key(|val| val.1),
            Facing::Down => tiles
                .filter(|(_, col)| *col == position.1)
                .min_by_key(|val| val.0),
            Facing::Up => tiles
                .filter(|(_, col)| *col == position.1)
                .max_by_key(|val| val.0),
        }
        .unwrap()
        .clone();
        State {
            position,
            facing: state.facing,
        }
    }
}

struct Part2;

struct FaceSeq([u8; 4]);

impl FaceSeq {
    fn generate_seq(&self, sub: &[u8; 2]) -> Option<[u8; 4]> {
        let starting_index = self.0.iter().position(|vertex| *vertex == sub[0])?;
        if sub[1] == self.0[Self::next(starting_index)] {
            Some([
                self.0[starting_index],
                self.0[(0..1).fold(starting_index, |a, _| Self::next(a))],
                self.0[(0..2).fold(starting_index, |a, _| Self::next(a))],
                self.0[(0..3).fold(starting_index, |a, _| Self::next(a))],
            ])
        } else {
            None
        }
    }

    fn next(idx: usize) -> usize {
        (idx + 1) % 4
    }
}

const FACE_SEQS: [FaceSeq; 6] = [
    FaceSeq([0, 1, 2, 3]),
    FaceSeq([1, 5, 6, 2]),
    FaceSeq([5, 4, 7, 6]),
    FaceSeq([4, 0, 3, 7]),
    FaceSeq([4, 5, 1, 0]),
    FaceSeq([3, 2, 6, 7]),
];

impl WrapHandler for Part2 {
    fn wrap_position(&self, state: &State, board: &Board) -> State {
        let tiles: HashSet<_> = board.empty.union(&board.wall).cloned().collect();
        let grid_size = grid_size(&tiles);
        let mut lut: HashMap<(i32, i32), HashMap<Facing, [u8; 2]>> = HashMap::new();

        let grid_tile = to_grid(board.starting(), grid_size);
        build_face_lut(&mut lut, &tiles, grid_tile, Facing::Up, [0, 1], grid_size);

        let src_tile = to_grid(state.position, grid_size);
        let src_side = lut.get(&src_tile).unwrap().get(&state.facing).unwrap();
        let dst_side = [src_side[1], src_side[0]];
        let (dst_tile, facing) = lut
            .iter()
            .find_map(|(tile, map)| {
                map.iter()
                    .find(|(_, side)| side[0] == dst_side[0] && side[1] == dst_side[1])
                    .map(|(facing, _)| (tile, facing))
            })
            .unwrap();
        let src_index = side_index(&src_tile, grid_size, state);
        let dst_index = grid_size as usize - 1 - src_index;
        let position = side_tiles(&dst_tile, *facing, grid_size).remove(dst_index);
        let facing = facing.rotate(&Rotation::Right).rotate(&Rotation::Right);
        State { position, facing }
    }
}

fn side_index(tile: &(i32, i32), grid_size: i32, state: &State) -> usize {
    side_tiles(tile, state.facing, grid_size)
        .into_iter()
        .position(|position| position == state.position)
        .unwrap()
}

fn side_tiles(tile: &(i32, i32), facing: Facing, grid_size: i32) -> Vec<(i32, i32)> {
    (0..grid_size)
        .into_iter()
        .map(|i| match facing {
            Facing::Up => (0, i),
            Facing::Down => (grid_size - 1, grid_size - 1 - i),
            Facing::Right => (i, grid_size - 1),
            Facing::Left => (grid_size - 1 - i, 0),
        })
        .map(|position| {
            (
                position.0 + tile.0 * grid_size,
                position.1 + tile.1 * grid_size,
            )
        })
        .collect()
}

fn build_face_lut(
    lut: &mut HashMap<(i32, i32), HashMap<Facing, [u8; 2]>>,
    tiles: &HashSet<(i32, i32)>,
    position: (i32, i32),
    facing: Facing,
    restriction: [u8; 2],
    grid_size: i32,
) {
    let map = build_facing_map(facing, restriction);
    lut.insert(position, map.clone());
    for facing in FACINGS {
        let delta = facing.delta();
        let new_position = (position.0 + delta.0, position.1 + delta.1);
        if !lut.contains_key(&new_position)
            && tiles.contains(&(new_position.0 * grid_size, new_position.1 * grid_size))
        {
            let reverse_facing = facing.rotate(&Rotation::Right).rotate(&Rotation::Right);
            let side = map.get(&facing).unwrap();
            let restriction = [side[1], side[0]];
            build_face_lut(
                lut,
                tiles,
                new_position,
                reverse_facing,
                restriction,
                grid_size,
            );
        }
    }
}

fn build_facing_map(mut facing: Facing, vertices: [u8; 2]) -> HashMap<Facing, [u8; 2]> {
    let seq = FACE_SEQS
        .iter()
        .find_map(|seq| seq.generate_seq(&vertices))
        .unwrap();
    let mut res = HashMap::new();
    for i in 0..4 {
        res.insert(facing, [seq[i], seq[FaceSeq::next(i)]]);
        facing = facing.rotate(&Rotation::Right);
    }
    res
}

fn to_grid(position: (i32, i32), grid_size: i32) -> (i32, i32) {
    (position.0 / grid_size, position.1 / grid_size)
}

fn grid_size(tiles: &HashSet<(i32, i32)>) -> i32 {
    let mut sizes = HashSet::new();
    let rows: HashSet<_> = tiles.iter().map(|tile| tile.1).collect();
    for row in rows {
        let row_tiles: Vec<_> = tiles
            .iter()
            .filter(|tile| tile.1 == row)
            .map(|tile| tile.0)
            .collect();
        let min = row_tiles.iter().min();
        let max = row_tiles.iter().max();
        if let (Some(min), Some(max)) = (min, max) {
            sizes.insert(max - min + 1);
        }
    }
    let cols: HashSet<_> = tiles.iter().map(|tile| tile.0).collect();
    for col in cols {
        let col_tiles: Vec<_> = tiles
            .iter()
            .filter(|tile| tile.0 == col)
            .map(|tile| tile.1)
            .collect();
        let min = col_tiles.iter().min();
        let max = col_tiles.iter().max();
        if let (Some(min), Some(max)) = (min, max) {
            sizes.insert(max - min + 1);
        }
    }
    sizes.into_iter().reduce(gcd).unwrap()
}

fn gcd(mut n: i32, mut m: i32) -> i32 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_t_field(t: i32) -> HashSet<(i32, i32)> {
        let mut empty = HashSet::new();
        for i in 0..t * 3 {
            for j in 0..t {
                empty.insert((j, i));
            }
        }
        for i in t..t * 4 {
            for j in 0..t {
                empty.insert((i, j + t));
            }
        }
        empty
    }

    #[test]
    fn grid_size_for_size_2() {
        let empty = build_t_field(2);
        assert_eq!(grid_size(&empty), 2);
    }

    #[test]
    fn grid_size_for_size_3() {
        let empty = build_t_field(3);
        assert_eq!(grid_size(&empty), 3);
    }

    #[test]
    fn grid_size_for_size_4() {
        let empty = build_t_field(4);
        assert_eq!(grid_size(&empty), 4);
    }

    #[test]
    fn size2() {
        let empty = build_t_field(2);
        let wall = HashSet::new();
        let board = Board { empty, wall };
        let state = State {
            position: (1, 1),
            facing: Facing::Down,
        };
        assert_eq!(
            Part2.wrap_position(&state, &board),
            State {
                position: (2, 2),
                facing: Facing::Right
            }
        );
        let state = State {
            position: (1, 0),
            facing: Facing::Left,
        };
        assert_eq!(
            Part2.wrap_position(&state, &board),
            State {
                position: (4, 2),
                facing: Facing::Right
            }
        );
        let state = State {
            position: (0, 2),
            facing: Facing::Up,
        };
        assert_eq!(
            Part2.wrap_position(&state, &board),
            State {
                position: (7, 2),
                facing: Facing::Up,
            }
        );
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
        for _ in 0..count {
            let delta = state.facing.delta();
            let mut next_state = state.from_delta(delta);
            if !self.exists(&next_state.position) {
                next_state = wrap.wrap_position(&state, &self.board);
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
        let mut row = 0;
        while let Some(line) = it.next() {
            if line.is_empty() {
                break;
            }
            let mut col = 0;
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

const FACINGS: [Facing; 4] = [Facing::Right, Facing::Down, Facing::Left, Facing::Up];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    let part2 = problem.password(Part2);
    println!("Part 2: {}", part2);
}
