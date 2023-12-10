use crate::competition::Competition;

pub trait Scoreboard {
    fn competitions(&self) -> &[Competition];

    fn wining_move_count(&self) -> u64 {
        self.competitions()
            .iter()
            .map(|competition| competition.wining_move_count())
            .fold(1, |acc, new| acc * new)
    }
}
