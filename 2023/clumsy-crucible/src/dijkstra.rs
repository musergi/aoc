use std::collections::{BTreeMap, BinaryHeap};

pub fn dijkstra<T, F1, F2>(adjancency_function: F1, start: T, is_done: F2) -> Option<usize>
where
    F1: Fn(&T) -> Vec<State<T>>,
    F2: Fn(&T) -> bool,
    T: Ord + Clone,
{
    let mut best_distances = BTreeMap::new();
    let mut heap = BinaryHeap::new();
    best_distances.insert(start.clone(), 0);
    heap.push(State {
        cost: 0,
        node: start,
    });

    while let Some(State { cost, node }) = heap.pop() {
        if is_done(&node) {
            return Some(cost);
        }

        if best_distances
            .get(&node)
            .map(|&old_cost| old_cost < cost)
            .unwrap_or(false)
        {
            continue;
        }

        for edge in adjancency_function(&node) {
            let next = State {
                cost: cost + edge.cost,
                node: edge.node,
            };

            if best_distances
                .get(&next.node)
                .map(|&old_cost| old_cost > next.cost)
                .unwrap_or(true)
            {
                best_distances.insert(next.node.clone(), next.cost);
                heap.push(next);
            }
        }
    }
    None
}

#[derive(Clone, PartialEq, Eq)]
pub struct State<T> {
    pub cost: usize,
    pub node: T,
}

impl<T> Ord for State<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl<T> PartialOrd for State<T>
where
    T: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use crate::dijkstra::{dijkstra, State};

    #[test]
    fn simple_graph() {
        let graph = vec![
            vec![State { node: 2, cost: 10 }, State { node: 1, cost: 1 }],
            vec![State { node: 3, cost: 2 }],
            vec![
                State { node: 1, cost: 1 },
                State { node: 3, cost: 3 },
                State { node: 4, cost: 1 },
            ],
            vec![State { node: 0, cost: 7 }, State { node: 4, cost: 2 }],
            vec![],
        ];

        let func = |&index: &usize| graph[index].iter().cloned().collect();
        assert_eq!(dijkstra(func, 0, |&v| v == 1), Some(1));
        assert_eq!(dijkstra(func, 0, |&v| v == 3), Some(3));
        assert_eq!(dijkstra(func, 3, |&v| v == 0), Some(7));
        assert_eq!(dijkstra(func, 0, |&v| v == 4), Some(5));
        assert_eq!(dijkstra(func, 4, |&v| v == 0), None);
    }
}
