fn main() {
    let s = std::fs::read_to_string("assets/input.txt").expect("file not found");
    println!("Part 1: {}", part1(&s));
    println!("Part 2: {}", part2(&s));
}

fn part1(s: &str) -> i64 {
    let mut nums: Vec<(usize, i64)> = s.lines().map(|l| l.parse().unwrap()).enumerate().collect();
    for num in 0..nums.len() {
        shuffle(&mut nums, num);
    }
    let start = nums
        .iter()
        .position(|&v| v.1 == 0)
        .expect("start not found");
    (1..=3)
        .into_iter()
        .map(|v| v * 1000)
        .map(|v| start + v)
        .map(|v| v % nums.len())
        .map(|idx| nums.get(idx).expect("idx not found").1)
        .sum()
}

fn part2(s: &str) -> i64 {
    let mut nums: Vec<_> = s
        .lines()
        .map(|l| l.parse().unwrap())
        .map(|v: i64| v * 811589153)
        .enumerate()
        .collect();
    for _ in 0..10 {
        for num in 0..nums.len() {
            shuffle(&mut nums, num);
        }
    }
    let start = nums
        .iter()
        .position(|&v| v.1 == 0)
        .expect("start not found");
    (1..=3)
        .into_iter()
        .map(|v| v * 1000)
        .map(|v| start + v)
        .map(|v| v % nums.len())
        .map(|idx| nums.get(idx).expect("idx not found").1)
        .sum()
}

fn shuffle(nums: &mut Vec<(usize, i64)>, num: usize) {
    let mut idx = nums
        .iter()
        .position(|&v| v.0 == num)
        .expect("number not found");
    let wrap = nums.len() as i64;
    let mut destination = idx as i64 + nums.get(idx).unwrap().1;
    if destination < 0 {
        destination += (wrap - 1) * (-destination / wrap);
        while destination < 0 {
            destination += wrap - 1;
        }
    } else if destination >= wrap {
        destination -= (wrap - 1) * (destination / wrap);
        while destination >= wrap {
            destination -= wrap - 1;
        }
    }

    let movement = destination - idx as i64;
    let dist = movement.abs();
    let dir = movement.signum();
    for _ in 0..dist {
        let next = (idx as i64 + dir) as usize;
        nums.swap(idx, next);
        idx = next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_movement() {
        let mut nums = vec![4, 5, 6, 1, 7, 8, 9].into_iter().enumerate().collect();
        shuffle(&mut nums, 3);
        assert_eq!(
            nums.into_iter().map(|v| v.1).collect::<Vec<_>>(),
            vec![4, 5, 6, 7, 1, 8, 9]
        );
    }

    #[test]
    fn negative_movement() {
        let mut nums = vec![4, -2, 5, 6, 7, 8, 9].into_iter().enumerate().collect();
        shuffle(&mut nums, 1);
        assert_eq!(
            nums.into_iter().map(|v| v.1).collect::<Vec<_>>(),
            vec![4, 5, 6, 7, 8, -2, 9]
        );
    }

    #[test]
    fn example_part1() {
        let s = include_str!("../assets/example.txt");
        assert_eq!(part1(s), 3);
    }

    #[test]
    fn example_part2() {
        let s = include_str!("../assets/example.txt");
        assert_eq!(part2(s), 1623178306);
    }
}
