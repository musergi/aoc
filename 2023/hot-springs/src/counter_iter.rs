pub struct CountIter<I, T> {
    iter: I,
    last: Option<T>,
}

impl<I, T> Iterator for CountIter<I, T>
where
    I: Iterator<Item = T>,
    T: PartialEq,
{
    type Item = (usize, T);

    fn next(&mut self) -> Option<Self::Item> {
        let mut count = 1usize;
        if self.last.is_none() {
            return None;
        }
        loop {
            let mut new = self.iter.next();
            if new != self.last {
                std::mem::swap(&mut self.last, &mut new);
                return Some((count, new.unwrap()));
            } else {
                count += 1;
            }
        }
    }
}

pub trait IteratorExt: Iterator + Sized {
    fn counts(self) -> CountIter<Self, Self::Item>;
}

impl<I, T> IteratorExt for I
where
    I: Iterator<Item = T>,
    T: PartialEq,
{
    fn counts(mut self) -> CountIter<Self, Self::Item> {
        let last = self.next();
        CountIter { iter: self, last }
    }
}

#[cfg(test)]
mod tests {
    use super::IteratorExt;

    #[test]
    fn count_iter_single() {
        let mut it = vec![1, 1, 1].into_iter().counts();
        assert_eq!(it.next().unwrap(), (3, 1));
        assert!(it.next().is_none());
    }

    #[test]
    fn count_iter_double() {
        let mut it = vec![1, 1, 2, 2, 2, 2].into_iter().counts();
        assert_eq!(it.next().unwrap(), (2, 1));
        assert_eq!(it.next().unwrap(), (4, 2));
        assert!(it.next().is_none());
    }

    #[test]
    fn count_optional() {
        let mut it = vec![Some(1), Some(1), None, Some(2), Some(2), Some(2)]
            .into_iter()
            .counts();
        assert_eq!(it.next().unwrap(), (2, Some(1)));
        assert_eq!(it.next().unwrap(), (1, None));
        assert_eq!(it.next().unwrap(), (3, Some(2)));
        assert!(it.next().is_none());
    }
}
