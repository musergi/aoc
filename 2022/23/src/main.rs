use std::{
    collections::HashSet,
    convert::Infallible,
    fs::read_to_string,
    ops::{Add, AddAssign},
    str::FromStr,
};

fn main() {
    let content = read_to_string("assets/input.txt").unwrap();
    let state: State = content.parse().unwrap();
    {
        let mut state = state.clone();
        for _ in 0..10 {
            state.next();
        }
        println!("Part 1: {}", state.empty_tiles())
    }
    {
        let mut state = state.clone();
        println!("Part 2: {}", state.rounds_until_spaced())
    }
}

#[derive(Debug, Clone)]
struct State {
    elf_positions: Vec<Vector2>,
    moves: [Move; 4],
}

impl State {
    fn rounds_until_spaced(&mut self) -> usize {
        let mut count = 0;
        while self.next() {
            count += 1
        }
        count + 1
    }

    fn next(&mut self) -> bool {
        let mut elf_positions = Vec::with_capacity(self.elf_positions.len());
        for elf_position in self.elf_positions.iter() {
            let has_other_arround = Self::all_directions()
                .iter()
                .map(|delta| *elf_position + *delta)
                .any(|position| self.elf_positions.contains(&position));
            let new_position = if has_other_arround {
                self.moves
                    .iter()
                    .find(|m| {
                        let is_occupied = m.deltas().into_iter().any(|delta| {
                            let new_position = *elf_position + delta;
                            self.elf_positions.contains(&new_position)
                        });
                        !is_occupied
                    })
                    .map(|m| *elf_position + m.delta())
                    .unwrap_or(*elf_position)
            } else {
                *elf_position
            };
            elf_positions.push(new_position);
        }
        let mut found = HashSet::new();
        let mut repeated = HashSet::new();
        for elf_position in elf_positions.iter() {
            if found.contains(elf_position) {
                repeated.insert(*elf_position);
            } else {
                found.insert(*elf_position);
            }
        }
        for (idx, elf_position) in elf_positions.iter_mut().enumerate() {
            if repeated.contains(elf_position) {
                *elf_position = self.elf_positions[idx];
            }
        }
        for i in 0..self.moves.len() - 1 {
            self.moves.swap(i, i + 1);
        }
        if self.elf_positions != elf_positions {
            self.elf_positions = elf_positions;
            true
        } else {
            false
        }
    }

    fn all_directions() -> [Vector2; 8] {
        [
            Vector2::north_west(),
            Vector2::north(),
            Vector2::north_east(),
            Vector2::east(),
            Vector2::south_east(),
            Vector2::south(),
            Vector2::south_west(),
            Vector2::west(),
        ]
    }

    fn empty_tiles(&self) -> usize {
        let width = self.elf_positions.iter().map(|p| p.col).max().unwrap()
            - self.elf_positions.iter().map(|p| p.col).min().unwrap()
            + 1;
        let height = self.elf_positions.iter().map(|p| p.row).max().unwrap()
            - self.elf_positions.iter().map(|p| p.row).min().unwrap()
            + 1;
        width as usize * height as usize - self.elf_positions.len()
    }
}

impl FromStr for State {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut position = Vector2::zero();
        let mut elf_positions = Vec::new();
        for line in s.lines() {
            for c in line.chars() {
                if c == '#' {
                    elf_positions.push(position);
                }
                position += Vector2::east();
            }
            position += Vector2::south();
            position.col = 0;
        }
        let moves = [Move::North, Move::South, Move::West, Move::East];
        Ok(Self {
            elf_positions,
            moves,
        })
    }
}

#[derive(Debug, Clone)]
enum Move {
    North,
    East,
    South,
    West,
}

impl Move {
    fn deltas(&self) -> [Vector2; 3] {
        match self {
            Move::North => [
                Vector2::north(),
                Vector2::north_east(),
                Vector2::north_west(),
            ],
            Move::East => [
                Vector2::east(),
                Vector2::north_east(),
                Vector2::south_east(),
            ],
            Move::South => [
                Vector2::south(),
                Vector2::south_east(),
                Vector2::south_west(),
            ],
            Move::West => [
                Vector2::west(),
                Vector2::north_west(),
                Vector2::south_west(),
            ],
        }
    }

    fn delta(&self) -> Vector2 {
        match self {
            Move::North => Vector2::north(),
            Move::East => Vector2::east(),
            Move::South => Vector2::south(),
            Move::West => Vector2::west(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vector2 {
    row: i32,
    col: i32,
}

impl Vector2 {
    fn zero() -> Self {
        Self { row: 0, col: 0 }
    }

    fn south() -> Self {
        Self { row: 1, col: 0 }
    }

    fn east() -> Self {
        Self { row: 0, col: 1 }
    }

    fn west() -> Self {
        Self { row: 0, col: -1 }
    }

    fn north() -> Vector2 {
        Self { row: -1, col: 0 }
    }

    fn north_east() -> Vector2 {
        Self::north() + Self::east()
    }

    fn north_west() -> Vector2 {
        Self::north() + Self::west()
    }

    fn south_east() -> Vector2 {
        Self::south() + Self::east()
    }

    fn south_west() -> Vector2 {
        Self::south() + Self::west()
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_small_example() {
        let example = include_str!("../assets/small_example.txt");
        let state: State = example.parse().expect("expected parsed initial state");
        assert_eq!(state.elf_positions.len(), 5);
        assert!(state.elf_positions.contains(&Vector2 { row: 1, col: 2 }));
        assert!(state.elf_positions.contains(&Vector2 { row: 1, col: 3 }));
        assert!(state.elf_positions.contains(&Vector2 { row: 2, col: 2 }));
        assert!(state.elf_positions.contains(&Vector2 { row: 4, col: 2 }));
        assert!(state.elf_positions.contains(&Vector2 { row: 4, col: 3 }));
    }

    #[test]
    fn one_state_transition() {
        let example = include_str!("../assets/small_example.txt");
        let mut state: State = example.parse().expect("expected parsed initial state");
        state.next();
        assert_eq!(state.elf_positions.len(), 5);
        assert!(state.elf_positions.contains(&Vector2 { row: 0, col: 2 }));
        assert!(state.elf_positions.contains(&Vector2 { row: 0, col: 3 }));
        assert!(state.elf_positions.contains(&Vector2 { row: 2, col: 2 }));
        assert!(state.elf_positions.contains(&Vector2 { row: 4, col: 2 }));
        assert!(state.elf_positions.contains(&Vector2 { row: 3, col: 3 }));
    }

    #[test]
    fn two_state_transition() {
        let example = include_str!("../assets/small_example.txt");
        let mut state: State = example.parse().expect("expected parsed initial state");
        state.next();
        state.next();
        assert_eq!(state.elf_positions.len(), 5);
        assert!(state.elf_positions.contains(&Vector2 { row: 1, col: 2 }));
        assert!(state.elf_positions.contains(&Vector2 { row: 1, col: 3 }));
        assert!(state.elf_positions.contains(&Vector2 { row: 2, col: 1 }));
        assert!(state.elf_positions.contains(&Vector2 { row: 3, col: 4 }));
        assert!(state.elf_positions.contains(&Vector2 { row: 5, col: 2 }));
    }

    #[test]
    fn stable_state_when_separated() {
        let example = include_str!("../assets/small_example.txt");
        let mut state: State = example.parse().expect("expected parsed initial state");
        for _ in 0..10 {
            state.next();
        }
        assert_eq!(state.elf_positions.len(), 5);
        assert!(state.elf_positions.contains(&Vector2 { row: 0, col: 2 }));
        assert!(state.elf_positions.contains(&Vector2 { row: 1, col: 4 }));
        assert!(state.elf_positions.contains(&Vector2 { row: 2, col: 0 }));
        assert!(state.elf_positions.contains(&Vector2 { row: 3, col: 4 }));
        assert!(state.elf_positions.contains(&Vector2 { row: 5, col: 2 }));
    }

    #[test]
    fn example_final_state() {
        let example = include_str!("../assets/example.txt");
        let mut state: State = example.parse().expect("expected parsed initial state");
        for _ in 0..10 {
            state.next();
        }
        assert_eq!(state.elf_positions.len(), 22);
        assert!(state.elf_positions.contains(&Vector2 { row: 0, col: 7 }));
        assert!(state.elf_positions.contains(&Vector2 { row: 1, col: 11 }));
        assert!(state.elf_positions.contains(&Vector2 { row: 2, col: 2 }));
        assert!(state.elf_positions.contains(&Vector2 { row: 2, col: 4 }));
        assert!(state.elf_positions.contains(&Vector2 { row: 2, col: 7 }));
        assert!(state.elf_positions.contains(&Vector2 { row: 3, col: 6 }));
    }

    #[test]
    fn example_part1() {
        let example = include_str!("../assets/example.txt");
        let mut state: State = example.parse().expect("expected parsed initial state");
        for _ in 0..10 {
            state.next();
        }
        assert_eq!(state.empty_tiles(), 110);
    }

    #[test]
    fn example_part2() {
        let example = include_str!("../assets/example.txt");
        let mut state: State = example.parse().expect("expected parsed initial state");
        assert_eq!(state.rounds_until_spaced(), 20);
    }
}
