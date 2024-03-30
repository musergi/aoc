pub struct Path<T> {
    tail: T,
    route: Vec<T>,
}

impl<T> Path<T>
where
    T: Copy + PartialEq,
{
    pub fn new(start: T) -> Self {
        Path {
            tail: start,
            route: Vec::new(),
        }
    }

    pub fn extend(&self, new: T) -> Self {
        let Path { tail, route } = self;
        let mut route = route.clone();
        route.push(*tail);
        Path { tail: new, route }
    }

    pub fn tail(&self) -> T {
        self.tail
    }

    pub fn len(&self) -> usize {
        self.route.len()
    }

    pub fn distance<F>(&self, func: F) -> usize
    where
        F: Fn(T, T) -> usize,
    {
        self.route
            .iter()
            .enumerate()
            .map(|(idx, start)| {
                let end = self.route.get(idx + 1).unwrap_or(&self.tail);
                func(*start, *end)
            })
            .sum()
    }

    pub fn contains(&self, position: &T) -> bool {
        position == &self.tail || self.route.contains(position)
    }
}
