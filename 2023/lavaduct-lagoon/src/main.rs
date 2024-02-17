use std::fmt::Display;

use dig::DigPlan;

mod dig;
mod direction;
mod instruction;
mod point;
mod polygon;

fn main() {
    aoc::aoc_main(aoc::input!(), part1, part2)
}

fn part1(input: &str) -> impl Display {
    common(input, DigPlan::parse_v1)
}

fn part2(input: &str) -> impl Display {
    common(input, DigPlan::parse_v2)
}

fn common(input: &str, parse: fn(&str) -> Result<DigPlan, &'static str>) -> impl Display {
    parse(input)
        .map(|dig_plan| dig_plan.cubic_meters())
        .map(|value| value.to_string())
        .unwrap_or_else(|err| err.to_string())
}
