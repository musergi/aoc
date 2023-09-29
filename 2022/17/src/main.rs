use std::collections::HashMap;

fn main() {
    let s = std::fs::read_to_string("assets/input.txt").unwrap();
    let moves = parse(&s);
    let stack = Stack::default();
    println!("Part 1: {}", stack.stack_size(2022, &moves));
    println!("Part 2: {}", stack.stack_size_hash(1000000000000, &moves));
}

static SHAPES: [Shape; 5] = [
    Shape([0b1111, 0, 0, 0]),
    Shape([0b10, 0b111, 0b10, 0]),
    Shape([0b111, 0b100, 0b100, 0]),
    Shape([0b1, 0b1, 0b1, 0b1]),
    Shape([0b11, 0b11, 0, 0]),
];

struct Shape([u8; 4]);

enum Move {
    Left,
    Right,
}

fn parse(s: &str) -> Vec<Move> {
    s.chars()
        .filter_map(|c| match c {
            '>' => Some(Move::Right),
            '<' => Some(Move::Left),
            _ => None,
        })
        .collect()
}

struct FallingShape {
    left_offset: u8,
    bottom_offset: usize,
    shape: &'static Shape,
}

impl FallingShape {
    fn can_shift_left(&self, rows: &[u8]) -> bool {
        self.left_offset > 0
            && self.shape.0.iter().enumerate().all(|(idx, row)| {
                rows.get(self.bottom_offset + idx)
                    .map(|placed| (placed & (row << self.left_offset - 1)) == 0)
                    .unwrap_or(true)
            })
    }

    fn can_shift_right(&self, rows: &[u8]) -> bool {
        self.shape
            .0
            .iter()
            .map(|v| v.leading_zeros() - 1)
            .min()
            .unwrap() as u8
            > self.left_offset
            && self.shape.0.iter().enumerate().all(|(idx, row)| {
                rows.get(self.bottom_offset + idx)
                    .map(|placed| (placed & (row << self.left_offset + 1)) == 0)
                    .unwrap_or(true)
            })
    }

    fn shift_left(&mut self) {
        self.left_offset -= 1;
    }

    fn shift_right(&mut self) {
        self.left_offset += 1;
    }

    fn can_fall(&self, rows: &[u8]) -> bool {
        match self.bottom_offset.checked_sub(1) {
            Some(new_bottom) => self.shape.0.iter().enumerate().all(|(idx, row)| {
                rows.get(new_bottom + idx)
                    .map(|placed| (placed & (row << self.left_offset)) == 0)
                    .unwrap_or(true)
            }),
            None => false,
        }
    }

    fn fall(&mut self) {
        self.bottom_offset -= 1;
    }
}

impl FallingShape {
    fn new(top: usize, shape: usize) -> Self {
        Self {
            left_offset: 2,
            bottom_offset: top + 3,
            shape: &SHAPES[shape],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct CacheKey {
    move_idx: usize,
    shape_idx: usize,
    shape: Vec<u8>,
}

#[derive(Debug, Default, Clone)]
struct Stack {
    rows: Vec<u8>,
}

impl Stack {
    fn stack_size(&self, n: usize, moves: &[Move]) -> usize {
        let mut stack = self.clone();
        let mut off = 0;
        let mut shape = 0;
        for _ in 0..n {
            off = stack.place(off, moves, shape);
            shape = (shape + 1) % SHAPES.len();
        }
        stack.rows.len()
    }

    fn stack_size_hash(&self, n: usize, moves: &[Move]) -> usize {
        let mut stack = self.clone();
        let mut off = 0;
        let mut shape = 0;
        let mut cache = HashMap::new();
        for idx in 0..n {
            off = stack.place(off, moves, shape);
            let key = CacheKey {
                move_idx: off,
                shape_idx: shape,
                shape: stack.open_shape(),
            };
            if cache.contains_key(&key) {
                let (last_idx, last) = cache.get(&key).unwrap();
                let period = idx - last_idx;
                if (n - 1 - idx) % period == 0 {
                    return stack.rows.len() + (n - 1 - idx) / period * (stack.rows.len() - last);
                }
            } else {
                cache.insert(key, (idx, stack.rows.len()));
            }
            shape = (shape + 1) % SHAPES.len();
        }
        stack.rows.len()
    }

    fn place(&mut self, mut move_offset: usize, moves: &[Move], shape: usize) -> usize {
        let mut shape = FallingShape::new(self.rows.len(), shape);
        loop {
            match moves[move_offset] {
                Move::Left => {
                    if shape.can_shift_left(&self.rows) {
                        shape.shift_left();
                    }
                }
                Move::Right => {
                    if shape.can_shift_right(&self.rows) {
                        shape.shift_right();
                    }
                }
            };
            move_offset = (move_offset + 1) % moves.len();
            if shape.can_fall(&self.rows) {
                shape.fall();
            } else {
                break;
            }
        }
        for relative_offset in 0..4 {
            let offset = shape.bottom_offset + relative_offset;
            let new_row = shape.shape.0[relative_offset] << shape.left_offset;
            match self.rows.get_mut(offset) {
                Some(row) => *row |= new_row,
                None => {
                    if new_row != 0 {
                        self.rows.push(new_row)
                    }
                }
            }
        }
        move_offset
    }

    fn open_shape(&self) -> Vec<u8> {
        let mut shape = Vec::new();
        for row in self.rows.iter().rev() {
            let mut empty_itervals = Vec::new();
            for tile in 0..7 {
                let is_empty = (row & (1 << tile)) == 0;
                if is_empty {
                    if empty_itervals
                        .last()
                        .map(|&(_, b)| b == tile - 1)
                        .unwrap_or(false)
                    {
                        empty_itervals.last_mut().unwrap().1 = tile;
                    } else {
                        empty_itervals.push((tile, tile));
                    }
                }
            }
            let row = empty_itervals
                .into_iter()
                .map(|(a, b)| {
                    (a..=b)
                        .into_iter()
                        .map(|o| 1 << o)
                        .reduce(|a, b| a | b)
                        .unwrap()
                })
                .filter(|val| shape.last().map(|last| last & val != 0).unwrap_or(true))
                .fold(0, |a, b| a | b);
            if row == 0 {
                break;
            } else {
                shape.push(row);
            }
        }
        shape
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_no_offset_cant_shift_lefth() {
        let f = FallingShape {
            left_offset: 0,
            bottom_offset: 0,
            shape: &SHAPES[0],
        };
        assert!(!f.can_shift_left(&Vec::new()));
    }

    #[test]
    fn when_offset_can_shift_left() {
        let f = FallingShape {
            left_offset: 1,
            bottom_offset: 0,
            shape: &SHAPES[0],
        };
        assert!(f.can_shift_left(&Vec::new()));
    }

    #[test]
    fn when_enough_offset_can_shift_right() {
        let f = FallingShape {
            left_offset: 2,
            bottom_offset: 0,
            shape: &SHAPES[0],
        };
        assert!(f.can_shift_right(&Vec::new()));
    }

    #[test]
    fn when_not_enough_offset_can_shift_right() {
        let f = FallingShape {
            left_offset: 3,
            bottom_offset: 0,
            shape: &SHAPES[0],
        };
        assert!(!f.can_shift_right(&Vec::new()));
    }

    #[test]
    fn test_one_placement() {
        let s = include_str!("../assets/example.txt");
        let moves = parse(s);
        let mut stack = Stack::default();
        let _off = stack.place(0, &moves, 0);
        assert_eq!(*stack.rows.get(0).unwrap(), 0b111100);
    }

    #[test]
    fn test_two_placement() {
        let s = include_str!("../assets/example.txt");
        let moves = parse(s);
        let mut stack = Stack::default();
        let off = stack.place(0, &moves, 0);
        let _off = stack.place(off, &moves, 1);
        assert_eq!(*stack.rows.get(0).unwrap(), 0b111100);
        assert_eq!(*stack.rows.get(1).unwrap(), 0b001000);
        assert_eq!(*stack.rows.get(2).unwrap(), 0b011100);
        assert_eq!(*stack.rows.get(3).unwrap(), 0b001000);
    }

    #[test]
    fn test_three_placement() {
        let s = include_str!("../assets/example.txt");
        let moves = parse(s);
        let mut stack = Stack::default();
        let off = stack.place(0, &moves, 0);
        let off = stack.place(off, &moves, 1);
        let _off = stack.place(off, &moves, 2);
        assert_eq!(*stack.rows.get(0).unwrap(), 0b111100);
        assert_eq!(*stack.rows.get(1).unwrap(), 0b001000);
        assert_eq!(*stack.rows.get(2).unwrap(), 0b011100);
        assert_eq!(*stack.rows.get(3).unwrap(), 0b001111);
        assert_eq!(*stack.rows.get(4).unwrap(), 0b000100);
        assert_eq!(*stack.rows.get(5).unwrap(), 0b000100);
    }

    #[test]
    fn test_four_placement() {
        let s = include_str!("../assets/example.txt");
        let moves = parse(s);
        let mut stack = Stack::default();
        let off = stack.place(0, &moves, 0);
        let off = stack.place(off, &moves, 1);
        let off = stack.place(off, &moves, 2);
        let _off = stack.place(off, &moves, 3);
        assert_eq!(*stack.rows.get(0).unwrap(), 0b111100);
        assert_eq!(*stack.rows.get(1).unwrap(), 0b001000);
        assert_eq!(*stack.rows.get(2).unwrap(), 0b011100);
        assert_eq!(*stack.rows.get(3).unwrap(), 0b011111);
        assert_eq!(*stack.rows.get(4).unwrap(), 0b010100);
        assert_eq!(*stack.rows.get(5).unwrap(), 0b010100);
        assert_eq!(*stack.rows.get(6).unwrap(), 0b010000);
    }

    #[test]
    fn test_five_placement() {
        let s = include_str!("../assets/example.txt");
        let moves = parse(s);
        let mut stack = Stack::default();
        let off = stack.place(0, &moves, 0);
        let off = stack.place(off, &moves, 1);
        let off = stack.place(off, &moves, 2);
        let off = stack.place(off, &moves, 3);
        let _off = stack.place(off, &moves, 4);
        assert_eq!(*stack.rows.get(0).unwrap(), 0b111100);
        assert_eq!(*stack.rows.get(1).unwrap(), 0b001000);
        assert_eq!(*stack.rows.get(2).unwrap(), 0b011100);
        assert_eq!(*stack.rows.get(3).unwrap(), 0b011111);
        assert_eq!(*stack.rows.get(4).unwrap(), 0b010100);
        assert_eq!(*stack.rows.get(5).unwrap(), 0b010100);
        assert_eq!(*stack.rows.get(6).unwrap(), 0b010000);
        assert_eq!(*stack.rows.get(7).unwrap(), 0b110000);
        assert_eq!(*stack.rows.get(8).unwrap(), 0b110000);
    }

    #[test]
    fn test_six_placement() {
        let s = include_str!("../assets/example.txt");
        let moves = parse(s);
        let mut stack = Stack::default();
        let off = stack.place(0, &moves, 0);
        let off = stack.place(off, &moves, 1);
        let off = stack.place(off, &moves, 2);
        let off = stack.place(off, &moves, 3);
        let off = stack.place(off, &moves, 4);
        let _off = stack.place(off, &moves, 0);
        assert_eq!(*stack.rows.get(0).unwrap(), 0b111100);
        assert_eq!(*stack.rows.get(1).unwrap(), 0b001000);
        assert_eq!(*stack.rows.get(2).unwrap(), 0b011100);
        assert_eq!(*stack.rows.get(3).unwrap(), 0b011111);
        assert_eq!(*stack.rows.get(4).unwrap(), 0b010100);
        assert_eq!(*stack.rows.get(5).unwrap(), 0b010100);
        assert_eq!(*stack.rows.get(6).unwrap(), 0b010000);
        assert_eq!(*stack.rows.get(7).unwrap(), 0b110000);
        assert_eq!(*stack.rows.get(8).unwrap(), 0b110000);
        assert_eq!(*stack.rows.get(9).unwrap(), 0b011110);
    }

    #[test]
    fn example_part1() {
        let s = include_str!("../assets/example.txt");
        let moves = parse(s);
        let stack = Stack::default();
        assert_eq!(stack.stack_size(2022, &moves), 3068);
    }

    #[test]
    fn test_open_shape_height_one() {
        let s = include_str!("../assets/example.txt");
        let moves = parse(s);
        let mut stack = Stack::default();
        let _off = stack.place(0, &moves, 0);
        let shape = stack.open_shape();
        assert_eq!(*shape.get(0).unwrap(), 0b1000011);
    }

    #[test]
    fn test_open_shape_overhang() {
        let s = include_str!("../assets/example.txt");
        let moves = parse(s);
        let mut stack = Stack::default();
        let off = stack.place(0, &moves, 0);
        let _off = stack.place(off, &moves, 1);
        let shape = stack.open_shape();
        assert_eq!(*shape.get(0).unwrap(), 0b1110111);
        assert_eq!(*shape.get(1).unwrap(), 0b1100011);
        assert_eq!(*shape.get(2).unwrap(), 0b1110111);
        assert_eq!(*shape.get(3).unwrap(), 0b1000011);
    }

    #[test]
    fn test_open_shape_closed() {
        let s = include_str!("../assets/example.txt");
        let moves = parse(s);
        let mut stack = Stack::default();
        let off = stack.place(0, &moves, 0);
        let off = stack.place(off, &moves, 1);
        let _off = stack.place(off, &moves, 2);
        let shape = stack.open_shape();
        assert_eq!(*shape.get(0).unwrap(), 0b1111011);
        assert_eq!(*shape.get(1).unwrap(), 0b1111011);
        assert_eq!(*shape.get(2).unwrap(), 0b1110000);
        assert_eq!(*shape.get(3).unwrap(), 0b1100000);
        assert_eq!(*shape.get(4).unwrap(), 0b1110000);
        assert_eq!(*shape.get(5).unwrap(), 0b1000000);
    }

    #[test]
    fn example_part2() {
        let s = include_str!("../assets/example.txt");
        let moves = parse(s);
        let stack = Stack::default();
        assert_eq!(stack.stack_size_hash(1000000000000, &moves), 1514285714288);
    }
}
