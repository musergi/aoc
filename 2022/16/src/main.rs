use std::{
    collections::{BTreeSet, HashMap},
    convert::Infallible,
    fs,
    str::FromStr,
};

fn main() {
    let s = fs::read_to_string("assets/input.txt").expect("read failed");
    let p: Problem = s.parse().expect("parse failed");
    println!("Part 1: {}", p.part1());
    println!("Part 2: {}", p.part2());
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Identifier([char; 2]);

impl FromStr for Identifier {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.chars();
        let res = Identifier([
            it.next().expect("missing first char"),
            it.next().expect("missing second char"),
        ]);
        assert!(it.next().is_none());
        Ok(res)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Line {
    source: Identifier,
    flow: u32,
    destinations: Vec<Identifier>,
}

impl FromStr for Line {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split("; ");
        let (source, flow) = {
            let mut it = it
                .next()
                .unwrap()
                .strip_prefix("Valve ")
                .unwrap()
                .split(" has flow rate=");
            (
                it.next().unwrap().parse::<Identifier>().unwrap(),
                it.next().unwrap().parse::<u32>().unwrap(),
            )
        };
        let s = it.next().unwrap();
        let dest = s
            .strip_prefix("tunnels lead to valves ")
            .or_else(|| s.strip_prefix("tunnel leads to valve "))
            .unwrap()
            .split(", ")
            .into_iter()
            .map(|id| id.parse::<Identifier>().unwrap())
            .collect();
        Ok(Line {
            source,
            flow,
            destinations: dest,
        })
    }
}

struct Problem {
    nodes: Vec<(Identifier, u32)>,
    distances: Vec<Vec<u32>>,
}

impl Problem {
    fn part1(&self) -> u32 {
        let start = self
            .nodes
            .iter()
            .position(|node| node.0 == Identifier(['A', 'A']))
            .unwrap();
        let initial = Path::new(start);
        let mut open = Vec::new();
        open.push(initial.clone());

        let mut best = initial;
        while let Some(path) = open.pop() {
            if path.flow() > best.flow() {
                best = path.clone();
            }
            for (idx, (_, flow)) in self.nodes.iter().enumerate() {
                if !path.contains(idx) {
                    let new_time = path.time() + self.distances[path.end()][idx] + 1;
                    if new_time <= 30 {
                        let new_flow = path.flow() + (30 - new_time) * flow;
                        let new_path = path.extend(idx, new_time, new_flow);
                        open.push(new_path);
                    }
                }
            }
        }
        best.flow()
    }

    fn part2(&self) -> u32 {
        let start = self
            .nodes
            .iter()
            .position(|node| node.0 == Identifier(['A', 'A']))
            .unwrap();
        let initial = Path::new(start);
        let mut open = Vec::new();
        open.push(initial.clone());

        let mut bests = HashMap::new();
        while let Some(path) = open.pop() {
            let current_best = bests.entry(path.key(start)).or_insert_with(|| path.flow());
            if path.flow() > *current_best {
                *current_best = path.flow();
            }
            for (idx, (_, flow)) in self.nodes.iter().enumerate() {
                if !path.contains(idx) {
                    let new_time = path.time() + self.distances[path.end()][idx] + 1;
                    if new_time <= 26 {
                        let new_flow = path.flow() + (26 - new_time) * flow;
                        let new_path = path.extend(idx, new_time, new_flow);
                        open.push(new_path);
                    }
                }
            }
        }
        bests
            .iter()
            .map(|(person, person_flow)| {
                bests
                    .iter()
                    .filter(|(elefant, _)| person.is_disjoint(elefant))
                    .map(|(_, elefant_flow)| person_flow + elefant_flow)
                    .max()
                    .unwrap_or(0)
            })
            .max()
            .unwrap()
    }
}

#[derive(Clone)]
struct Path {
    nodes: Vec<PathElement>,
}

impl Path {
    fn new(node: usize) -> Self {
        let nodes = vec![PathElement::new(node)];
        Path { nodes }
    }

    fn key(&self, start: usize) -> BTreeSet<usize> {
        self.nodes
            .iter()
            .map(|p| p.node)
            .filter(|&n| n != start)
            .collect()
    }

    fn contains(&self, node: usize) -> bool {
        self.nodes.iter().any(|e| e.node == node)
    }

    fn end(&self) -> usize {
        self.nodes.last().unwrap().node
    }

    fn time(&self) -> u32 {
        self.nodes.last().unwrap().arrive_time
    }

    fn flow(&self) -> u32 {
        self.nodes.last().unwrap().acc_flow
    }

    fn extend(&self, node: usize, arrive_time: u32, acc_flow: u32) -> Path {
        let mut nodes = self.nodes.clone();
        nodes.push(PathElement {
            node,
            arrive_time,
            acc_flow,
        });
        Path { nodes }
    }
}

#[derive(Debug, Clone)]
struct PathElement {
    node: usize,
    arrive_time: u32,
    acc_flow: u32,
}

impl PathElement {
    fn new(node: usize) -> Self {
        Self {
            node,
            arrive_time: 0,
            acc_flow: 0,
        }
    }
}

impl FromStr for Problem {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .lines()
            .map(|line| line.parse::<Line>().unwrap())
            .collect::<Vec<_>>();
        let nodes = lines
            .iter()
            .map(|line| (line.source.clone(), line.flow))
            .collect::<Vec<_>>();
        let mut distances = lines
            .iter()
            .map(|line| {
                nodes
                    .iter()
                    .map(|node| {
                        if line.destinations.contains(&node.0) {
                            1
                        } else {
                            u32::MAX
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        floyd_warshall(&mut distances);
        let keep = nodes
            .iter()
            .enumerate()
            .filter(|(_, n)| n.1 != 0 || n.0 == Identifier(['A', 'A']))
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        let nodes = nodes
            .into_iter()
            .enumerate()
            .filter(|(i, _)| keep.contains(i))
            .map(|(_, v)| v)
            .collect();
        let distances = distances
            .into_iter()
            .enumerate()
            .filter(|(i, _)| keep.contains(i))
            .map(|(_, v)| {
                v.into_iter()
                    .enumerate()
                    .filter(|(i, _)| keep.contains(i))
                    .map(|(_, v)| v)
                    .collect()
            })
            .collect();
        Ok(Problem { nodes, distances })
    }
}

pub fn floyd_warshall(adj_mat: &mut Vec<Vec<u32>>) {
    let n = adj_mat.len();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                adj_mat[i][j] =
                    std::cmp::min(adj_mat[i][j], adj_mat[i][k].saturating_add(adj_mat[k][j]));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intentifier_parsing() {
        let identifier: Identifier = "AA".parse().expect("parsing failed");
        assert_eq!(Identifier(['A', 'A']), identifier)
    }

    #[test]
    fn line_parsing_multivalve() {
        let line: Line = "Valve AA has flow rate=0; tunnels lead to valves DD, II"
            .parse()
            .expect("parsing failed");
        assert_eq!(
            line,
            Line {
                source: "AA".parse().unwrap(),
                flow: 0,
                destinations: vec!["DD".parse().unwrap(), "II".parse().unwrap()]
            }
        )
    }

    #[test]
    fn line_parsing_monovalve() {
        let line: Line = "Valve JJ has flow rate=21; tunnel leads to valve II"
            .parse()
            .expect("parsing failed");
        assert_eq!(
            line,
            Line {
                source: "JJ".parse().unwrap(),
                flow: 21,
                destinations: vec!["II".parse().unwrap()]
            }
        )
    }

    #[test]
    fn example_parse() {
        let s = include_str!("../assets/example.txt");
        let p: Problem = s.parse().unwrap();
        assert_eq!(p.nodes.len(), 7);
        assert!(p.nodes.contains(&("AA".parse().unwrap(), 0)));
        assert!(p.nodes.contains(&("BB".parse().unwrap(), 13)));
        assert!(p.nodes.contains(&("CC".parse().unwrap(), 2)));
        assert!(p.nodes.contains(&("DD".parse().unwrap(), 20)));
        assert!(p.nodes.contains(&("EE".parse().unwrap(), 3)));
        assert!(p.nodes.contains(&("HH".parse().unwrap(), 22)));
        assert!(p.nodes.contains(&("JJ".parse().unwrap(), 21)));
    }

    #[test]
    fn example_part1() {
        let s = include_str!("../assets/example.txt");
        let p: Problem = s.parse().unwrap();
        assert_eq!(p.part1(), 1651);
    }

    #[test]
    fn example_part2() {
        let s = include_str!("../assets/example.txt");
        let p: Problem = s.parse().unwrap();
        assert_eq!(p.part2(), 1707);
    }
}
