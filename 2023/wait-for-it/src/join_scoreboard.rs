use crate::{competition::Competition, scoreboard::Scoreboard};
use std::str::FromStr;

pub struct JoinScoreboard {
    competition: [Competition; 1],
}

impl Scoreboard for JoinScoreboard {
    fn competitions(&self) -> &[Competition] {
        &self.competition
    }
}

impl FromStr for JoinScoreboard {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let time_string: String = lines
            .next()
            .ok_or("missing time line")?
            .strip_prefix("Time:")
            .ok_or("missing time line prefix")?
            .chars()
            .filter(|char| !char.is_whitespace())
            .collect();
        let time = time_string.parse().map_err(|_| "failed to parse time")?;
        let distance_string: String = lines
            .next()
            .ok_or("missing distance line")?
            .strip_prefix("Distance:")
            .ok_or("missing distance line prefix")?
            .chars()
            .filter(|char| !char.is_whitespace())
            .collect();
        let distance = distance_string
            .parse()
            .map_err(|_| "failed to parse distance")?;
        let competition = [Competition::new(time, distance)];
        Ok(JoinScoreboard { competition })
    }
}

#[cfg(test)]
mod tests {
    use super::JoinScoreboard;
    use crate::scoreboard::Scoreboard;

    #[test]
    fn example_wining_move_count() {
        let string = include_str!("../assets/example.txt");
        let scoreboard: JoinScoreboard = string.parse().unwrap();
        assert_eq!(scoreboard.wining_move_count(), 71503);
    }
}
