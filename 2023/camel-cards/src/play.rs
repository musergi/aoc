use std::str::FromStr;

use crate::hand::Hand;

pub struct Play {
    pub hand: Hand,
    pub bid: u64,
}

impl FromStr for Play {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_whitespace();
        let hand = it.next().ok_or("missing hand")?.parse()?;
        let bid = it
            .next()
            .ok_or("missing bid")?
            .parse()
            .map_err(|_| "invalid bid")?;
        Ok(Play { hand, bid })
    }
}

#[cfg(test)]
mod tests {
    use crate::play::Play;

    #[test]
    fn parse_example1() {
        let play: Play = "32T3K 765".parse().unwrap();
        assert_eq!(play.bid, 765);
    }

    #[test]
    fn parse_example2() {
        let play: Play = "T55J5 684".parse().unwrap();
        assert_eq!(play.bid, 684);
    }
}
