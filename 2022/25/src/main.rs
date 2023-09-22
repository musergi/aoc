fn main() {
    let s = std::fs::read_to_string("assets/input.txt").unwrap();
    println!("Part 1: {}", part1(&s));
}

fn part1(s: &str) -> String {
    let val = s
        .lines()
        .map(|line| line.chars().map(SnafuPengit::from).collect::<Vec<_>>())
        .map(|n| to_decimal(&n))
        .sum();
    to_snafu(val).into_iter().map(char::from).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SnafuPengit {
    Two,
    One,
    Zero,
    Minus,
    Equal,
}

impl From<SnafuPengit> for char {
    fn from(value: SnafuPengit) -> Self {
        match value {
            SnafuPengit::Two => '2',
            SnafuPengit::One => '1',
            SnafuPengit::Zero => '0',
            SnafuPengit::Minus => '-',
            SnafuPengit::Equal => '=',
        }
    }
}

impl From<char> for SnafuPengit {
    fn from(value: char) -> Self {
        match value {
            '2' => Self::Two,
            '1' => Self::One,
            '0' => Self::Zero,
            '-' => Self::Minus,
            '=' => Self::Equal,
            _ => panic!("invalid digit"),
        }
    }
}

impl From<SnafuPengit> for i64 {
    fn from(value: SnafuPengit) -> Self {
        match value {
            SnafuPengit::Two => 2,
            SnafuPengit::One => 1,
            SnafuPengit::Zero => 0,
            SnafuPengit::Minus => -1,
            SnafuPengit::Equal => -2,
        }
    }
}

fn to_decimal(n: &[SnafuPengit]) -> i64 {
    n.iter()
        .rev()
        .enumerate()
        .map(|(idx, s)| i64::from(*s) * 5i64.pow(idx as u32))
        .sum()
}

fn to_snafu(n: i64) -> Vec<SnafuPengit> {
    let mut start = vec![SnafuPengit::Two];
    while to_decimal(&start) < n {
        start.push(SnafuPengit::Two);
    }
    for i in 0..start.len() {
        let mut last = SnafuPengit::Two;
        for other in [
            SnafuPengit::One,
            SnafuPengit::Zero,
            SnafuPengit::Minus,
            SnafuPengit::Equal,
        ] {
            *start.get_mut(i).unwrap() = other;
            if to_decimal(&start) < n {
                break;
            }
            last = other;
        }
        *start.get_mut(i).unwrap() = last;
    }
    start
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_one() {
        assert_eq!(to_snafu(1), vec![SnafuPengit::One]);
    }

    #[test]
    fn convert_two() {
        assert_eq!(to_snafu(2), vec![SnafuPengit::Two]);
    }

    #[test]
    fn convert_three() {
        assert_eq!(to_snafu(3), vec![SnafuPengit::One, SnafuPengit::Equal]);
    }

    #[test]
    fn convert_four() {
        assert_eq!(to_snafu(4), vec![SnafuPengit::One, SnafuPengit::Minus]);
    }

    #[test]
    fn convert_five() {
        assert_eq!(to_snafu(5), vec![SnafuPengit::One, SnafuPengit::Zero]);
    }

    #[test]
    fn convert_six() {
        assert_eq!(to_snafu(6), vec![SnafuPengit::One, SnafuPengit::One]);
    }

    #[test]
    fn convert_seven() {
        assert_eq!(to_snafu(7), vec![SnafuPengit::One, SnafuPengit::Two]);
    }

    #[test]
    fn convert_eight() {
        assert_eq!(to_snafu(8), vec![SnafuPengit::Two, SnafuPengit::Equal]);
    }

    #[test]
    fn convert_nine() {
        assert_eq!(to_snafu(9), vec![SnafuPengit::Two, SnafuPengit::Minus]);
    }

    #[test]
    fn convert_ten() {
        assert_eq!(to_snafu(10), vec![SnafuPengit::Two, SnafuPengit::Zero]);
    }

    #[test]
    fn convert_fifteen() {
        assert_eq!(
            to_snafu(15),
            vec![SnafuPengit::One, SnafuPengit::Equal, SnafuPengit::Zero]
        );
    }

    #[test]
    fn convert_twenty() {
        assert_eq!(
            to_snafu(20),
            vec![SnafuPengit::One, SnafuPengit::Minus, SnafuPengit::Zero]
        );
    }

    #[test]
    fn convert_2022() {
        assert_eq!(
            to_snafu(2022),
            vec![
                SnafuPengit::One,
                SnafuPengit::Equal,
                SnafuPengit::One,
                SnafuPengit::One,
                SnafuPengit::Minus,
                SnafuPengit::Two
            ]
        );
    }

    #[test]
    fn convert_12345() {
        assert_eq!(
            to_snafu(12345),
            vec![
                SnafuPengit::One,
                SnafuPengit::Minus,
                SnafuPengit::Zero,
                SnafuPengit::Minus,
                SnafuPengit::Minus,
                SnafuPengit::Minus,
                SnafuPengit::Zero,
            ]
        );
    }

    #[test]
    fn convert_314159265() {
        assert_eq!(
            to_snafu(314159265),
            vec![
                SnafuPengit::One,
                SnafuPengit::One,
                SnafuPengit::Two,
                SnafuPengit::One,
                SnafuPengit::Minus,
                SnafuPengit::One,
                SnafuPengit::One,
                SnafuPengit::One,
                SnafuPengit::Zero,
                SnafuPengit::Minus,
                SnafuPengit::One,
                SnafuPengit::Equal,
                SnafuPengit::Zero,
            ]
        );
    }

    #[test]
    fn example_part1() {
        let s = include_str!("../assets/example.txt");
        assert_eq!(part1(s), "2=-1=0")
    }
}
