pub struct RangeMapping {
    pub source_start: u64,
    pub destination_start: u64,
    pub length: u64,
}

impl RangeMapping {
    pub fn is_in(&self, value: u64) -> bool {
        value >= self.source_start && value < self.source_start + self.length
    }

    pub fn apply(&self, value: u64) -> u64 {
        let offset = value - self.source_start;
        self.destination_start + offset
    }

    pub fn intersects(&self, range: &Range) -> bool {
        let self_source_end = self.source_start + self.length - 1;
        self.source_start <= range.end() && self_source_end >= range.start
    }

    pub fn align(&self, in_range: &mut Range) -> Option<Range> {
        if in_range.start() < self.source_start {
            let start = in_range.start();
            let length = self.source_start - in_range.start();
            *in_range = Range::new(self.source_start, in_range.length() - length);
            Some(Range::new(start, length))
        } else {
            None
        }
    }

    pub fn partition(&self, in_range: &mut Range) -> Range {
        let offset = in_range.start() - self.source_start;
        let length = (self.length - offset).min(in_range.length());
        in_range.start += length;
        in_range.length -= length;
        Range::new(self.destination_start + offset, length)
    }
}

pub struct Range {
    start: u64,
    length: u64,
}

impl Range {
    pub fn new(start: u64, length: u64) -> Range {
        Range { start, length }
    }

    pub fn start(&self) -> u64 {
        self.start
    }

    pub fn end(&self) -> u64 {
        self.start + self.length - 1
    }

    pub fn length(&self) -> u64 {
        self.length
    }
}

#[cfg(test)]
mod tests {
    use crate::range::{Range, RangeMapping};

    #[test]
    fn in_range() {
        let range = RangeMapping {
            source_start: 98,
            destination_start: 50,
            length: 2,
        };
        assert!(!range.is_in(97));
        assert!(range.is_in(98));
        assert!(range.is_in(99));
        assert!(!range.is_in(100));
    }

    #[test]
    fn apply_range() {
        let range = RangeMapping {
            source_start: 98,
            destination_start: 50,
            length: 2,
        };
        assert_eq!(range.apply(98), 50);
        assert_eq!(range.apply(99), 51);
    }

    #[test]
    fn intersect_inner() {
        let range = RangeMapping {
            source_start: 1,
            destination_start: 10,
            length: 2,
        };
        assert!(range.intersects(&Range::new(0, 4)));
    }

    #[test]
    fn intersect_surround() {
        let range = RangeMapping {
            source_start: 0,
            destination_start: 10,
            length: 4,
        };
        assert!(range.intersects(&Range::new(1, 2)));
    }

    #[test]
    fn intersect_partial() {
        let range = RangeMapping {
            source_start: 0,
            destination_start: 10,
            length: 2,
        };
        assert!(range.intersects(&Range::new(1, 2)));
    }

    #[test]
    fn not_intersect() {
        let range = RangeMapping {
            source_start: 0,
            destination_start: 10,
            length: 2,
        };
        assert!(!range.intersects(&Range::new(2, 2)));
    }

    #[test]
    fn align_unaligned() {
        let range = RangeMapping {
            source_start: 1,
            destination_start: 10,
            length: 2,
        };
        let mut in_range = Range::new(0, 4);

        let prefix = range.align(&mut in_range).unwrap();

        assert_eq!(prefix.start(), 0);
        assert_eq!(prefix.end(), 0);
        assert_eq!(in_range.start(), 1);
        assert_eq!(in_range.end(), 3);
    }

    #[test]
    fn align_aligned() {
        let range = RangeMapping {
            source_start: 0,
            destination_start: 10,
            length: 2,
        };
        let mut in_range = Range::new(0, 4);

        assert!(range.align(&mut in_range).is_none());
    }

    #[test]
    fn partition_simple() {
        let range = RangeMapping {
            source_start: 0,
            destination_start: 10,
            length: 2,
        };
        let mut in_range = Range::new(0, 4);

        let partition = range.partition(&mut in_range);

        assert_eq!(partition.start(), 10);
        assert_eq!(partition.end(), 11);
        assert_eq!(in_range.start(), 2);
        assert_eq!(in_range.end(), 3);
    }

    #[test]
    fn partition_left_overflow() {
        let range = RangeMapping {
            source_start: 0,
            destination_start: 10,
            length: 2,
        };
        let mut in_range = Range::new(1, 4);

        let partition = range.partition(&mut in_range);

        assert_eq!(partition.start(), 11);
        assert_eq!(partition.end(), 11);
        assert_eq!(in_range.start(), 2);
        assert_eq!(in_range.end(), 4);
    }

    #[test]
    fn partition_right_overflow() {
        let range = RangeMapping {
            source_start: 0,
            destination_start: 10,
            length: 4,
        };
        let mut in_range = Range::new(0, 2);

        let partition = range.partition(&mut in_range);

        assert_eq!(partition.start(), 10);
        assert_eq!(partition.end(), 11);
        assert_eq!(in_range.length(), 0);
    }
}
