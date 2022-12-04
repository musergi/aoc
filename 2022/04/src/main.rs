use std::{
    fs::File,
    io::{self, BufRead},
    num::ParseIntError,
    str::FromStr,
};

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn fully_contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn intersect(&self, other: &Self) -> bool {
        self.start <= other.end && other.start <= self.end
    }
}

#[derive(Debug)]
enum RangeParseError {
    MissingStart(String),
    MissingEnd(String),
    BadNumberFormat(ParseIntError),
}

impl From<ParseIntError> for RangeParseError {
    fn from(err: ParseIntError) -> Self {
        Self::BadNumberFormat(err)
    }
}

impl FromStr for Range {
    type Err = RangeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split("-");
        let start = splits
            .next()
            .ok_or(RangeParseError::MissingStart(s.to_string()))?
            .parse::<u32>()?;
        let end = splits
            .next()
            .ok_or(RangeParseError::MissingEnd(s.to_string()))?
            .parse::<u32>()?;
        Ok(Range { start, end })
    }
}

struct ElfPair {
    first: Range,
    second: Range,
}

#[derive(Debug)]
enum ElfPairParseError {
    MissingFirst(String),
    MissingSecond(String),
    RangeParseError(RangeParseError),
}

impl From<RangeParseError> for ElfPairParseError {
    fn from(err: RangeParseError) -> Self {
        ElfPairParseError::RangeParseError(err)
    }
}

impl FromStr for ElfPair {
    type Err = ElfPairParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split(",");
        let first = splits
            .next()
            .ok_or(ElfPairParseError::MissingFirst(s.to_string()))?
            .parse::<Range>()?;
        let second = splits
            .next()
            .ok_or(ElfPairParseError::MissingSecond(s.to_string()))?
            .parse::<Range>()?;
        Ok(ElfPair { first, second })
    }
}

fn main() {
    let f = File::open("assets/input.txt").expect("File");
    let pairs = io::BufReader::new(f)
        .lines()
        .map(|l| l.expect("Line").parse::<ElfPair>().expect("Pair"))
        .collect::<Vec<_>>();
    let fully_contained = pairs
        .iter()
        .filter(|p| p.first.fully_contains(&p.second) || p.second.fully_contains(&p.first))
        .count();
    println!("Fully contained pairs: {}", fully_contained);
    let intersecting = pairs
        .iter()
        .filter(|p| p.first.intersect(&p.second))
        .count();
    println!("Intersecting pairs: {}", intersecting);
}
