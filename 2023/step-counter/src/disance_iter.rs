use crate::vec::Vec2i;

pub struct DistanceIter {
    index: i64,
    max: i64,
    secondary: TileIterator,
}

impl From<i64> for DistanceIter {
    fn from(value: i64) -> Self {
        DistanceIter {
            index: 0,
            max: value,
            secondary: TileIterator::new((0, value).into()),
        }
    }
}

impl Iterator for DistanceIter {
    type Item = Vec2i;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.secondary.next() {
            Some(next)
        } else {
            self.index += 1;
            if self.index <= self.max {
                let base = (self.index, self.max - self.index);
                self.secondary = TileIterator::new(base.into());
                self.secondary.next()
            } else {
                None
            }
        }
    }
}

struct TileIterator {
    index: usize,
    size: usize,
    tiles: [Vec2i; 4],
}

impl TileIterator {
    fn new(base: Vec2i) -> Self {
        let mut tiles = [(0, 0).into(); 4];
        let mut size = 0;
        for multiplier in [(1, 1), (-1, 1), (1, -1), (-1, -1)] {
            let position = (base.row * multiplier.0, base.column * multiplier.1).into();
            if !tiles[0..size].contains(&position) {
                tiles[size] = position;
                size += 1;
            }
        }
        Self {
            index: 0,
            size,
            tiles,
        }
    }
}

impl Iterator for TileIterator {
    type Item = Vec2i;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.size {
            let v = self.tiles[self.index];
            self.index += 1;
            Some(v)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DistanceIter, TileIterator};

    #[test]
    fn return_single_element_on_full_symmetry() {
        let mut iter = TileIterator::new((0, 0).into());
        assert_eq!(iter.next().unwrap(), (0, 0).into());
        assert!(iter.next().is_none())
    }

    #[test]
    fn return_two_on_single_axis_symmetry() {
        let mut iter = TileIterator::new((3, 0).into());
        assert_eq!(iter.next().unwrap(), (3, 0).into());
        assert_eq!(iter.next().unwrap(), (-3, 0).into());
        assert!(iter.next().is_none())
    }

    #[test]
    fn return_all_on_no_symmetry() {
        let mut iter = TileIterator::new((3, 1).into());
        assert_eq!(iter.next().unwrap(), (3, 1).into());
        assert_eq!(iter.next().unwrap(), (-3, 1).into());
        assert_eq!(iter.next().unwrap(), (3, -1).into());
        assert_eq!(iter.next().unwrap(), (-3, -1).into());
        assert!(iter.next().is_none())
    }

    #[test]
    fn example_distance_iter() {
        let mut iter = DistanceIter::from(2);
        assert_eq!(iter.next().unwrap(), (0, 2).into());
        assert_eq!(iter.next().unwrap(), (0, -2).into());
        assert_eq!(iter.next().unwrap(), (1, 1).into());
        assert_eq!(iter.next().unwrap(), (-1, 1).into());
        assert_eq!(iter.next().unwrap(), (1, -1).into());
        assert_eq!(iter.next().unwrap(), (-1, -1).into());
        assert_eq!(iter.next().unwrap(), (2, 0).into());
        assert_eq!(iter.next().unwrap(), (-2, 0).into());
        assert!(iter.next().is_none())
    }
}
