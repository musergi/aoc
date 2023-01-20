use std::collections::VecDeque;

pub struct Path<T> {
    pub path: Vec<T>,
    pub metric: u32,
}

pub fn breadth_first_search<T>(
    start: T,
    edges: &dyn Fn(&[T]) -> Vec<T>,
    metric: &dyn Fn(&[T]) -> u32,
) -> Option<Path<T>>
where
    T: Clone,
{
    let mut queue = VecDeque::new();
    let mut best: Option<Path<T>> = None;

    queue.push_back(vec![start]);

    while !queue.is_empty() {
        let current_path = queue.pop_front().unwrap();
        let current_metric = metric(&current_path);
        let current = Path {
            path: current_path.clone(),
            metric: current_metric,
        };
        if let Some(b) = best {
            best = Some(match current.metric > b.metric {
                true => current,
                false => b,
            })
        } else {
            best = Some(current)
        }
        let neighbors = edges(&current_path);
        for neighbor in neighbors.iter() {
            let mut new_path = current_path.clone();
            new_path.push(neighbor.clone());
            queue.push_back(new_path);
        }
    }
    best
}

pub fn floyd_warshall(adj_mat: &mut Vec<Vec<u32>>) {
    let n = adj_mat.len();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                adj_mat[i][j] = std::cmp::min(adj_mat[i][j], adj_mat[i][k] + adj_mat[k][j]);
            }
        }
    }
}
