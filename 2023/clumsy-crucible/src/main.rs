use map::{LongPath, Map, Path, ShortPath};
use std::fmt::Display;

mod dijkstra;
mod map;

fn main() {
    aoc::aoc_main(aoc::input!(), part1, part2)
}

fn part1(input: &str) -> impl Display {
    common::<ShortPath>(input)
}

fn part2(input: &str) -> impl Display {
    common::<LongPath>(input)
}

fn common<P: Path>(input: &str) -> impl Display {
    input
        .parse::<Map>()
        .map(|map| map.heat_loss::<P>())
        .map(|value| {
            value
                .map(|v| v.to_string())
                .unwrap_or("no path".to_string())
        })
        .unwrap_or_else(|err| err.to_string())
}
