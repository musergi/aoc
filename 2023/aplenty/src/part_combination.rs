#[derive(Clone)]
pub struct PartCombination {
    pub x: (u64, u64),
    pub m: (u64, u64),
    pub a: (u64, u64),
    pub s: (u64, u64),
}

impl Default for PartCombination {
    fn default() -> Self {
        PartCombination {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        }
    }
}

impl PartCombination {
    pub fn combinations(&self) -> u64 {
        let Self { x, m, a, s } = self;
        [x, m, a, s]
            .map(|v| v.1 - v.0 + 1)
            .into_iter()
            .fold(1, |a, b| a * b)
    }
}

#[cfg(test)]
mod tests {
    use super::PartCombination;

    #[test]
    fn single_combination() {
        let combinations = PartCombination {
            x: (2, 2),
            m: (3, 3),
            a: (6, 6),
            s: (4000, 4000),
        };
        assert_eq!(combinations.combinations(), 1);
    }

    #[test]
    fn single_rating_combinations() {
        let combinations = PartCombination {
            x: (2, 2),
            m: (3, 4),
            a: (6, 6),
            s: (4000, 4000),
        };
        assert_eq!(combinations.combinations(), 2);
    }

    #[test]
    fn multi_rating_combinations() {
        let combinations = PartCombination {
            x: (2, 2),
            m: (3, 4),
            a: (6, 8),
            s: (4000, 4000),
        };
        assert_eq!(combinations.combinations(), 6);
    }
}
