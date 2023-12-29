use std::{collections::HashMap, str::FromStr};

const TARGET: usize = 1000000000;

#[derive(PartialEq, Eq)]
pub struct Platform {
    tiles: Vec<Tile>,
    rows: usize,
    columns: usize,
}

impl Platform {
    pub fn north_load(self) -> usize {
        let north = self.fall(North);
        north.top_load()
    }

    pub fn cycled_north_load(mut self) -> usize {
        let mut cache = HashMap::new();
        let mut idx = 0;
        loop {
            if let Some(cached) = cache.get(&self.tiles) {
                let period = idx - cached;
                if (idx + (TARGET - idx) / period * period) % TARGET == 0 {
                    break;
                }
            }
            cache.insert(self.tiles.clone(), idx);
            self = self.cycle();
            idx += 1;
        }
        self.top_load()
    }

    fn cycle(mut self) -> Self {
        self = self.fall(North);
        self = self.fall(West);
        self = self.fall(South);
        self = self.fall(East);
        self
    }

    fn fall(mut self, direction: impl Direction) -> Self {
        for fixed_idx in 0..direction.get_fixed_max(self.rows, self.columns) {
            let fall_ranges = self.fall_ranges(fixed_idx, &direction);
            for (start, end) in fall_ranges {
                let mut round_count = (start..end)
                    .into_iter()
                    .filter(|&variable_idx| {
                        direction.get(&self, fixed_idx, variable_idx) == Tile::Round
                    })
                    .count();
                for variable_idx in direction.get_range_iter(start, end) {
                    let new_tile = if round_count > 0 {
                        round_count -= 1;
                        Tile::Round
                    } else {
                        Tile::Empty
                    };
                    *direction.get_mut(&mut self, fixed_idx, variable_idx) = new_tile;
                }
            }
        }
        self
    }

    fn fall_ranges(&self, fixed_idx: usize, direction: &impl Direction) -> Vec<(usize, usize)> {
        let variable_max = direction.get_variable_max(self.rows, self.columns);
        let square_positions: Vec<_> = (0..variable_max)
            .filter(|&variable_idx| direction.get(self, fixed_idx, variable_idx) == Tile::Square)
            .collect();
        let mut fall_ranges = Vec::new();
        let first_square = square_positions.first().copied().unwrap_or(variable_max);
        if first_square > 0 {
            fall_ranges.push((0, first_square));
        }
        for (idx, square_position) in square_positions.iter().enumerate() {
            let range_start = square_position + 1;
            let range_end = square_positions
                .get(idx + 1)
                .copied()
                .unwrap_or(variable_max);
            if range_start + 1 < range_end {
                fall_ranges.push((range_start, range_end));
            }
        }
        fall_ranges
    }

    fn top_load(&self) -> usize {
        let mut total = 0;
        for column in 0..self.columns {
            for row in 0..self.rows {
                if self.get(row, column) == Tile::Round {
                    total += self.rows - row;
                }
            }
        }
        total
    }

    fn get(&self, row: usize, column: usize) -> Tile {
        *self.tiles.get(row * self.columns + column).unwrap()
    }

    fn get_mut(&mut self, row: usize, column: usize) -> &mut Tile {
        self.tiles.get_mut(row * self.columns + column).unwrap()
    }
}

impl std::fmt::Debug for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f)?;
        for row in 0..self.rows {
            for column in 0..self.columns {
                match self.get(row, column) {
                    Tile::Round => write!(f, "O")?,
                    Tile::Square => write!(f, "#")?,
                    Tile::Empty => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromStr for Platform {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows = 0;
        let mut columns = 0;
        let mut tiles = Vec::new();
        for line in s.lines() {
            columns = 0;
            for character in line.chars() {
                let tile = match character {
                    'O' => Ok(Tile::Round),
                    '#' => Ok(Tile::Square),
                    '.' => Ok(Tile::Empty),
                    _ => Err("invalid tile"),
                }?;
                tiles.push(tile);
                columns += 1;
            }
            rows += 1;
        }
        Ok(Platform {
            tiles,
            rows,
            columns,
        })
    }
}

trait Axis {
    fn get(platform: &Platform, fixed: usize, variable: usize) -> Tile;
    fn get_mut<'a>(platform: &'a mut Platform, fixed: usize, variable: usize) -> &'a mut Tile;
    fn get_fixed_max(rows: usize, columns: usize) -> usize;
    fn get_variable_max(rows: usize, columns: usize) -> usize;
}

trait Direction {
    type Item: Axis;

    fn get_range_iter(&self, start: usize, end: usize) -> impl Iterator<Item = usize>;

    fn get(&self, platform: &Platform, fixed: usize, variable: usize) -> Tile {
        Self::Item::get(platform, fixed, variable)
    }

    fn get_mut<'a>(
        &self,
        platform: &'a mut Platform,
        fixed: usize,
        variable: usize,
    ) -> &'a mut Tile {
        Self::Item::get_mut(platform, fixed, variable)
    }

    fn get_fixed_max(&self, rows: usize, columns: usize) -> usize {
        Self::Item::get_fixed_max(rows, columns)
    }

    fn get_variable_max(&self, rows: usize, columns: usize) -> usize {
        Self::Item::get_variable_max(rows, columns)
    }
}

struct VerticalAxis;

impl Axis for VerticalAxis {
    fn get(platform: &Platform, fixed: usize, variable: usize) -> Tile {
        platform.get(variable, fixed)
    }

    fn get_mut<'a>(platform: &'a mut Platform, fixed: usize, variable: usize) -> &'a mut Tile {
        platform.get_mut(variable, fixed)
    }

    fn get_fixed_max(_rows: usize, columns: usize) -> usize {
        columns
    }

    fn get_variable_max(rows: usize, _columns: usize) -> usize {
        rows
    }
}

struct North;

impl Direction for North {
    type Item = VerticalAxis;

    fn get_range_iter(&self, start: usize, end: usize) -> impl Iterator<Item = usize> {
        start..end
    }
}

struct South;

impl Direction for South {
    type Item = VerticalAxis;

    fn get_range_iter(&self, start: usize, end: usize) -> impl Iterator<Item = usize> {
        (start..end).rev()
    }
}

struct HorizontalAxis;

impl Axis for HorizontalAxis {
    fn get(platform: &Platform, fixed: usize, variable: usize) -> Tile {
        platform.get(fixed, variable)
    }

    fn get_mut<'a>(platform: &'a mut Platform, fixed: usize, variable: usize) -> &'a mut Tile {
        platform.get_mut(fixed, variable)
    }

    fn get_fixed_max(rows: usize, _columns: usize) -> usize {
        rows
    }

    fn get_variable_max(_rows: usize, columns: usize) -> usize {
        columns
    }
}

struct East;

impl Direction for East {
    type Item = HorizontalAxis;

    fn get_range_iter(&self, start: usize, end: usize) -> impl Iterator<Item = usize> {
        (start..end).rev()
    }
}

struct West;

impl Direction for West {
    type Item = HorizontalAxis;

    fn get_range_iter(&self, start: usize, end: usize) -> impl Iterator<Item = usize> {
        start..end
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Round,
    Square,
    Empty,
}

#[cfg(test)]
mod tests {
    use crate::platform::{North, South};

    use super::{Platform, West};

    const EXAMPLE: &str = include_str!("../assets/example.txt");
    const FALLEN: &str = include_str!("../assets/fallen.txt");

    #[test]
    fn part1() {
        let platform: Platform = EXAMPLE.parse().unwrap();
        assert_eq!(platform.north_load(), 136);
    }

    #[test]
    fn part2() {
        let platform: Platform = EXAMPLE.parse().unwrap();
        assert_eq!(platform.cycled_north_load(), 64);
    }

    #[test]
    fn fall() {
        let platform: Platform = EXAMPLE.parse().unwrap();
        assert_eq!(platform.fall(North), FALLEN.parse().unwrap());
    }

    #[test]
    fn opposite_fall_identity() {
        let mut platform: Platform = EXAMPLE.parse().unwrap();
        platform = platform.fall(North);
        platform = platform.fall(South);
        assert_eq!(platform.fall(North), FALLEN.parse().unwrap());
    }

    #[test]
    fn cycle() {
        let platform: Platform = EXAMPLE.parse().unwrap();
        assert_eq!(
            platform.cycle(),
            include_str!("../assets/cycle1.txt").parse().unwrap()
        );
    }

    #[test]
    fn two_step() {
        let mut platform: Platform = EXAMPLE.parse().unwrap();
        platform = platform.fall(North);
        platform = platform.fall(West);
        assert_eq!(
            platform,
            include_str!("../assets/west.txt").parse().unwrap()
        );
    }
}
