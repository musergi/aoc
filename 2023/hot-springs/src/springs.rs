use crate::spring_row::SpringRow;
use std::str::FromStr;

pub struct Springs {
    rows: Vec<SpringRow>,
}

impl Springs {
    pub fn count_sum(&self) -> usize {
        self.rows.iter().map(|row| row.combinations()).sum()
    }
}

impl FromStr for Springs {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s
            .lines()
            .map(|line| line.parse::<SpringRow>())
            .collect::<Result<_, _>>()?;
        Ok(Springs { rows })
    }
}
