use std::str::FromStr;

pub struct Card {
    id: u32,
    winning: Vec<u32>,
    contained: Vec<u32>,
}

impl Card {
    pub fn points(&self) -> u32 {
        match self.count() {
            0 => 0,
            n => 1 << n - 1,
        }
    }

    pub fn count(&self) -> usize {
        self.contained
            .iter()
            .filter(|num| self.winning.contains(num))
            .count()
    }
}

impl FromStr for Card {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(": ").ok_or("missing description separator")?;
        let id = left
            .strip_prefix("Card ")
            .ok_or("missing prefix")?
            .trim()
            .parse()
            .map_err(|_| "invalid id")?;
        let (winning_s, contained_s) = right.split_once(" | ").ok_or("missing values separator")?;
        let winning = winning_s
            .split_whitespace()
            .map(|num_s| num_s.parse())
            .collect::<Result<_, _>>()
            .map_err(|_| "invalid win number")?;
        let contained = contained_s
            .split_whitespace()
            .map(|num_s| num_s.parse())
            .collect::<Result<_, _>>()
            .map_err(|_| "invalid contained number")?;
        Ok(Card {
            id,
            winning,
            contained,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Card;

    #[test]
    fn parse_example1() {
        let card: Card = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
            .parse()
            .unwrap();
        assert_eq!(card.id, 1);
        assert_eq!(card.winning[0], 41);
        assert_eq!(card.winning[1], 48);
        assert_eq!(card.winning[2], 83);
        assert_eq!(card.winning[3], 86);
        assert_eq!(card.winning[4], 17);
        assert_eq!(card.contained[0], 83);
        assert_eq!(card.contained[1], 86);
        assert_eq!(card.contained[2], 6);
        assert_eq!(card.contained[3], 31);
        assert_eq!(card.contained[4], 17);
        assert_eq!(card.contained[5], 9);
        assert_eq!(card.contained[6], 48);
        assert_eq!(card.contained[7], 53);
    }

    #[test]
    fn parse_example2() {
        let card: Card = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"
            .parse()
            .unwrap();
        assert_eq!(card.id, 2);
        assert_eq!(card.winning[0], 13);
        assert_eq!(card.winning[1], 32);
        assert_eq!(card.winning[2], 20);
        assert_eq!(card.winning[3], 16);
        assert_eq!(card.winning[4], 61);
        assert_eq!(card.contained[0], 61);
        assert_eq!(card.contained[1], 30);
        assert_eq!(card.contained[2], 68);
        assert_eq!(card.contained[3], 82);
        assert_eq!(card.contained[4], 17);
        assert_eq!(card.contained[5], 32);
        assert_eq!(card.contained[6], 24);
        assert_eq!(card.contained[7], 19);
    }

    #[test]
    fn parse_example3() {
        let card: Card = "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"
            .parse()
            .unwrap();
        assert_eq!(card.id, 3);
        assert_eq!(card.winning[0], 1);
        assert_eq!(card.winning[1], 21);
        assert_eq!(card.winning[2], 53);
        assert_eq!(card.winning[3], 59);
        assert_eq!(card.winning[4], 44);
        assert_eq!(card.contained[0], 69);
        assert_eq!(card.contained[1], 82);
        assert_eq!(card.contained[2], 63);
        assert_eq!(card.contained[3], 72);
        assert_eq!(card.contained[4], 16);
        assert_eq!(card.contained[5], 21);
        assert_eq!(card.contained[6], 14);
        assert_eq!(card.contained[7], 1);
    }

    #[test]
    fn parse_example4() {
        let card: Card = "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"
            .parse()
            .unwrap();
        assert_eq!(card.id, 4);
        assert_eq!(card.winning[0], 41);
        assert_eq!(card.winning[1], 92);
        assert_eq!(card.winning[2], 73);
        assert_eq!(card.winning[3], 84);
        assert_eq!(card.winning[4], 69);
        assert_eq!(card.contained[0], 59);
        assert_eq!(card.contained[1], 84);
        assert_eq!(card.contained[2], 76);
        assert_eq!(card.contained[3], 51);
        assert_eq!(card.contained[4], 58);
        assert_eq!(card.contained[5], 5);
        assert_eq!(card.contained[6], 54);
        assert_eq!(card.contained[7], 83);
    }

    #[test]
    fn parse_example5() {
        let card: Card = "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"
            .parse()
            .unwrap();
        assert_eq!(card.id, 5);
        assert_eq!(card.winning[0], 87);
        assert_eq!(card.winning[1], 83);
        assert_eq!(card.winning[2], 26);
        assert_eq!(card.winning[3], 28);
        assert_eq!(card.winning[4], 32);
        assert_eq!(card.contained[0], 88);
        assert_eq!(card.contained[1], 30);
        assert_eq!(card.contained[2], 70);
        assert_eq!(card.contained[3], 12);
        assert_eq!(card.contained[4], 93);
        assert_eq!(card.contained[5], 22);
        assert_eq!(card.contained[6], 82);
        assert_eq!(card.contained[7], 36);
    }

    #[test]
    fn parse_example6() {
        let card: Card = "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .parse()
            .unwrap();
        assert_eq!(card.id, 6);
        assert_eq!(card.winning[0], 31);
        assert_eq!(card.winning[1], 18);
        assert_eq!(card.winning[2], 13);
        assert_eq!(card.winning[3], 56);
        assert_eq!(card.winning[4], 72);
        assert_eq!(card.contained[0], 74);
        assert_eq!(card.contained[1], 77);
        assert_eq!(card.contained[2], 10);
        assert_eq!(card.contained[3], 23);
        assert_eq!(card.contained[4], 35);
        assert_eq!(card.contained[5], 67);
        assert_eq!(card.contained[6], 36);
        assert_eq!(card.contained[7], 11);
    }

    #[test]
    fn points_3() {
        let card: Card = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
            .parse()
            .unwrap();
        assert_eq!(card.points(), 8);
    }

    #[test]
    fn points_2() {
        let card: Card = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"
            .parse()
            .unwrap();
        assert_eq!(card.points(), 2);
    }

    #[test]
    fn points_none() {
        let card: Card = "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"
            .parse()
            .unwrap();
        assert_eq!(card.points(), 0);
    }
}
