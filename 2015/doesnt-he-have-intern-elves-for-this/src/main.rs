fn main() {
    println!("Hello, world!");
}

fn is_nice(word: &str) -> bool {
    has_three_vowels(word) && has_double_letter(word) && !has_forbidden_subword(word)
}

fn has_three_vowels(word: &str) -> bool {
    word.chars()
        .map(|c| match c {
            'a' | 'e' | 'i' | 'o' | 'u' => 1,
            _ => 0,
        })
        .sum::<u32>()
        >= 3
}

fn has_double_letter(word: &str) -> bool {
    word.chars().into_iter().tuples().any(|(a, b)| a == b)
}

fn has_forbidden_subword(word: &str) -> bool {
    word.contains("ab") | word.contains("cd") | word.contains("pq") | word.contains("xy")
}

struct TupleIter<I, T> {
    inner: I,
    previous: Option<T>,
}

impl<I, T> Iterator for TupleIter<I, T>
where
    I: Iterator<Item = T>,
    T: Clone,
{
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        let previous = self.previous.clone()?;
        self.previous = Some(self.inner.next()?);
        Some((previous, self.previous.clone()?))
    }
}

impl<I> ExtIterator for I where I: Iterator {}

trait ExtIterator: Iterator + Sized {
    fn tuples(mut self) -> TupleIter<Self, Self::Item> {
        let previous = self.next();
        TupleIter {
            inner: self,
            previous,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::is_nice;

    #[test]
    fn example1() {
        assert!(is_nice("ugknbfddgicrmopn"));
    }

    #[test]
    fn example2() {
        assert!(is_nice("aaa"));
    }

    #[test]
    fn example3() {
        assert!(!is_nice("jchzalrnumimnmhp"));
    }

    #[test]
    fn example4() {
        assert!(!is_nice("haegwjzuvuyypxyu"));
    }

    #[test]
    fn example5() {
        assert!(!is_nice("dvszwmarrgswjxmb"));
    }
}
