use std::str::FromStr;

const CARD_COUNT: usize = 13;
const CARDS: [char; CARD_COUNT] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];
const NEW_CARDS: [char; CARD_COUNT] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

pub struct OldHand {
    hand: Hand,
    counters: [u8; CARD_COUNT],
}

impl OldHand {
    pub fn new(hand: Hand) -> OldHand {
        let mut counters = [0; CARD_COUNT];
        for card in hand.cards.iter() {
            let idx = CARDS
                .iter()
                .position(|card_type| card_type == card)
                .unwrap();
            counters[idx] += 1;
        }
        OldHand { hand, counters }
    }

    fn hand_type(&self) -> HandType {
        joker_less_hand_type(&self.counters)
    }

    fn indices(&self) -> [usize; 5] {
        self.hand.indices(&CARDS)
    }
}

impl PartialEq for OldHand {
    fn eq(&self, other: &Self) -> bool {
        self.hand.eq(&other.hand)
    }
}

impl Eq for OldHand {}

impl PartialOrd for OldHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(match self.hand_type().cmp(&other.hand_type()) {
            std::cmp::Ordering::Equal => other.indices().cmp(&self.indices()),
            ord => ord,
        })
    }
}

impl Ord for OldHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct NewHand {
    hand: Hand,
    counters: [u8; CARD_COUNT - 1],
    jokers: usize,
}

impl NewHand {
    pub fn new(hand: Hand) -> NewHand {
        let mut counters = [0; CARD_COUNT - 1];
        let mut jokers = 0;
        for card in hand.cards.iter() {
            if *card == 'J' {
                jokers += 1
            } else {
                let idx = CARDS
                    .iter()
                    .filter(|&card_type| *card_type != 'J')
                    .position(|card_type| card_type == card)
                    .unwrap();
                counters[idx] += 1;
            }
        }
        NewHand {
            hand,
            counters,
            jokers,
        }
    }

    fn hand_type(&self) -> HandType {
        let mut hand_type = joker_less_hand_type(&self.counters);
        for _ in 0..self.jokers {
            hand_type = hand_type.improve();
        }
        hand_type
    }

    fn indices(&self) -> [usize; 5] {
        self.hand.indices(&NEW_CARDS)
    }
}

impl PartialEq for NewHand {
    fn eq(&self, other: &Self) -> bool {
        self.hand.eq(&other.hand)
    }
}

impl Eq for NewHand {}

impl PartialOrd for NewHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(match self.hand_type().cmp(&other.hand_type()) {
            std::cmp::Ordering::Equal => other.indices().cmp(&self.indices()),
            ord => ord,
        })
    }
}

impl Ord for NewHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn joker_less_hand_type(counters: &[u8]) -> HandType {
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

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    cards: [char; 5],
}

impl Hand {
    fn indices(&self, order: &[char; CARD_COUNT]) -> [usize; 5] {
        [
            order
                .iter()
                .position(|&card| card == self.cards[0])
                .unwrap(),
            order
                .iter()
                .position(|&card| card == self.cards[1])
                .unwrap(),
            order
                .iter()
                .position(|&card| card == self.cards[2])
                .unwrap(),
            order
                .iter()
                .position(|&card| card == self.cards[3])
                .unwrap(),
            order
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

impl HandType {
    fn improve(self) -> HandType {
        match self {
            HandType::High => HandType::Pair,
            HandType::Pair => HandType::Three,
            HandType::Double => HandType::Full,
            HandType::Three => HandType::Four,
            HandType::Full => HandType::Four,
            HandType::Four => HandType::Five,
            HandType::Five => HandType::Five,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Hand, NewHand, OldHand};

    #[test]
    fn example_sort_old() {
        let mut hands: Vec<_> = vec![
            "32T3K".parse().unwrap(),
            "T55J5".parse().unwrap(),
            "KK677".parse().unwrap(),
            "KTJJT".parse().unwrap(),
            "QQQJA".parse().unwrap(),
        ]
        .into_iter()
        .map(OldHand::new)
        .collect();
        hands.sort();
        assert_eq!(hands.get(4).unwrap().hand, "QQQJA".parse::<Hand>().unwrap());
        assert_eq!(hands.get(3).unwrap().hand, "T55J5".parse::<Hand>().unwrap());
        assert_eq!(hands.get(2).unwrap().hand, "KK677".parse::<Hand>().unwrap());
        assert_eq!(hands.get(1).unwrap().hand, "KTJJT".parse::<Hand>().unwrap());
        assert_eq!(hands.get(0).unwrap().hand, "32T3K".parse::<Hand>().unwrap());
    }

    #[test]
    fn example_sort_new() {
        let mut hands: Vec<_> = vec![
            "32T3K".parse().unwrap(),
            "T55J5".parse().unwrap(),
            "KK677".parse().unwrap(),
            "KTJJT".parse().unwrap(),
            "QQQJA".parse().unwrap(),
        ]
        .into_iter()
        .map(NewHand::new)
        .collect();
        hands.sort();
        assert_eq!(hands.get(4).unwrap().hand, "KTJJT".parse::<Hand>().unwrap());
        assert_eq!(hands.get(3).unwrap().hand, "QQQJA".parse::<Hand>().unwrap());
        assert_eq!(hands.get(2).unwrap().hand, "T55J5".parse::<Hand>().unwrap());
        assert_eq!(hands.get(1).unwrap().hand, "KK677".parse::<Hand>().unwrap());
        assert_eq!(hands.get(0).unwrap().hand, "32T3K".parse::<Hand>().unwrap());
    }
}
