use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Id(char, char);

#[derive(Debug, Clone)]
struct Valve {
    id: Id,
    flow: u8,
}

impl Valve {
    fn new(s: &str, flow: u8) -> Valve {
        let mut v = Valve::from_id(s);
        v.flow = flow;
        v
    }
    fn from_id(s: &str) -> Valve {
        assert!(s.len() == 2);
        let mut it = s.chars().into_iter();
        let id = Id(it.next().unwrap(), it.next().unwrap());
        Valve { id: id, flow: 0 }
    }
}

#[derive(Debug, Default)]
struct Graph {
    nodes: Vec<Valve>,
    edges: HashMap<usize, HashSet<usize>>,
}

enum Move {
    Open,
    Move(usize),
}

struct Path {
    moves: Vec<Move>,
    presure: u32,
    score: i32,
}

fn main() {
    let mut g = Graph::default();
    let mut edges = Vec::new();
    for line in fs::read_to_string("assets/example.txt")
        .expect("Parsed file")
        .lines()
    {
        let line = line.strip_prefix("Valve ").expect("Prefix");
        let mut it = line.split(" has flow rate=");
        let id_str = it.next().expect("Valve");
        let line = it.next().expect("Not valve");
        println!("{}", line);
        let mut it = line.split(";");
        let flow = it.next().expect("Flow").parse::<u8>().expect("Parsed flow");
        let valve = Valve::new(id_str, flow);
        let line = it.next().expect("Passages list");
        let line = line
            .strip_prefix(" tunnels lead to valves ")
            .or_else(|| line.strip_prefix(" tunnel leads to valve "))
            .expect("Path prefix");
        for dest in line.split(", ") {
            edges.push((valve.clone(), Valve::from_id(dest)))
        }
        g.nodes.push(valve);
    }
    for edge in edges {
        let s = g
            .nodes
            .iter()
            .position(|v| v.id == edge.0.id)
            .expect("Start to exist");
        let e = g
            .nodes
            .iter()
            .position(|v| v.id == edge.1.id)
            .expect("End to exist");
        g.edges.entry(s).or_default().insert(e);
    }
    println!("{:?}", g)
}
