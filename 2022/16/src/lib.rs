use std::{collections::HashMap, iter, str::FromStr, vec};

mod valve;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct ValveId([char; 2]);

impl FromStr for ValveId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.chars();
        let c1 = it.next().unwrap();
        let c2 = it.next().unwrap();
        Ok(ValveId([c1, c2]))
    }
}

struct LineInfo {
    source_id: ValveId,
    flow_rate: u32,
    destination_ids: Vec<ValveId>,
}

impl FromStr for LineInfo {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = split_by(s, "; ").unwrap();
        let fs = split_by(first.strip_prefix("Valve ").unwrap(), " has flow rate=").unwrap();
        let source_id = fs.0.parse::<ValveId>().unwrap();
        let flow_rate = fs.1.parse::<u32>().unwrap();
        let destination_ids = second
            .strip_prefix("tunnels lead to valves ")
            .unwrap_or_else(|| second.strip_prefix("tunnel leads to valve ").unwrap())
            .split(", ")
            .map(|s| s.parse::<ValveId>().unwrap())
            .collect::<Vec<_>>();
        Ok(LineInfo {
            source_id,
            flow_rate,
            destination_ids,
        })
    }
}

fn split_by<'a>(s: &'a str, sep: &str) -> Option<(&'a str, &'a str)> {
    let mut it = s.split(sep);
    let f = it.next()?;
    let s = it.next()?;
    Some((f, s))
}

struct Valve {
    id: ValveId,
    flow: u32,
}

pub struct Graph {
    nodes: Vec<Valve>,
    edges: HashMap<usize, Vec<usize>>,
}

impl Graph {
    pub fn get_solution(&self, length: usize) -> Vec<Step> {
        let mut best_solution = (None, None);

        let mut options = vec![self.available_steps(self.get_start(), iter::empty())];
        let mut path = Vec::new();
        while !options.is_empty() {
            path.push(options.last_mut().unwrap().pop().unwrap());
            if path.len() == length {
                let new_score = self.get_score(path.iter(), length);
                if best_solution.0.is_none() || new_score > best_solution.0.unwrap() {
                    best_solution = (Some(new_score), Some(path.clone()));
                }
                path.pop().unwrap();
                while !options.is_empty() && options.last().unwrap().is_empty() {
                    options.pop().unwrap();
                    path.pop();
                }
            } else {
                options.push(self.available_steps(path.last().unwrap().destination(), path.iter()));
            }
        }

        best_solution.1.unwrap()
    }

    fn get_start(&self) -> usize {
        let start_id = "AA".parse().unwrap();
        self.nodes.iter().position(|n| n.id == start_id).unwrap()
    }

    pub fn get_score<'a, I>(&self, steps: I, length: usize) -> u32
    where
        I: Iterator<Item = &'a Step>,
    {
        steps
            .enumerate()
            .filter_map(|(idx, s)| match s {
                Step::Move(_) => None,
                Step::Open(valve) => Some((idx, valve)),
            })
            .map(|(minute, &node)| {
                self.nodes.get(node).unwrap().flow * (length - minute - 1) as u32
            })
            .sum()
    }

    fn available_steps<'a, I>(&self, node: usize, mut done: I) -> Vec<Step>
    where
        I: Iterator<Item = &'a Step>,
    {
        let connections = self.edges.get(&node).unwrap();
        let mut steps = connections
            .iter()
            .map(|c| Step::Move(*c))
            .collect::<Vec<_>>();
        let already_open = done.any(|s| &Step::Open(node) == s);
        if !already_open {
            steps.push(Step::Open(node));
        }
        steps
    }
}

impl FromStr for Graph {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes = Vec::new();
        let mut connections = HashMap::<ValveId, Vec<ValveId>>::new();
        for line in s.lines().map(|l| l.parse::<LineInfo>().unwrap()) {
            nodes.push(Valve {
                id: line.source_id.clone(),
                flow: line.flow_rate,
            });
            connections.insert(line.source_id, line.destination_ids);
        }
        let edges = connections
            .into_iter()
            .map(|(k, v)| {
                (
                    nodes.iter().position(|n| n.id == k).unwrap(),
                    v.into_iter()
                        .map(|d| nodes.iter().position(|n| n.id == d).unwrap())
                        .collect(),
                )
            })
            .collect();
        Ok(Graph { nodes, edges })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Step {
    Move(usize),
    Open(usize),
}

impl Step {
    fn destination(&self) -> usize {
        match self {
            Step::Move(d) => *d,
            Step::Open(d) => *d,
        }
    }
}
