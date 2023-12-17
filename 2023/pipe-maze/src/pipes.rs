use crate::{direction::Direction, pipe_type::PipeType, vec2::Vec2};
use std::{collections::HashMap, str::FromStr};

pub struct Pipes {
    pipes: HashMap<Vec2, PipeType>,
}

impl Pipes {
    pub fn farthest_distance(mut self) -> u32 {
        self.simplify();
        let start = self
            .pipes
            .iter()
            .find(|(_, &pipe)| pipe == PipeType::Start)
            .unwrap();
        let (mut position, mut dir) = Direction::all()
            .into_iter()
            .find_map(|dir| {
                let destination = start.0.clone() + dir.into();
                self.pipes
                    .get(&destination)
                    .map(|pipe| (destination, pipe.next(&dir).unwrap()))
            })
            .unwrap();
        let mut count = 1;
        loop {
            let destination = position + dir.into();
            let pipe = self.pipes.get(&destination).unwrap();
            if let Some(new_dir) = pipe.next(&dir) {
                position = destination;
                dir = new_dir;
            } else {
                break;
            }
            count += 1;
        }
        (count + 1) / 2
    }

    fn simplify(&mut self) {
        loop {
            let new_pipes: HashMap<Vec2, PipeType> = self
                .pipes
                .iter()
                .filter(|(&vec, pipe)| {
                    pipe.connections()
                        .map(|connections| {
                            connections.into_iter().all(|dir| {
                                let target = vec.clone() + dir.into();
                                self.pipes
                                    .get(&target)
                                    .map(|target| target.connects(&dir))
                                    .unwrap_or(false)
                            })
                        })
                        .unwrap_or(true)
                })
                .map(|(vec, pipe)| (vec.clone(), pipe.clone()))
                .collect();
            if new_pipes.len() < self.pipes.len() {
                self.pipes = new_pipes;
            } else {
                break;
            }
        }
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
}
