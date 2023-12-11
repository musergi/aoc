use crate::history::History;
use std::str::FromStr;

pub struct Reading {
    histories: Vec<History>,
}

impl Reading {
    pub fn predict_next_sum(&self) -> i32 {
        self.histories.iter().map(|line| line.predict_next()).sum()
    }

    pub fn predict_previous_sum(&self) -> i32 {
        self.histories
            .iter()
            .map(|line| line.predict_previous())
            .sum()
    }
}

impl FromStr for Reading {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let histories = s
            .lines()
            .map(|string| string.parse::<History>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Reading { histories })
    }
}

#[cfg(test)]
mod tests {
    use super::Reading;

    #[test]
    fn example() {
        let string = include_str!("../assets/example.txt");
        let reading: Reading = string.parse().unwrap();
        assert_eq!(reading.predict_next_sum(), 114);
    }
}
