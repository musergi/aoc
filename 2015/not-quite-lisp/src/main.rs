const INPUT_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/input.txt");

fn main() {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> impl std::fmt::Display {
    input.chars().into_iter().map(get_value).sum::<i32>()
}

fn part2(input: &str) -> impl std::fmt::Display {
    find_index(input)
        .map(|v| v.to_string())
        .unwrap_or("Not Found".to_string())
}

fn find_index(input: &str) -> Option<usize> {
    let mut acc = 0;
    for (idx, c) in input.chars().enumerate() {
        acc += get_value(c);
        if acc == -1 {
            return Some(idx + 1);
        }
    }
    None
}

fn get_value(c: char) -> i32 {
    match c {
        '(' => 1,
        ')' => -1,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    mod part1 {
        use crate::part1;

        #[test]
        fn symetric() {
            assert_eq!(part1("(())").to_string(), "0");
        }

        #[test]
        fn inmediatly_closed() {
            assert_eq!(part1("()()").to_string(), "0");
        }

        #[test]
        fn only_open() {
            assert_eq!(part1("(((").to_string(), "3");
        }

        #[test]
        fn majority_open() {
            assert_eq!(part1("(()(()(").to_string(), "3");
        }

        #[test]
        fn majority_open_start_closing() {
            assert_eq!(part1("))(((((").to_string(), "3");
        }

        #[test]
        fn first_up_basement() {
            assert_eq!(part1("())").to_string(), "-1");
        }

        #[test]
        fn first_down_basement() {
            assert_eq!(part1("))(").to_string(), "-1");
        }

        #[test]
        fn only_close() {
            assert_eq!(part1(")))").to_string(), "-3");
        }

        #[test]
        fn mix_negative() {
            assert_eq!(part1(")())())").to_string(), "-3");
        }
    }

    mod part2 {
        use crate::part2;

        #[test]
        fn direct() {
            assert_eq!(part2(")").to_string(), "1");
        }

        #[test]
        fn oscilate() {
            assert_eq!(part2("()())").to_string(), "5");
        }

        #[test]
        fn not_found() {
            assert_eq!(part2("()()").to_string(), "Not Found");
        }
    }
}
