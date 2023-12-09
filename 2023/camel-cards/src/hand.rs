use std::str::FromStr;

const CARD_COUNT: usize = 13;
const CARDS: [char; CARD_COUNT] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    cards: [char; 5],
}

impl Hand {
    fn counters(&self) -> [u8; CARD_COUNT] {
        let mut counters = [0; CARD_COUNT];
        for card in self.cards.iter() {
            let idx = CARDS
                .iter()
                .position(|card_type| card_type == card)
                .unwrap();
            counters[idx] += 1;
        }
        counters
    }

    fn hand_type(&self) -> HandType {
        let counters = self.counters();
        if counters.iter().any(|&c| c == 5) {
            HandType::Five
        } else if counters.iter().any(|&c| c == 4) {
            HandType::Four
        } else {
            let has_three = counters.iter().any(|&c| c == 3);
            if has_three && counters.iter().any(|&c| c == 2) {
                HandType::Full
            } else if has_three {
                HandType::Three
            } else {
                match counters.iter().filter(|&c| *c == 2).count() {
                    2 => HandType::Double,
                    1 => HandType::Pair,
                    _ => HandType::High,
                }
            }
        }
    }

    fn indices(&self) -> [usize; 5] {
        [
            CARDS
                .iter()
                .position(|&card| card == self.cards[0])
                .unwrap(),
            CARDS
                .iter()
                .position(|&card| card == self.cards[1])
                .unwrap(),
            CARDS
                .iter()
                .position(|&card| card == self.cards[2])
                .unwrap(),
            CARDS
                .iter()
                .position(|&card| card == self.cards[3])
                .unwrap(),
            CARDS
                .iter()
                .position(|&card| card == self.cards[4])
                .unwrap(),
        ]
    }
}

impl FromStr for Hand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.chars();
        let cards = [
            parse_card(&mut it)?,
            parse_card(&mut it)?,
            parse_card(&mut it)?,
            parse_card(&mut it)?,
            parse_card(&mut it)?,
        ];
        Ok(Hand { cards })
    }
}

fn parse_card<I>(it: &mut I) -> Result<char, &'static str>
where
    I: Iterator<Item = char>,
{
    let c = it.next().ok_or("missing card in hand")?;
    if CARDS.contains(&c) {
        Ok(c)
    } else {
        Err("invalid char for card")
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(match self.hand_type().cmp(&other.hand_type()) {
            std::cmp::Ordering::Equal => other.indices().cmp(&self.indices()),
            ord => ord,
        })
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    High,
    Pair,
    Double,
    Three,
    Full,
    Four,
    Five,
}

#[cfg(test)]
mod tests {
    use super::Hand;

    #[test]
    fn example_sort() {
        let mut hands: Vec<Hand> = vec![
            "32T3K".parse().unwrap(),
            "T55J5".parse().unwrap(),
            "KK677".parse().unwrap(),
            "KTJJT".parse().unwrap(),
            "QQQJA".parse().unwrap(),
        ];
        hands.sort();
        assert_eq!(hands.get(4).unwrap(), &"QQQJA".parse::<Hand>().unwrap());
        assert_eq!(hands.get(3).unwrap(), &"T55J5".parse::<Hand>().unwrap());
        assert_eq!(hands.get(2).unwrap(), &"KK677".parse::<Hand>().unwrap());
        assert_eq!(hands.get(1).unwrap(), &"KTJJT".parse::<Hand>().unwrap());
        assert_eq!(hands.get(0).unwrap(), &"32T3K".parse::<Hand>().unwrap());
    }
}
