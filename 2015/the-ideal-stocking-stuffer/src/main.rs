use std::fmt::Display;

use md5::hash;

fn main() {
    aoc::aoc_main(aoc::input!(), part1, part2);
}

fn part1(input: &str) -> impl Display {
    find_zeros(input, 5)
}

fn part2(input: &str) -> impl Display {
    find_zeros(input, 6)
}

fn find_zeros(input: &str, hex_digit_count: u32) -> impl Display {
    let bit_count = hex_digit_count * 4;
    let mut i = 0;
    loop {
        let s = format!("{}{}", input, i);
        let v = hash(s.clone().into_bytes());
        if v.leading_zeros() >= bit_count {
            return i;
        }
        i += 1;
    }
}

mod md5;

#[cfg(test)]
mod tests {
    use crate::{part1, md5::hash};

    #[test]
    fn example1() {
        assert_eq!(format!("{}", part1("abcdef")), "609043");
    }

    #[test]
    fn example2() {
        assert_eq!(format!("{}", part1("pqrstuv")), "1048970");
    }
}
