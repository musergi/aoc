use std::collections::HashSet;

#[derive(Clone)]
pub struct Pattern {
    rocks: HashSet<(usize, usize)>,
    size: (usize, usize),
}

impl Pattern {
    pub fn new(rocks: HashSet<(usize, usize)>, columns: usize, rows: usize) -> Self {
        let size = (rows, columns);
        Self { rocks, size }
    }

    pub fn summarize(&self) -> Result<usize, &'static str> {
        self.filtered_summary(None)
    }

    fn filtered_summary(&self, filter: Option<usize>) -> Result<usize, &'static str> {
        for column_boundry in 1..self.size.1 {
            if self.is_symmetric(column_boundry, Horizontal) {
                if filter
                    .map(|filter| column_boundry != filter)
                    .unwrap_or(true)
                {
                    return Ok(column_boundry);
                }
            }
        }
        for row_boundry in 1..self.size.0 {
            if self.is_symmetric(row_boundry, Vertical) {
                if filter
                    .map(|filter| row_boundry * 100 != filter)
                    .unwrap_or(true)
                {
                    return Ok(row_boundry * 100);
                }
            }
        }
        Err("no symmetry")
    }

    pub fn one_off_summarize(&self) -> Result<usize, &'static str> {
        let mut modifiable = self.clone();
        let filter = Some(self.summarize()?);
        for row in 0..self.size.0 {
            for column in 0..self.size.1 {
                let modified = (row, column);
                if self.rocks.contains(&modified) {
                    modifiable.rocks.remove(&modified);
                    if let Ok(summary) = modifiable.filtered_summary(filter) {
                        return Ok(summary);
                    }
                    modifiable.rocks.insert(modified);
                } else {
                    modifiable.rocks.insert(modified);
                    if let Ok(summary) = modifiable.filtered_summary(filter) {
                        return Ok(summary);
                    }
                    modifiable.rocks.remove(&modified);
                }
            }
        }
        Err("no one off symmetry")
    }

    fn is_symmetric(&self, boundry: usize, axis: impl Axis) -> bool {
        for rock in self.rocks.iter() {
            if let Some(symmetric) = self.get_symmetric(rock, &boundry, &axis) {
                if !self.rocks.contains(&symmetric) {
                    return false;
                }
            }
        }
        true
    }

    fn get_symmetric(
        &self,
        rock: &(usize, usize),
        boundry: &usize,
        axis: &impl Axis,
    ) -> Option<(usize, usize)> {
        let prev = boundry - 1;
        prev.checked_sub(axis.get(rock))
            .map(|prev_distance| boundry + prev_distance)
            .or_else(|| {
                axis.get(rock)
                    .checked_sub(*boundry)
                    .and_then(|boundry_dist| prev.checked_sub(boundry_dist))
            })
            .filter(|new_position| new_position < &axis.get(&self.size))
            .map(|axis_position| axis.changed(rock, axis_position))
    }
}

trait Axis {
    fn get(&self, point: &(usize, usize)) -> usize;
    fn changed(&self, point: &(usize, usize), new_value: usize) -> (usize, usize);
}

struct Horizontal;

impl Axis for Horizontal {
    fn get(&self, point: &(usize, usize)) -> usize {
        point.1
    }

    fn changed(&self, point: &(usize, usize), new_value: usize) -> (usize, usize) {
        (point.0, new_value)
    }
}

struct Vertical;

impl Axis for Vertical {
    fn get(&self, point: &(usize, usize)) -> usize {
        point.0
    }

    fn changed(&self, point: &(usize, usize), new_value: usize) -> (usize, usize) {
        (new_value, point.1)
    }
}

#[cfg(test)]
mod tests {
    use super::Pattern;

    #[test]
    fn middle_horizontal_summarization() {
        let pattern = Pattern::new([(0, 0), (0, 3), (1, 1), (1, 2)].into_iter().collect(), 4, 2);
        assert_eq!(pattern.summarize().unwrap(), 2);
    }

    #[test]
    fn left_out_of_bound_summarization() {
        let pattern = Pattern::new([(1, 0), (1, 1), (0, 2)].into_iter().collect(), 3, 2);
        assert_eq!(pattern.summarize().unwrap(), 1);
    }

    #[test]
    fn right_out_of_bound_summarization() {
        let pattern = Pattern::new([(0, 0), (1, 1), (1, 2)].into_iter().collect(), 3, 2);
        assert_eq!(pattern.summarize().unwrap(), 2);
    }

    #[test]
    fn middle_vertical_summarization() {
        let pattern = Pattern::new([(0, 0), (3, 0), (1, 1), (2, 1)].into_iter().collect(), 2, 4);
        assert_eq!(pattern.summarize().unwrap(), 200);
    }
}
