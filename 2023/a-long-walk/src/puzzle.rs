use crate::{generator::PathGenerator, graph::Graph, ice::Ice, path::Path, position::Position};
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

pub struct Puzzle {
    open: HashSet<Position>,
    ice: HashMap<Position, Ice>,
    start: Position,
    end: Position,
}

impl Puzzle {
    pub fn part1(self) -> usize {
        let graph = self.graph(|puzzle, path, position| puzzle.part1_filter(path, position));
        GraphPathGenerator { graph: &graph }
            .paths()
            .into_iter()
            .map(|path| {
                path.distance(|a, b| {
                    graph
                        .edges(a)
                        .find(|edge| edge.destination == b)
                        .unwrap()
                        .distance
                })
            })
            .max()
            .unwrap_or(0)
    }

    pub fn part2(self) -> usize {
        let graph = self.graph(|puzzle, path, position| puzzle.part2_filter(path, position));
        GraphPathGenerator { graph: &graph }
            .paths()
            .into_iter()
            .map(|path| {
                path.distance(|a, b| {
                    graph
                        .edges(a)
                        .find(|edge| edge.destination == b)
                        .unwrap()
                        .distance
                })
            })
            .max()
            .unwrap_or(0)
    }

    pub fn graph<F>(&self, filter: F) -> Graph
    where
        F: Fn(&Puzzle, &Path<Position>, &Position) -> bool + Copy,
    {
        let mut nodes = vec![self.start.clone(), self.end.clone()];
        nodes.extend(
            self.open
                .iter()
                .chain(self.ice.keys())
                .cloned()
                .filter(|position| {
                    Ice::all()
                        .map(|ice| ice.delta())
                        .map(|delta| *position + delta)
                        .into_iter()
                        .filter(|position| self.valid(position))
                        .count()
                        > 2
                }),
        );
        let edges = nodes
            .iter()
            .enumerate()
            .map(|(source_idx, node)| {
                (
                    source_idx,
                    NodePaths {
                        node: *node,
                        filter,
                        puzzle: self,
                        nodes: &nodes,
                    }
                    .paths()
                    .into_iter()
                    .map(|path| {
                        (
                            nodes.iter().position(|node| node == &path.tail()).unwrap(),
                            path.len(),
                        )
                    })
                    .collect(),
                )
            })
            .collect();
        Graph { nodes, edges }
    }

    fn part1_filter(&self, path: &Path<Position>, position: &Position) -> bool {
        self.valid(position) && !path.contains(&position) && !self.is_climbing_ice(path, position)
    }

    fn part2_filter(&self, path: &Path<Position>, position: &Position) -> bool {
        self.valid(position) && !path.contains(&position)
    }

    fn valid(&self, position: &Position) -> bool {
        self.open.contains(position) || self.ice.contains_key(position)
    }

    fn is_climbing_ice(&self, path: &Path<Position>, position: &Position) -> bool {
        self.ice
            .get(position)
            .map(|ice| ice.delta())
            .map(|delta| delta != *position - path.tail())
            .unwrap_or(false)
    }
}

struct Part2PathGenerator {
    puzzle: Puzzle,
}

impl PathGenerator for Part2PathGenerator {
    type Item = Position;

    fn start(&self) -> Self::Item {
        self.puzzle.start
    }

    fn next(&self, path: &Path<Self::Item>) -> Vec<Self::Item> {
        Ice::all()
            .map(|ice| ice.delta())
            .map(|delta| path.tail() + delta)
            .into_iter()
            .filter(|new| self.puzzle.part2_filter(&path, new))
            .collect()
    }

    fn close(&self, next: &Self::Item) -> bool {
        next == &self.puzzle.end
    }
}

struct GraphPathGenerator<'a> {
    graph: &'a Graph,
}

impl PathGenerator for GraphPathGenerator<'_> {
    type Item = usize;

    fn start(&self) -> Self::Item {
        0
    }

    fn next(&self, path: &Path<Self::Item>) -> Vec<Self::Item> {
        self.graph
            .edges(path.tail())
            .map(|edge| edge.destination)
            .filter(|destination| !path.contains(destination))
            .collect()
    }

    fn close(&self, next: &Self::Item) -> bool {
        *next == 1
    }
}

trait PositionPathFilter {
    fn filter(&self, path: &Path<Position>, new: &Position) -> bool;
}

struct Part1<'a> {
    puzzle: &'a Puzzle,
}

impl<'a> PositionPathFilter for Part1<'a> {
    fn filter(&self, path: &Path<Position>, new: &Position) -> bool {
        self.puzzle.part1_filter(path, new)
    }
}

struct NodePaths<'a, F> {
    node: Position,
    filter: F,
    puzzle: &'a Puzzle,
    nodes: &'a [Position],
}

impl<F> PathGenerator for NodePaths<'_, F>
where
    F: Fn(&Puzzle, &Path<Position>, &Position) -> bool,
{
    type Item = Position;

    fn start(&self) -> Self::Item {
        self.node
    }

    fn next(&self, path: &Path<Self::Item>) -> Vec<Self::Item> {
        Ice::all()
            .map(|ice| ice.delta())
            .map(|delta| path.tail() + delta)
            .into_iter()
            .filter(|new| (self.filter)(self.puzzle, &path, new))
            .collect()
    }

    fn close(&self, next: &Self::Item) -> bool {
        self.nodes.contains(next)
    }
}

impl FromStr for Puzzle {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut open = HashSet::new();
        let mut ice = HashMap::new();
        for (row, line) in s.lines().enumerate() {
            for (column, char) in line.chars().enumerate() {
                if let Some(new) = Ice::from_char(char) {
                    ice.insert(
                        Position::new(
                            row.try_into().map_err(|_| "row overflow")?,
                            column.try_into().map_err(|_| "column overflow")?,
                        ),
                        new,
                    );
                } else if char == '.' {
                    open.insert(Position::new(
                        row.try_into().map_err(|_| "row overflow")?,
                        column.try_into().map_err(|_| "column overflow")?,
                    ));
                }
            }
        }
        let start = open
            .iter()
            .min_by_key(|key| key.row())
            .ok_or("start not found")?
            .clone();
        let end = open
            .iter()
            .max_by_key(|key| key.row())
            .ok_or("start not found")?
            .clone();
        Ok(Puzzle {
            open,
            ice,
            start,
            end,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../assets/example.txt");

    #[test]
    fn parse_open() {
        let s = "#.##\n#..#\n#..#\n##.#";
        let puzzle: Puzzle = s.parse().unwrap();
        assert_eq!(puzzle.open.len(), 6);
        assert!(puzzle.open.contains(&Position::new(0, 1)));
        assert!(puzzle.open.contains(&Position::new(1, 1)));
        assert!(puzzle.open.contains(&Position::new(1, 2)));
        assert!(puzzle.open.contains(&Position::new(2, 1)));
        assert!(puzzle.open.contains(&Position::new(2, 2)));
        assert!(puzzle.open.contains(&Position::new(3, 2)));
    }

    #[test]
    fn parse_start() {
        let s = "#.##\n#..#\n#..#\n##.#";
        let puzzle: Puzzle = s.parse().unwrap();
        assert_eq!(puzzle.start, Position::new(0, 1));
    }

    #[test]
    fn parse_end() {
        let s = "#.##\n#..#\n#..#\n##.#";
        let puzzle: Puzzle = s.parse().unwrap();
        assert_eq!(puzzle.end, Position::new(3, 2));
    }

    #[test]
    fn parse_ice() {
        let s = "#.##\n#><#\n#v^#\n##.#";
        let puzzle: Puzzle = s.parse().unwrap();
        assert_eq!(puzzle.ice.len(), 4);
        assert_eq!(puzzle.ice.get(&Position::new(1, 1)).unwrap(), &Ice::East);
        assert_eq!(puzzle.ice.get(&Position::new(1, 2)).unwrap(), &Ice::West);
        assert_eq!(puzzle.ice.get(&Position::new(2, 1)).unwrap(), &Ice::South);
        assert_eq!(puzzle.ice.get(&Position::new(2, 2)).unwrap(), &Ice::North);
    }

    #[test]
    fn single_solution_part1() {
        let s = "#.##\n#..#\n##.#\n##.#";
        let puzzle: Puzzle = s.parse().unwrap();
        assert_eq!(puzzle.part1(), 4);
    }

    #[test]
    fn example_part1() {
        let puzzle: Puzzle = EXAMPLE.parse().unwrap();
        assert_eq!(puzzle.part1(), 94);
    }

    #[test]
    fn example_part2() {
        let puzzle: Puzzle = EXAMPLE.parse().unwrap();
        assert_eq!(puzzle.part2(), 154);
    }
}
