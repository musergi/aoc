use std::str::FromStr;

use crate::competition::Competition;

pub struct Scoreboard {
    competitions: Vec<Competition>,
}
impl Scoreboard {
    pub fn wining_move_count(&self) -> u32 {
        self.competitions
            .iter()
            .map(|competition| competition.wining_move_count())
            .fold(1, |acc, new| acc * new)
    }
}

impl FromStr for Scoreboard {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let times = lines
            .next()
            .ok_or("missing time line")?
            .strip_prefix("Time:")
            .ok_or("missing time line prefix")?
            .split_whitespace()
            .map(|time_string| time_string.parse::<u32>().map_err(|_| "invalid time value"))
            .collect::<Result<Vec<_>, _>>()?;
        let distances = lines
            .next()
            .ok_or("missing distance line")?
            .strip_prefix("Distance:")
            .ok_or("missing distance line prefix")?
            .split_whitespace()
            .map(|time_string| {
                time_string
                    .parse::<u32>()
                    .map_err(|_| "invalid distance value")
            })
            .collect::<Result<Vec<_>, _>>()?;
        if times.len() == distances.len() {
            let competitions = times
                .into_iter()
                .zip(distances)
                .map(|(time, distance)| Competition::new(time, distance))
                .collect();
            Ok(Scoreboard { competitions })
        } else {
            Err("scoreboard lines must have equal length")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Scoreboard;

    #[test]
    fn example_wining_move_count() {
        let string = include_str!("../assets/example.txt");
        let scoreboard: Scoreboard = string.parse().unwrap();
        assert_eq!(scoreboard.wining_move_count(), 288);
    }
}
