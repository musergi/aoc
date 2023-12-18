use crate::{direction::Direction, pipe_type::PipeType, vec2::Vec2};
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

pub struct Pipes {
    pipes: HashMap<Vec2, PipeType>,
}

impl Pipes {
    pub fn farthest_distance(mut self) -> usize {
        self.simplify();
        (self.pipes.len() + 1) / 2
    }

    pub fn inner_tiles(mut self) -> usize {
        self.simplify();
        let x_min = self.pipes.keys().map(|vec| vec.x).min().unwrap();
        let x_max = self.pipes.keys().map(|vec| vec.x).max().unwrap();
        let y_min = self.pipes.keys().map(|vec| vec.y).min().unwrap();
        let y_max = self.pipes.keys().map(|vec| vec.y).max().unwrap();
        let start_replacement = self.get_start_replacement();
        (x_min..=x_max)
            .into_iter()
            .flat_map(|x| (y_min..=y_max).into_iter().map(move |y| Vec2::new(x, y)))
            .filter(|vec| !self.pipes.contains_key(vec))
            .filter(|vec| self.is_inner(vec, start_replacement))
            .count()
    }

    fn is_inner(&self, vec: &Vec2, start_replacement: PipeType) -> bool {
        let mut left_tiles: Vec<_> = self
            .pipes
            .iter()
            .filter(|(position, _)| position.x < vec.x && position.y == vec.y)
            .map(|(position, pipe)| (position.x, pipe))
            .collect();
        left_tiles.sort_by_key(|(x, _)| x.clone());
        left_tiles
            .iter()
            .rev()
            .fold((0, None), |(count, prev), (_, &pipe)| {
                let pipe = if pipe == PipeType::Start {
                    start_replacement
                } else {
                    pipe
                };
                match pipe {
                    PipeType::NorthSouth => (count + 1, None),
                    PipeType::EastWest => (count, prev),
                    PipeType::NorthEast => (
                        count
                            + if prev.unwrap() == PipeType::SouthWest {
                                1
                            } else {
                                0
                            },
                        None,
                    ),
                    PipeType::NorthWest => (count, Some(PipeType::NorthWest)),
                    PipeType::SouthWest => (count, Some(PipeType::SouthWest)),
                    PipeType::SouthEast => (
                        count
                            + if prev.unwrap() == PipeType::NorthWest {
                                1
                            } else {
                                0
                            },
                        None,
                    ),
                    PipeType::Start => panic!("start not allowed"),
                }
            })
            .0
            % 2
            == 1
    }

    fn get_start_replacement(&self) -> PipeType {
        let start = self.start().0.clone();
        match (
            self.connected(start, Direction::North),
            self.connected(start, Direction::South),
            self.connected(start, Direction::East),
            self.connected(start, Direction::West),
        ) {
            (true, true, false, false) => PipeType::NorthSouth,
            (true, false, true, false) => PipeType::NorthEast,
            (true, false, false, true) => PipeType::NorthWest,
            (false, true, true, false) => PipeType::SouthEast,
            (false, true, false, true) => PipeType::SouthWest,
            (false, false, true, true) => PipeType::EastWest,
            c => panic!("invalid combination: {:?}", c),
        }
    }

    fn connected(&self, vec: Vec2, direction: Direction) -> bool {
        let destination = vec + direction.into();
        self.pipes
            .get(&destination)
            .map(|pipe| pipe.connects(&direction))
            .unwrap_or(false)
    }

    fn start(&self) -> (&Vec2, &PipeType) {
        self.pipes
            .iter()
            .find(|(_, &pipe)| pipe == PipeType::Start)
            .unwrap()
    }

    fn simplify(&mut self) {
        let mut loop_positions: HashSet<Vec2> = HashSet::new();
        let start = self.start();
        loop_positions.insert(*start.0);
        let (mut position, mut dir) = Direction::all()
            .into_iter()
            .find_map(|dir| {
                if self.connected(*start.0, dir) {
                    let destination = *start.0 + dir.into();
                    self.pipes
                        .get(&destination)
                        .map(|pipe| (destination, pipe.next(&dir).unwrap()))
                } else {
                    None
                }
            })
            .unwrap();
        loop_positions.insert(position);
        loop {
            let destination = position + dir.into();
            loop_positions.insert(destination);
            let pipe = self.pipes.get(&destination).unwrap();
            if let Some(new_dir) = pipe.next(&dir) {
                position = destination;
                dir = new_dir;
            } else {
                break;
            }
        }
        let new_pipes = self
            .pipes
            .iter()
            .filter(|(vec, _)| loop_positions.contains(vec))
            .map(|(vec, pipe)| (vec.clone(), pipe.clone()))
            .collect();
        self.pipes = new_pipes;
    }
}

impl FromStr for Pipes {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pipes = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    match c {
                        '|' => Some(PipeType::NorthSouth),
                        '-' => Some(PipeType::EastWest),
                        'L' => Some(PipeType::NorthEast),
                        'J' => Some(PipeType::NorthWest),
                        '7' => Some(PipeType::SouthWest),
                        'F' => Some(PipeType::SouthEast),
                        'S' => Some(PipeType::Start),
                        _ => None,
                    }
                    .map(|pipe| (Vec2::new(x as i32, y as i32), pipe))
                })
            })
            .collect();
        Ok(Pipes { pipes })
    }
}

#[cfg(test)]
mod tests {
    use super::Pipes;
    use crate::{pipe_type::PipeType, vec2::Vec2};

    const EXAMPLE1: &str = include_str!("../assets/example1.txt");
    const EXAMPLE2: &str = include_str!("../assets/example2.txt");
    const EXAMPLE3: &str = include_str!("../assets/example3.txt");
    const EXAMPLE4: &str = include_str!("../assets/example4.txt");
    const EXAMPLE5: &str = include_str!("../assets/example5.txt");

    #[test]
    fn example1() {
        let pipes: Pipes = EXAMPLE1.parse().unwrap();
        assert_eq!(pipes.farthest_distance(), 4);
    }

    #[test]
    fn example2() {
        let pipes: Pipes = EXAMPLE2.parse().unwrap();
        assert_eq!(pipes.farthest_distance(), 8);
    }

    macro_rules! assert_pipe {
        ($pipes:expr, $x:literal, $y:literal, $t:pat) => {
            assert!(
                matches!($pipes.pipes.get(&Vec2::new($x, $y)).unwrap(), $t),
                "unexpected type at {} {}",
                $x,
                $y
            );
        };
    }

    #[test]
    fn parse_example1() {
        let pipes: Pipes = EXAMPLE1.parse().unwrap();
        assert_pipe!(pipes, 1, 0, PipeType::NorthEast);
        assert_pipe!(pipes, 2, 0, PipeType::NorthSouth);
        assert_pipe!(pipes, 3, 0, PipeType::SouthEast);
        assert_pipe!(pipes, 4, 0, PipeType::SouthWest);
        assert_pipe!(pipes, 0, 1, PipeType::SouthWest);
        assert_pipe!(pipes, 1, 1, PipeType::Start);
    }

    #[test]
    fn simplify_example1() {
        let mut pipes: Pipes = EXAMPLE1.parse().unwrap();
        pipes.simplify();
        assert_eq!(pipes.pipes.len(), 8);
        assert_pipe!(pipes, 1, 1, PipeType::Start);
        assert_pipe!(pipes, 2, 1, PipeType::EastWest);
        assert_pipe!(pipes, 3, 1, PipeType::SouthWest);
        assert_pipe!(pipes, 1, 2, PipeType::NorthSouth);
        assert_pipe!(pipes, 3, 2, PipeType::NorthSouth);
        assert_pipe!(pipes, 1, 3, PipeType::NorthEast);
        assert_pipe!(pipes, 2, 3, PipeType::EastWest);
        assert_pipe!(pipes, 3, 3, PipeType::NorthWest);
    }

    #[test]
    fn inner_example1() {
        let pipes: Pipes = EXAMPLE1.parse().unwrap();
        assert_eq!(pipes.inner_tiles(), 1);
    }

    #[test]
    fn inner_example2() {
        let pipes: Pipes = EXAMPLE2.parse().unwrap();
        assert_eq!(pipes.inner_tiles(), 1);
    }

    #[test]
    fn inner_example3() {
        let pipes: Pipes = EXAMPLE3.parse().unwrap();
        assert_eq!(pipes.inner_tiles(), 4);
    }

    #[test]
    fn inner_example4() {
        let pipes: Pipes = EXAMPLE4.parse().unwrap();
        assert_eq!(pipes.inner_tiles(), 8);
    }

    #[test]
    fn inner_example5() {
        let pipes: Pipes = EXAMPLE5.parse().unwrap();
        assert_eq!(pipes.inner_tiles(), 10);
    }
}
