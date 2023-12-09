use crate::range::{Range, RangeMapping};

pub struct Mapping {
    pub source: String,
    pub destination: String,
    pub ranges: Vec<RangeMapping>,
}

impl Mapping {
    pub fn new(source: &str, destination: &str) -> Mapping {
        let source = source.to_string();
        let destination = destination.to_string();
        Mapping {
            source,
            destination,
            ranges: Vec::new(),
        }
    }

    pub fn apply(&self, value: u64) -> u64 {
        self.ranges
            .iter()
            .find(|range| range.is_in(value))
            .map(|range| range.apply(value))
            .unwrap_or(value)
    }

    pub fn ranges(&self, mut in_range: Range) -> impl Iterator<Item = Range> {
        let mut sorted: Vec<_> = self.ranges.iter().collect();
        sorted.sort_by_key(|range| range.source_start);
        let mut out: Vec<Range> = Vec::new();
        for range in sorted {
            if range.intersects(&in_range) {
                if let Some(prefix) = range.align(&mut in_range) {
                    out.push(prefix);
                }
                let partition = range.partition(&mut in_range);
                out.push(partition);
            }
        }
        if in_range.length() > 0 {
            out.push(in_range);
        }
        out.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        mapping::Mapping,
        range::{Range, RangeMapping},
    };

    #[test]
    fn apply_mapping() {
        let mapping = Mapping {
            source: "s".to_string(),
            destination: "d".to_string(),
            ranges: vec![
                RangeMapping {
                    source_start: 98,
                    destination_start: 50,
                    length: 2,
                },
                RangeMapping {
                    source_start: 95,
                    destination_start: 0,
                    length: 2,
                },
            ],
        };
        assert_eq!(mapping.apply(95), 0);
        assert_eq!(mapping.apply(96), 1);
        assert_eq!(mapping.apply(97), 97);
        assert_eq!(mapping.apply(98), 50);
        assert_eq!(mapping.apply(99), 51);
        assert_eq!(mapping.apply(100), 100);
    }

    #[test]
    fn ranges_no_match() {
        let mapping = Mapping {
            source: "s".to_string(),
            destination: "d".to_string(),
            ranges: vec![],
        };
        let ranges: Vec<_> = mapping.ranges(Range::new(0, 2)).collect();
        assert_eq!(ranges.len(), 1);
        let range = ranges.get(0).unwrap();
        assert_eq!(range.start(), 0);
        assert_eq!(range.length(), 2);
    }

    #[test]
    fn ranges_inner_match() {
        let mapping = Mapping {
            source: "s".to_string(),
            destination: "d".to_string(),
            ranges: vec![RangeMapping {
                source_start: 1,
                destination_start: 10,
                length: 2,
            }],
        };
        let ranges: Vec<_> = mapping.ranges(Range::new(0, 4)).collect();
        assert_eq!(ranges.len(), 3);
        let range = ranges.get(0).unwrap();
        assert_eq!(range.start(), 0);
        assert_eq!(range.end(), 0);
        let range = ranges.get(1).unwrap();
        assert_eq!(range.start(), 10);
        assert_eq!(range.end(), 11);
        let range = ranges.get(2).unwrap();
        assert_eq!(range.start(), 3);
        assert_eq!(range.end(), 3);
    }
}
