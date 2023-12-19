use std::str::FromStr;

pub struct Universe {
    galaxies: Vec<(i64, i64)>,
}

impl Universe {
    pub fn shortest_path_sum(mut self, factor: i64) -> i64 {
        self.expand(factor);
        self.galaxies
            .iter()
            .enumerate()
            .flat_map(|(idx, first)| {
                self.galaxies[idx + 1..]
                    .iter()
                    .map(move |second| (first, second))
            })
            .map(move |((first_x, first_y), (second_x, second_y))| {
                (first_x - second_x).abs() + (first_y - second_y).abs()
            })
            .sum()
    }

    fn expand(&mut self, factor: i64) {
        let empty_xs = self.get_empties(|(x, _)| *x);
        let empty_ys = self.get_empties(|(_, y)| *y);
        self.galaxies = self
            .galaxies
            .iter()
            .map(|&(x, y)| {
                (
                    x + (empty_xs.iter().filter(|&empty_x| *empty_x < x).count() as i64)
                        * (factor - 1),
                    y + (empty_ys.iter().filter(|&empty_y| *empty_y < y).count() as i64)
                        * (factor - 1),
                )
            })
            .collect()
    }

    fn get_empties(&self, func: fn(&(i64, i64)) -> i64) -> Vec<i64> {
        let min_x = self.galaxies.iter().map(func).min().unwrap();
        let max_x = self.galaxies.iter().map(func).max().unwrap();
        (min_x..max_x)
            .into_iter()
            .filter(|x| !self.galaxies.iter().any(|galaxy| func(galaxy) == *x))
            .collect()
    }
}

impl FromStr for Universe {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut galaxies = Vec::new();
        let mut y = 0;
        for line in s.lines() {
            let mut x = 0;
            for character in line.chars() {
                if character == '#' {
                    galaxies.push((x, y));
                }
                x += 1;
            }
            y += 1;
        }
        Ok(Universe { galaxies })
    }
}

#[cfg(test)]
mod tests {
    use super::Universe;

    #[test]
    fn example_factor2() {
        let string = include_str!("../assets/example.txt");
        let universe: Universe = string.parse().unwrap();
        assert_eq!(universe.shortest_path_sum(2), 374);
    }

    #[test]
    fn example_factor10() {
        let string = include_str!("../assets/example.txt");
        let universe: Universe = string.parse().unwrap();
        assert_eq!(universe.shortest_path_sum(10), 1030);
    }

    #[test]
    fn example_factor100() {
        let string = include_str!("../assets/example.txt");
        let universe: Universe = string.parse().unwrap();
        assert_eq!(universe.shortest_path_sum(100), 8410);
    }
}
