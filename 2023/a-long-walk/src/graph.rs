use crate::position::Position;
use std::collections::BTreeMap;

pub struct Graph {
    pub nodes: Vec<Position>,
    pub edges: BTreeMap<usize, BTreeMap<usize, usize>>,
}

pub struct Edge {
    pub destination: usize,
    pub distance: usize,
}

impl Graph {
    pub fn edges(&self, node: usize) -> impl Iterator<Item = Edge> {
        self.edges
            .get(&node)
            .unwrap()
            .clone()
            .into_iter()
            .map(|(destination, distance)| Edge {
                destination,
                distance,
            })
    }
}
