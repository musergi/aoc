use std::str::FromStr;

use crate::{
    mapping::Mapping,
    range::{Range, RangeMapping},
};

pub struct Almanac {
    seeds: Vec<u64>,
    mappings: Vec<Mapping>,
}

impl Almanac {
    pub fn get_min_location(&self) -> Option<u64> {
        self.seeds
            .iter()
            .map(|seed| {
                self.mappings
                    .iter()
                    .fold(*seed, |seed, mapping| mapping.apply(seed))
            })
            .min()
    }

    pub fn get_min_ranges_location(&self) -> Option<u64> {
        self.seeds
            .iter()
            .pairs()
            .map(|(&start, &length)| Range::new(start, length))
            .filter_map(|range| self.get_min_range_location(range))
            .min()
    }

    fn get_min_range_location(&self, range: Range) -> Option<u64> {
        let mut it: Box<dyn Iterator<Item = Range>> = Box::new(std::iter::once(range));
        for mapping in self.mappings.iter() {
            it = Box::new(it.flat_map(|range| mapping.ranges(range)));
        }
        it.map(|range| range.start()).min()
    }
}

impl FromStr for Almanac {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.lines();
        let seeds = it
            .next()
            .ok_or("missing seeds line")?
            .strip_prefix("seeds:")
            .ok_or("missing seed prefix")?
            .split_whitespace()
            .map(|num| num.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| "invalid seed number")?;
        if !it
            .next()
            .ok_or("missing empty line after seeds")?
            .is_empty()
        {
            return Err("line after seeds should be empty");
        }
        let mut mappings = Vec::new();
        mappings.push(start_mapping(&mut it)?);
        let mut input_consumed = false;
        while !input_consumed {
            let line = it.next();
            if let Some(line) = line {
                if line.is_empty() {
                    mappings.push(start_mapping(&mut it)?);
                } else {
                    let mut it = line.split_whitespace();
                    let destination_start = it
                        .next()
                        .ok_or("missing range destination start")?
                        .parse()
                        .map_err(|_| "invalid range destination start")?;
                    let source_start = it
                        .next()
                        .ok_or("missing range source start")?
                        .parse()
                        .map_err(|_| "invalid range source start")?;
                    let length = it
                        .next()
                        .ok_or("missing range length")?
                        .parse()
                        .map_err(|_| "invalid range length")?;
                    let range = RangeMapping {
                        source_start,
                        destination_start,
                        length,
                    };
                    mappings.last_mut().unwrap().ranges.push(range);
                }
            } else {
                input_consumed = true;
            }
        }
        Ok(Almanac { seeds, mappings })
    }
}

fn start_mapping<'a, I>(it: &mut I) -> Result<Mapping, &'static str>
where
    I: Iterator<Item = &'a str>,
{
    it.next()
        .ok_or("missing map section")?
        .strip_suffix(" map:")
        .ok_or("missing map suffix")?
        .split_once("-to-")
        .map(|(source, destination)| Mapping::new(source, destination))
        .ok_or("missing map source to destination separator")
}

struct PairIterator<I>
where
    I: Iterator,
{
    inner: I,
}

impl<I> Iterator for PairIterator<I>
where
    I: Iterator,
{
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let first = self.inner.next()?;
        let second = self.inner.next()?;
        Some((first, second))
    }
}

trait IteratorExt: Iterator + Sized {
    fn pairs(self) -> PairIterator<Self> {
        PairIterator { inner: self }
    }
}

impl<T> IteratorExt for T where T: Iterator + Sized {}

#[cfg(test)]
mod tests {
    use crate::almanac::Almanac;

    #[test]
    fn parse_almanac() {
        let s = include_str!("../assets/example.txt");
        let almanac: Almanac = s.parse().unwrap();
        assert_eq!(almanac.seeds, vec![79, 14, 55, 13]);
        assert_eq!(almanac.mappings.len(), 7);
        let mapping = almanac.mappings.get(0).unwrap();
        assert_eq!(mapping.source, "seed");
        assert_eq!(mapping.destination, "soil");
        assert_eq!(mapping.ranges.len(), 2);
        let range = mapping.ranges.get(0).unwrap();
        assert_eq!(range.source_start, 98);
        assert_eq!(range.destination_start, 50);
        assert_eq!(range.length, 2);
    }

    #[test]
    fn example_location() {
        let s = include_str!("../assets/example.txt");
        let almanac: Almanac = s.parse().unwrap();
        assert_eq!(almanac.get_min_location().unwrap(), 35);
    }

    #[test]
    fn example_get_min_range_location() {
        let s = include_str!("../assets/example.txt");
        let almanac: Almanac = s.parse().unwrap();
        assert_eq!(almanac.get_min_ranges_location().unwrap(), 46);
    }
}
