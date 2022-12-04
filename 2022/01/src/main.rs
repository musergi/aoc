use std::{cmp, collections::BinaryHeap, fs};

#[derive(Clone)]
struct TopN {
    n: usize,
    calories: BinaryHeap<cmp::Reverse<i32>>,
}

impl TopN {
    fn new(n: usize) -> Self {
        Self {
            n,
            calories: BinaryHeap::new(),
        }
    }

    fn combine(&mut self, new: i32) {
        self.calories.push(cmp::Reverse(new));
        if self.calories.len() > self.n {
            self.calories.pop();
        }
    }
}

fn main() {
    let elf_calories: Vec<_> = fs::read_to_string("assets/input.txt")
        .expect("Input")
        .split("\n\n")
        .map(get_calorie_count)
        .collect();
    let max = elf_calories
        .iter()
        .reduce(|a, b| cmp::max(a, b))
        .unwrap_or(&0);
    println!("Max calories: {}", max);
    let mut top = TopN::new(3);
    for cal in elf_calories {
        top.combine(cal);
    }
    let addition = top
        .calories
        .into_iter()
        .map(|v| v.0)
        .reduce(|a, b| a + b)
        .unwrap_or(0);
    println!("Top 3 calories: {}", addition)
}

fn get_calorie_count(s: &str) -> i32 {
    s.split("\n")
        .map(|s| s.parse::<i32>().unwrap_or(0))
        .reduce(|a, b| a + b)
        .unwrap_or(0)
}
