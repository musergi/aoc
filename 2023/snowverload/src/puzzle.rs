use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;
use std::usize;

pub struct Puzzle {
    components: Vec<String>,
    connections: HashMap<usize, Vec<usize>>,
}

impl Puzzle {
    pub fn solve(self) -> impl Display {
        let edges: Vec<_> = self
            .connections
            .iter()
            .flat_map(|(source, destinations)| {
                destinations
                    .iter()
                    .map(|destination| Edge(*source, *destination))
            })
            .collect();
        find_partition(&edges, self.components.len())
    }
}

fn find_partition(edges: &[Edge], nodes: usize) -> usize {
    let mut rng = Random::default();
    std::iter::repeat_with(|| edges.to_vec())
        .find_map(|edges| try_partition(&mut rng, edges, nodes))
        .unwrap()
}

fn try_partition(rng: &mut Random, mut edges: Vec<Edge>, nodes: usize) -> Option<usize> {
    let mut nodes: Vec<_> = std::iter::repeat(1).take(nodes).collect();
    while nodes.iter().filter(|&weight| *weight != 0).count() > 2 {
        let removed_edge = rng.sample(&edges);
        let (left, right) = removed_edge.unpack();
        *nodes.get_mut(left).unwrap() += *nodes.get(right).unwrap();
        *nodes.get_mut(right).unwrap() = 0;
        edges.retain(|edge| *edge != removed_edge);
        for edge in edges.iter_mut() {
            edge.swap_node(right, left);
        }
    }
    if edges.len() == 3 {
        Some(
            nodes
                .iter()
                .filter(|&weight| *weight != 0)
                .fold(1, |a, b| a * b),
        )
    } else {
        None
    }
}

impl FromStr for Puzzle {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut components = Vec::new();
        let mut connections = HashMap::new();
        for line in s.lines() {
            let (left, rights) = line.split_once(": ").ok_or("colon separator not found")?;
            let left_idx =
                if let Some(idx) = components.iter().position(|component| component == left) {
                    idx
                } else {
                    components.push(left.to_string());
                    components.len() - 1
                };
            let mut right_indices = Vec::new();
            for right in rights.split_whitespace() {
                let right_idx =
                    if let Some(idx) = components.iter().position(|component| component == right) {
                        idx
                    } else {
                        components.push(right.to_string());
                        components.len() - 1
                    };
                right_indices.push(right_idx)
            }
            connections.insert(left_idx, right_indices);
        }
        Ok(Puzzle {
            components,
            connections,
        })
    }
}

#[derive(Clone, Copy)]
struct Edge(usize, usize);

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}

impl Edge {
    fn swap_node(&mut self, source: usize, destination: usize) {
        if self.0 == source {
            self.0 = destination
        } else if self.1 == source {
            self.1 = destination
        }
    }

    fn unpack(self) -> (usize, usize) {
        (self.0, self.1)
    }
}

struct Random {
    state: u32,
}

impl Default for Random {
    fn default() -> Self {
        Random { state: 123456789 }
    }
}

impl Random {
    fn next(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
        self.state
    }

    fn sample<T>(&mut self, l: &[T]) -> T
    where
        T: Copy,
    {
        let length = u32::try_from(l.len()).unwrap();
        let idx = usize::try_from(self.next_in_range(length)).unwrap();
        *l.get(idx).unwrap()
    }

    fn next_in_range(&mut self, range: u32) -> u32 {
        let mut modulo = None;
        loop {
            let new = modulo.map(|v| v >> 1).unwrap_or(1 << 30);
            if new < range {
                break;
            }
            modulo = Some(new);
        }
        loop {
            let value = self.next();
            let value = modulo.map(|modulo| value % modulo).unwrap_or(value);
            if value < range {
                return value;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let string = include_str!("../assets/example.txt");
        let puzzle: Puzzle = string.parse().unwrap();
        assert_eq!(puzzle.solve().to_string(), "54");
    }
}
