use std::str::FromStr;

pub struct History {
    values: Vec<i32>,
}

impl History {
    pub fn predict_next(&self) -> i32 {
        let value = self
            .build_stack()
            .iter()
            .rev()
            .fold(0, |acc, new| new.last().unwrap() + acc);
        self.values.last().unwrap() + value
    }

    pub fn predict_previous(&self) -> i32 {
        let value = self
            .build_stack()
            .iter()
            .rev()
            .fold(0, |acc, new| new.first().unwrap() - acc);
        self.values.first().unwrap() - value
    }

    fn build_stack(&self) -> Vec<Vec<i32>> {
        let mut stack = vec![History::diffs(&self.values)];
        while !stack.last().unwrap().iter().all(|&value| value == 0) {
            stack.push(Self::diffs(stack.last().unwrap()));
        }
        stack
    }

    fn diffs(previous: &[i32]) -> Vec<i32> {
        let mut output = Vec::with_capacity(previous.len() - 1);
        for (element, next) in previous[..previous.len() - 1]
            .iter()
            .zip(previous[1..].iter())
        {
            output.push(next - element);
        }
        output
    }
}

impl FromStr for History {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .split_whitespace()
            .map(|str| str.parse::<i32>().map_err(|_| "invalid value"))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(History { values })
    }
}

#[cfg(test)]
mod tests {
    use super::History;

    #[test]
    fn example1() {
        let string = "0 3 6 9 12 15";
        let values: History = string.parse().unwrap();
        assert_eq!(values.predict_next(), 18);
    }

    #[test]
    fn example2() {
        let string = "1 3 6 10 15 21";
        let values: History = string.parse().unwrap();
        assert_eq!(values.predict_next(), 28);
    }

    #[test]
    fn example3() {
        let string = "10 13 16 21 30 45";
        let values: History = string.parse().unwrap();
        assert_eq!(values.predict_next(), 68);
    }

    #[test]
    fn example_previous() {
        let string = "10 13 16 21 30 45";
        let values: History = string.parse().unwrap();
        assert_eq!(values.predict_previous(), 5);
    }
}
