use crate::polygon::Polygon;
use crate::{instruction::Instruction, point::Point};
use std::collections::BTreeSet;

pub struct DigPlan {
    instructions: Vec<Instruction>,
}

impl DigPlan {
    pub fn cubic_meters(&self) -> usize {
        let polygon = self.polygon();
        let xs: BTreeSet<i64> = polygon.points.iter().map(|p| p.x).collect();
        let ys: BTreeSet<i64> = polygon.points.iter().map(|p| p.y).collect();
        let x_ranges = to_ranges(xs);
        let y_ranges = to_ranges(ys);
        x_ranges
            .iter()
            .map(|x_range| {
                y_ranges
                    .iter()
                    .filter(|&y_range| polygon.is_in(&Point::new(x_range.0, y_range.0)))
                    .map(|&y_range| {
                        ((y_range.1 - y_range.0 + 1) * (x_range.1 - x_range.0 + 1)) as usize
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    fn polygon(&self) -> Polygon {
        let mut points = vec![Point::new(0, 0)];
        for instruction in self.instructions.iter() {
            let point = instruction.apply(points.last().unwrap());
            points.push(point);
        }
        Polygon { points }
    }

    pub fn parse_v1(s: &str) -> Result<Self, &'static str> {
        DigPlan::parse(s, Instruction::parse_v1)
    }

    pub fn parse_v2(s: &str) -> Result<Self, &'static str> {
        DigPlan::parse(s, Instruction::parse_v2)
    }

    pub fn parse(
        s: &str,
        parse: fn(&str) -> Result<Instruction, &'static str>,
    ) -> Result<Self, &'static str> {
        let instructions = s.lines().map(parse).collect::<Result<_, _>>()?;
        Ok(DigPlan { instructions })
    }
}

fn to_ranges(values: BTreeSet<i64>) -> Vec<(i64, i64)> {
    let mut ranges: Vec<_> = values.iter().map(|v| (*v, *v)).collect();
    ranges.sort();
    let missing_ranges: Vec<_> = ranges[..ranges.len() - 1]
        .iter()
        .zip(ranges[1..].iter())
        .map(|((first, _), (second, _))| (*first + 1, *second - 1))
        .filter(|(first, second)| first <= second)
        .collect();
    ranges.extend(missing_ranges);
    ranges.sort();
    ranges
}

#[cfg(test)]
mod tests {
    use crate::{dig::DigPlan, point::Point};

    const EXAMPLE: &str = include_str!("../assets/example.txt");

    #[test]
    fn example() {
        let dig_plan: DigPlan = DigPlan::parse_v1(EXAMPLE).unwrap();
        assert_eq!(dig_plan.cubic_meters(), 62);
    }

    #[test]
    fn in_line_is_in() {
        let dig_plan: DigPlan = DigPlan::parse_v1(EXAMPLE).unwrap();
        let polygon = dig_plan.polygon();
        assert!(polygon.is_in(&Point::new(0, 0)));
    }

    #[test]
    fn example_part2() {
        let dig_plan: DigPlan = DigPlan::parse_v2(EXAMPLE).unwrap();
        assert_eq!(dig_plan.cubic_meters(), 952408144115);
    }
}
