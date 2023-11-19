use std::{collections::HashSet, fmt::Display};

fn main() {
    aoc::aoc_main(aoc::input!(), part1, part2)
}

fn part1(input: &str) -> impl Display {
    filtered_count(input, is_nice_part1)
}

fn part2(input: &str) -> impl Display {
    filtered_count(input, is_nice_part2)
}

fn filtered_count(input: &str, filter: fn(&str) -> bool) -> impl Display {
    input.lines().filter(|s| filter(&s)).count()
}

fn is_nice_part1(word: &str) -> bool {
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

fn is_nice_part2(word: &str) -> bool {
    has_double_pair(word) && has_alternating(word)
}

fn has_double_pair(word: &str) -> bool {
    let mut pairs = HashSet::new();
    let mut previous = None;
    for pair in word.chars().tuples() {
        if pairs.contains(&pair) {
            return true;
        }
        if let Some(previous) = previous {
            pairs.insert(previous);
        }
        previous = Some(pair);
    }
    false
}

fn has_alternating(word: &str) -> bool {
    word.chars().triplet().any(|(a, _, c)| a == c)
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

struct TripletIter<I, T> {
    inner: I,
    previous: Option<(T, T)>,
}

impl<I, T> Iterator for TripletIter<I, T>
where
    I: Iterator<Item = T>,
    T: Clone,
{
    type Item = (T, T, T);

    fn next(&mut self) -> Option<Self::Item> {
        let previous = self.previous.as_mut()?;
        let new = self.inner.next()?;
        let res = Some((previous.0.clone(), previous.1.clone(), new.clone()));
        std::mem::swap(&mut previous.0, &mut previous.1);
        previous.1 = new;
        res
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

    fn triplet(mut self) -> TripletIter<Self, Self::Item> {
        let previous = get_init(&mut self);
        TripletIter {
            inner: self,
            previous,
        }
    }
}

fn get_init<I: Iterator>(iter: &mut I) -> Option<(I::Item, I::Item)> {
    let first = iter.next()?;
    let second = iter.next()?;
    Some((first, second))
}

#[cfg(test)]
mod tests {
    use crate::{has_double_pair, is_nice_part1, is_nice_part2};

    #[test]
    fn example1() {
        assert!(is_nice_part1("ugknbfddgicrmopn"));
    }

    #[test]
    fn example2() {
        assert!(is_nice_part1("aaa"));
    }

    #[test]
    fn example3() {
        assert!(!is_nice_part1("jchzalrnumimnmhp"));
    }

    #[test]
    fn example4() {
        assert!(!is_nice_part1("haegwjzuvuyypxyu"));
    }

    #[test]
    fn example5() {
        assert!(!is_nice_part1("dvszwmarrgswjxmb"));
    }

    #[test]
    fn no_overlap_double_pair() {
        assert!(has_double_pair("xyxy"));
    }

    #[test]
    fn overlap_double_pair() {
        assert!(!has_double_pair("aaa"));
    }

    #[test]
    fn example6() {
        assert!(is_nice_part2("qjhvhtzxzqqjkmpb"));
    }

    #[test]
    fn example7() {
        assert!(is_nice_part2("xxyxx"));
    }

    #[test]
    fn example8() {
        assert!(!is_nice_part2("uurcxstgmygtbstg"));
    }

    #[test]
    fn example9() {
        assert!(!is_nice_part2("ieodomkazucvgmuy"));
    }
}
