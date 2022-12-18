use std::{fs, num::ParseIntError, str::FromStr};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn manhattan_distance(&self, other: &Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl FromStr for Position {
    type Err = PositionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(", ");
        let x = it
            .next()
            .ok_or(PositionParseError::MissingComponent)?
            .strip_prefix("x=")
            .ok_or(PositionParseError::MissingPrefix)?
            .parse::<i32>()?;
        let y = it
            .next()
            .ok_or(PositionParseError::MissingComponent)?
            .strip_prefix("y=")
            .ok_or(PositionParseError::MissingPrefix)?
            .parse::<i32>()?;
        Ok(Position { x, y })
    }
}

#[derive(Debug)]
enum PositionParseError {
    MissingComponent,
    MissingPrefix,
    ParseIntError(ParseIntError),
}

impl From<ParseIntError> for PositionParseError {
    fn from(err: ParseIntError) -> Self {
        PositionParseError::ParseIntError(err)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Sensor {
    position: Position,
    closest_becon: Position,
}

impl Sensor {
    fn in_range_y(&self, y: &i32) -> bool {
        let distance = self.position.manhattan_distance(&self.closest_becon);
        self.position.y.abs_diff(*y) <= distance
    }

    fn get_range_y(&self, y: &i32) -> Range {
        let distance = self.position.manhattan_distance(&self.closest_becon);
        let freedom = distance - self.position.y.abs_diff(*y);
        Range::new(self.position.x, freedom)
    }
}

impl FromStr for Sensor {
    type Err = SensorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s
            .strip_prefix("Sensor at ")
            .ok_or(SensorParseError::MissingPrefix)?
            .split(": closest beacon is at ");
        let position = it
            .next()
            .ok_or(SensorParseError::MissingPosition)?
            .parse()?;
        let closest_becon = it.next().ok_or(SensorParseError::MissingBecon)?.parse()?;
        Ok(Sensor {
            position,
            closest_becon,
        })
    }
}

#[derive(Debug)]
enum SensorParseError {
    MissingPrefix,
    MissingPosition,
    MissingBecon,
    PositionParseError(PositionParseError),
}

impl From<PositionParseError> for SensorParseError {
    fn from(err: PositionParseError) -> Self {
        SensorParseError::PositionParseError(err)
    }
}

#[derive(Debug, Clone)]
struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn new(c: i32, d: u32) -> Range {
        Range {
            start: c - d as i32,
            end: c + d as i32,
        }
    }

    fn union(&self, other: &Self) -> Range {
        if !self.overlaps(other) {
            panic!("Impossible to perform union");
        }
        Range {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.start - 1 <= other.end && self.end + 1 >= other.start
    }

    fn contains(&self, el: &i32) -> bool {
        self.start <= *el && self.end >= *el
    }
}

#[derive(Debug, Clone)]
struct Ranges {
    ranges: Vec<Range>,
}

impl Ranges {
    fn new() -> Ranges {
        Ranges { ranges: Vec::new() }
    }

    fn add(&mut self, new: Range) {
        self.ranges.push(new)
    }

    fn merge(&mut self) {
        self.ranges.sort_by_key(|r| r.start);
        let mut new: Vec<Range> = Vec::new();
        for range in self.ranges.iter() {
            if new.last().is_some() && new.last().unwrap().overlaps(range) {
                *new.last_mut().unwrap() = new.last_mut().unwrap().union(range)
            } else {
                new.push(range.clone());
            }
        }
        self.ranges = new;
    }

    fn contains(&self, el: &i32) -> bool {
        self.ranges.iter().any(|r| r.contains(el))
    }

    fn split(&mut self, el: &i32) {
        let idx = self.ranges.iter().position(|r| r.contains(el)).unwrap();
        let old = self.ranges.remove(idx);
        self.ranges.insert(
            idx,
            Range {
                start: el + 1,
                end: old.end,
            },
        );
        self.ranges.insert(
            idx,
            Range {
                start: old.start,
                end: el - 1,
            },
        );
    }
}

fn ranges_y(sensors: &Vec<Sensor>, y: &i32) -> Ranges {
    let mut ranges = Ranges::new();
    for sensor in sensors.iter() {
        if sensor.in_range_y(&y) {
            ranges.add(sensor.get_range_y(&y))
        }
    }
    ranges.merge();
    ranges
}

fn main() {
    let sensors = fs::read_to_string("assets/input.txt")
        .expect("Read file")
        .lines()
        .map(|l| l.parse::<Sensor>())
        .collect::<Result<Vec<_>, _>>()
        .expect("Parsed sensors");
    let y = 10;
    let mut ranges = ranges_y(&sensors, &y);
    for sensor in sensors.iter() {
        if sensor.position.y == y && ranges.contains(&sensor.position.x) {
            ranges.split(&sensor.position.x);
        }
        if sensor.closest_becon.y == y && ranges.contains(&sensor.closest_becon.x) {
            ranges.split(&sensor.closest_becon.x);
        }
    }
    println!(
        "Total tiles in y={}: {}",
        y,
        ranges
            .ranges
            .iter()
            .map(|r| r.end - r.start + 1)
            .sum::<i32>()
    );
    let max = 4000000;
    for y in 0..=max {
        if !ranges_y(&sensors, &y)
            .ranges
            .iter()
            .all(|r| r.start <= 0 && r.end >= max)
        {
            let x = ranges_y(&sensors, &y).ranges.iter().find_map(|r| {
                if r.contains(&0) {
                    Some(r.end + 1)
                } else {
                    None
                }
            }).unwrap();
            println!("{:?}, {}", ranges_y(&sensors, &y).ranges, y);
            println!("{}", (x as i64) * (max as i64) + (y as i64));
            break;
        }
    }
}
