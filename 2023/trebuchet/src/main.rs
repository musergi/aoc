use std::fmt::Display;

fn main() {
    aoc::aoc_main(aoc::input!(), part1, part2)
}

fn part1(input: &str) -> impl Display {
    parse_sum(input, parse_line_part1)
}

fn part2(input: &str) -> impl Display {
    parse_sum(input, parse_line_part2)
}

fn parse_sum(input: &str, parser: fn(&str) -> Result<u32, &'static str>) -> impl Display {
    input
    .lines()
    .map(parser)
    .sum::<Result<u32, _>>()
    .map(|v| v.to_string())
    .unwrap_or_else(|err| err.to_string())
}

fn parse_line_part1(line: &str) -> Result<u32, &'static str> {
    let mut it = line.chars().filter(|c| c.is_numeric());
    let first = it.next().ok_or_else(|| "invalid line")?;
    let last = it
        .last()
        .or_else(|| line.chars().filter(|c| c.is_numeric()).next())
        .unwrap();
    Ok(String::from_iter([first, last]).parse().unwrap())
}

const CHAR_DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

const WRITTEN_DIGITS: [&'static str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parse_line_part2(line: &str) -> Result<u32, &'static str> {
    let mut left: Option<(usize, u32)> = None;
    for (digit, digit_text) in WRITTEN_DIGITS.iter().enumerate() {
        if let Some(new) = line.match_indices(digit_text).next() {
            if let Some(left) = &mut left {
                if new.0 < left.0 {
                    *left = (new.0, digit as u32);
                }
            } else {
                left = Some((new.0, digit as u32));
            }
        }
    }
    for (digit, digit_text) in CHAR_DIGITS.iter().enumerate() {
        if let Some(new) = line.match_indices(*digit_text).next() {
            if let Some(left) = &mut left {
                if new.0 < left.0 {
                    *left = (new.0, digit as u32);
                }
            } else {
                left = Some((new.0, digit as u32));
            }
        }
    }
    let mut right: Option<(usize, u32)> = None;
    for (digit, digit_text) in WRITTEN_DIGITS.iter().enumerate() {
        if let Some(new) = line.rmatch_indices(digit_text).next() {
            if let Some(right) = &mut right {
                if new.0 > right.0 {
                    *right = (new.0, digit as u32);
                }
            } else {
                right = Some((new.0, digit as u32));
            }
        }
    }
    for (digit, digit_text) in CHAR_DIGITS.iter().enumerate() {
        if let Some(new) = line.rmatch_indices(*digit_text).next() {
            if let Some(right) = &mut right {
                if new.0 > right.0 {
                    *right = (new.0, digit as u32);
                }
            } else {
                right = Some((new.0, digit as u32));
            }
        }
    }
    match (left, right) {
        (Some(left), Some(right)) => Ok(left.1 * 10 + right.1),
        _ => Err("invalid line"),
    }
}

#[cfg(test)]
mod tests {
    mod part1 {
        use crate::{parse_line_part1, part1};

        #[test]
        fn parse_line_start_end() {
            assert_eq!(parse_line_part1("1abc2").unwrap(), 12);
        }

        #[test]
        fn parse_line_middle() {
            assert_eq!(parse_line_part1("pqr3stu8vwx").unwrap(), 38);
        }

        #[test]
        fn parse_line_excess_numbers() {
            assert_eq!(parse_line_part1("a1b2c3d4e5f").unwrap(), 15);
        }

        #[test]
        fn parse_line_number_shortage() {
            assert_eq!(parse_line_part1("treb7uchet").unwrap(), 77);
        }

        #[test]
        fn parse_line_fail_on_no_number() {
            assert!(parse_line_part1("trebuchet").is_err());
        }

        #[test]
        fn example() {
            let example = include_str!("../assets/example1.txt");
            assert_eq!(format!("{}", part1(&example)), "142");
        }
    }

    mod part2 {
        use crate::{parse_line_part2, part2};

        #[test]
        fn parse_line_only_letters() {
            assert_eq!(parse_line_part2("two1nine").unwrap(), 29);
        }

        #[test]
        fn parse_line_more_than_2_words() {
            assert_eq!(parse_line_part2("eightwothree").unwrap(), 83);
        }

        #[test]
        fn parse_line_digit_mixing() {
            assert_eq!(parse_line_part2("abcone2threexyz").unwrap(), 13);
        }

        #[test]
        fn parse_line_shared_letter() {
            assert_eq!(parse_line_part2("xtwone3four").unwrap(), 24);
        }

        #[test]
        fn parse_line_digits() {
            assert_eq!(parse_line_part2("4nineeightseven2").unwrap(), 42);
        }

        #[test]
        fn parse_line_one_of_each() {
            assert_eq!(parse_line_part2("zoneight234").unwrap(), 14);
        }

        #[test]
        fn parse_line_only_digits() {
            assert_eq!(parse_line_part2("7pqrstsixteen").unwrap(), 76);
        }

        #[test]
        fn example() {
            let example = include_str!("../assets/example2.txt");
            assert_eq!(format!("{}", part2(&example)), "281");
        }
    }
}
