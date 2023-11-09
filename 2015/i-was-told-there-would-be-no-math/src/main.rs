use std::fmt::Display;

fn main() {
    aoc::aoc_main(aoc::input!(), part1, part2);
}

fn part1(input: &str) -> impl Display {
    present_calc(input, wrap_size)
}

fn part2(input: &str) -> impl Display {
    present_calc(input, ribbon_size)
}

fn present_calc(input: &str, func: fn([u32; 3]) -> u32) -> impl Display {
    input
        .lines()
        .map(|l| parse_line(l))
        .map(|sides| sides.map(func))
        .sum::<Option<u32>>()
        .map(|v| v.to_string())
        .unwrap_or("Invalid Input".to_string())
}

const SIDES: usize = 3;

fn parse_line(description: &str) -> Option<[u32; 3]> {
    let mut it = description.split('x');
    let strs = [
        it.next()?.parse().ok()?,
        it.next()?.parse().ok()?,
        it.next()?.parse().ok()?,
    ];
    if it.next().is_some() {
        return None;
    }
    Some(strs)
}

fn wrap_size(sides: [u32; 3]) -> u32 {
    let faces: Vec<_> = (0..SIDES)
        .into_iter()
        .map(|idx| sides[idx] * sides[(idx + 1) % SIDES])
        .collect();
    faces.iter().map(|side| side * 2).sum::<u32>() + faces.iter().min().unwrap()
}

fn ribbon_size(mut sides: [u32; 3]) -> u32 {
    sides.sort();
    sides[0] * 2 + sides[1] * 2 + sides.into_iter().fold(1, |a, b| a * b)
}

#[cfg(test)]
mod tests {
    use crate::{parse_line, ribbon_size, wrap_size};

    #[test]
    fn parse_line1() {
        assert_eq!(parse_line("2x3x4").unwrap(), [2, 3, 4]);
    }

    #[test]
    fn parse_line2() {
        assert_eq!(parse_line("1x1x10").unwrap(), [1, 1, 10]);
    }

    #[test]
    fn wrap_size_case1() {
        assert_eq!(wrap_size([2, 3, 4]), 58);
    }

    #[test]
    fn wrap_size_case2() {
        assert_eq!(wrap_size([1, 1, 10]), 43);
    }

    #[test]
    fn ribbon_size_case1() {
        assert_eq!(ribbon_size([2, 3, 4]), 34);
    }

    #[test]
    fn ribbon_size_case2() {
        assert_eq!(ribbon_size([1, 1, 10]), 14);
    }
}
