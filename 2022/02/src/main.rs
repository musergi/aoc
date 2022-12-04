use std::{fs, str::FromStr};

trait Scoreable {
    fn get_score(&self) -> u32;
}

#[derive(Debug, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
struct MoveParseError(String);

impl FromStr for Move {
    type Err = MoveParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "C" => Ok(Move::Scissors),
            "X" => Ok(Move::Rock),
            "Y" => Ok(Move::Paper),
            "Z" => Ok(Move::Scissors),
            _ => Err(MoveParseError(s.to_string())),
        }
    }
}

impl Scoreable for Move {
    fn get_score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

impl Move {
    fn wins(&self, other: &Self) -> bool {
        match (self, other) {
            (Move::Rock, Move::Scissors) => true,
            (Move::Paper, Move::Rock) => true,
            (Move::Scissors, Move::Paper) => true,
            _ => false,
        }
    }

    fn apply_result(&self, outcome: &Outcome) -> Move {
        match outcome {
            Outcome::Loss => self.get_losing(),
            Outcome::Draw => self.clone(),
            Outcome::Win => self.get_wining(),
        }
    }

    fn get_wining(&self) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    fn get_losing(&self) -> Move {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }
}

#[derive(Debug)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

#[derive(Debug)]
enum OutcomeParseError {
    NoElfMove(String),
    NoUserMove(String),
    InvalidResult(String),
    Move(MoveParseError),
}

impl From<MoveParseError> for OutcomeParseError {
    fn from(err: MoveParseError) -> Self {
        OutcomeParseError::Move(err)
    }
}

impl FromStr for Outcome {
    type Err = OutcomeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits = s.split_whitespace().into_iter().collect::<Vec<_>>();
        let other_move = splits
            .get(0)
            .ok_or(OutcomeParseError::NoElfMove(s.to_string()))?
            .parse::<Move>()?;
        let self_move = splits
            .get(1)
            .ok_or(OutcomeParseError::NoUserMove(s.to_string()))?
            .parse::<Move>()?;
        match (self_move.wins(&other_move), other_move.wins(&self_move)) {
            (true, false) => Ok(Self::Win),
            (false, true) => Ok(Self::Loss),
            (false, false) => Ok(Self::Draw),
            _ => Err(OutcomeParseError::InvalidResult(s.to_string())),
        }
    }
}

struct AlternativeRound(Round);

impl FromStr for AlternativeRound {
    type Err = OutcomeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits = s.split_whitespace().into_iter().collect::<Vec<_>>();
        let outcome = match splits
            .get(1)
            .ok_or(OutcomeParseError::NoUserMove(s.to_string()))?
            .as_ref()
        {
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(OutcomeParseError::InvalidResult(s.to_string())),
        }?;
        let hand = splits
            .get(0)
            .ok_or(OutcomeParseError::NoElfMove(s.to_string()))?
            .parse::<Move>()?
            .apply_result(&outcome);
        Ok(AlternativeRound(Round { hand, outcome }))
    }
}

impl Scoreable for AlternativeRound {
    fn get_score(&self) -> u32 {
        self.0.get_score()
    }
}

impl Scoreable for Outcome {
    fn get_score(&self) -> u32 {
        match self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

#[derive(Debug)]
struct Round {
    hand: Move,
    outcome: Outcome,
}

impl FromStr for Round {
    type Err = OutcomeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let outcome = s.parse::<Outcome>()?;
        let hand = s
            .split_whitespace()
            .into_iter()
            .nth(1)
            .ok_or(OutcomeParseError::NoUserMove(s.to_string()))?
            .parse::<Move>()?;
        Ok(Round { hand, outcome })
    }
}

impl Scoreable for Round {
    fn get_score(&self) -> u32 {
        self.hand.get_score() + self.outcome.get_score()
    }
}

fn main() {
    let total = fs::read_to_string("assets/input.txt")
        .expect("File")
        .split("\n")
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<Round>().expect("Round"))
        .map(|r| r.get_score())
        .reduce(|a, b| a + b)
        .unwrap_or(0);
    println!("Total score: {}", total);
    let total = fs::read_to_string("assets/input.txt")
        .expect("File")
        .split("\n")
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<AlternativeRound>().expect("Round"))
        .map(|r| r.get_score())
        .reduce(|a, b| a + b)
        .unwrap_or(0);
    println!("New total score: {}", total);
}
