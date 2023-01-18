use graph::{breadth_first_search, floyd_warshall};
use std::collections::HashMap;
use valve::{Valve, ValveLineInfo};

pub mod graph;
pub mod valve;

#[derive(Debug)]
pub struct Volcano {
    nodes: Vec<Valve>,
    edges: HashMap<usize, Vec<usize>>,
}

impl From<Vec<ValveLineInfo>> for Volcano {
    fn from(infos: Vec<ValveLineInfo>) -> Self {
        let nodes = infos.iter().map(|i| Valve::from(i)).collect::<Vec<_>>();
        let edges = infos
            .iter()
            .map(|i| {
                (
                    nodes
                        .iter()
                        .position(|valve| valve.id == i.subject)
                        .unwrap(),
                    i.destinations
                        .iter()
                        .map(|dest_id| nodes.iter().position(|valve| &valve.id == dest_id).unwrap())
                        .collect::<Vec<_>>(),
                )
            })
            .collect();
        Volcano { nodes, edges }
    }
}

impl Volcano {
    pub fn get_max_flow(&self, minutes: u32) -> u32 {
        let mut distances = (0..self.nodes.len())
            .map(|node| {
                (0..self.nodes.len())
                    .map(|other| {
                        match self
                            .edges
                            .get(&node)
                            .map(|connections| connections.contains(&other))
                        {
                            Some(true) => 1,
                            _ => (self.nodes.len() + 1) as u32,
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        floyd_warshall(&mut distances);
        let start = self
            .nodes
            .iter()
            .position(|valve| valve.id == "AA".parse().unwrap())
            .unwrap();
        breadth_first_search(
            start,
            &|path| self.get_next_nodes(&distances, minutes, path),
            &|path| self.get_path_flow(&distances, minutes, path),
        )
        .unwrap()
        .metric
    }

    fn get_next_nodes(
        &self,
        distances: &Vec<Vec<u32>>,
        minutes: u32,
        path: &[usize],
    ) -> Vec<usize> {
        let path_time = to_time(distances, path);
        self.nodes
            .iter()
            .enumerate()
            .filter_map(|(idx, valve)| match valve.flow {
                0 => None,
                _ => Some(idx),
            })
            .filter(|idx| !path.contains(idx))
            .filter(|dest| path_time + new_time(distances, path, *dest) < minutes)
            .collect()
    }

    fn get_path_flow(&self, distances: &Vec<Vec<u32>>, minutes: u32, path: &[usize]) -> u32 {
        path.windows(2)
            .map(|l| (l.get(0).unwrap(), l.get(1).unwrap()))
            .scan(0, |state, (start, end)| {
                let distance = distances.get(*start).unwrap().get(*end).unwrap();
                *state += distance + 1;
                minutes
                    .checked_sub(*state)
                    .map(|time| time * self.nodes.get(*end).unwrap().flow)
            })
            .sum()
    }
}

fn new_time(distances: &Vec<Vec<u32>>, path: &[usize], new_node: usize) -> u32 {
    distances
        .get(*path.last().unwrap())
        .unwrap()
        .get(new_node)
        .unwrap()
        + 1
}

fn to_time(distances: &Vec<Vec<u32>>, path: &[usize]) -> u32 {
    path.windows(2)
        .map(|elements| (elements.get(0).unwrap(), elements.get(1).unwrap()))
        .map(|(&start, &end)| distances.get(start).unwrap().get(end).unwrap())
        .map(|dist| dist + 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::valve::*;

    #[test]
    fn test_volcano_creation() {
        let line_infos = vec![
            ValveLineInfo {
                subject: "AA".parse().unwrap(),
                flow: 1,
                destinations: vec!["BB".parse().unwrap(), "CC".parse().unwrap()],
            },
            ValveLineInfo {
                subject: "BB".parse().unwrap(),
                flow: 2,
                destinations: vec!["AA".parse().unwrap()],
            },
            ValveLineInfo {
                subject: "CC".parse().unwrap(),
                flow: 3,
                destinations: vec!["AA".parse().unwrap()],
            },
        ];
        let volcano = Volcano::from(line_infos);
        assert!(volcano.nodes.iter().any(|n| n.id == "AA".parse().unwrap()));
        assert!(volcano.nodes.iter().any(|n| n.id == "BB".parse().unwrap()));
        assert!(volcano.nodes.iter().any(|n| n.id == "CC".parse().unwrap()));
        assert_eq!(get_outgoing_edges(&volcano, "AA".parse().unwrap()), 2);
        assert_eq!(get_outgoing_edges(&volcano, "BB".parse().unwrap()), 1);
        assert_eq!(get_outgoing_edges(&volcano, "CC".parse().unwrap()), 1);
    }

    fn get_outgoing_edges(volcano: &Volcano, id: ValveId) -> usize {
        volcano
            .edges
            .get(&volcano.nodes.iter().position(|n| n.id == id).unwrap())
            .unwrap()
            .len()
    }

    #[test]
    fn test_flow_calculation() {
        let line_infos = vec![
            ValveLineInfo {
                subject: "AA".parse().unwrap(),
                flow: 0,
                destinations: vec!["BB".parse().unwrap(), "CC".parse().unwrap()],
            },
            ValveLineInfo {
                subject: "BB".parse().unwrap(),
                flow: 2,
                destinations: vec!["AA".parse().unwrap()],
            },
            ValveLineInfo {
                subject: "CC".parse().unwrap(),
                flow: 3,
                destinations: vec!["AA".parse().unwrap()],
            },
        ];
        let volcano = Volcano::from(line_infos);
        let distances = vec![vec![10, 1, 1], vec![1, 10, 2], vec![1, 2, 10]];
        let path = vec![0, 2, 1];
        assert_eq!(volcano.get_path_flow(&distances, 3, &path), 3 * 1);
        assert_eq!(volcano.get_path_flow(&distances, 4, &path), 3 * 2);
        assert_eq!(volcano.get_path_flow(&distances, 5, &path), 3 * 3);
        assert_eq!(
            volcano.get_path_flow(&distances, 6, &path),
            3 * 3 + (3 + 2) * 1
        );
        assert_eq!(
            volcano.get_path_flow(&distances, 7, &path),
            3 * 3 + (3 + 2) * 2
        );
    }
}
