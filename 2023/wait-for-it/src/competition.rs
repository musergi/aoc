pub struct Competition {
    time: u32,
    record: u32,
}

impl Competition {
    pub fn new(time: u32, record: u32) -> Competition {
        Competition { time, record }
    }

    pub fn wining_move_count(&self) -> u32 {
        // let `d` be the distance
        // let `t` be the relase time
        // let `tl` be the competition time
        // d = (t * 1mm/s^2) (tl - t)
        // Like in the example
        // t = 1, tl = 7
        // d = 1 * 1 * (7 - 1) = 6
        // let `r` be the record
        // d > r => d - r > 0
        // -t^2 + tl * t - r > 0
        // Intersection at
        // t^2 - tl + t + r > 0
        let discriminant = self.time * self.time - 4 * self.record;
        let root: u32 = (discriminant as f32).sqrt() as u32;
        let negative_solution = (self.time - root) / 2;
        let positive_solution = (self.time + root) / 2;
        // Sweep a range because may not be exact square root
        let first_wining_num = (negative_solution - 1..=negative_solution + 1)
            .filter(|&t| self.beats_record(t))
            .next()
            .unwrap();
        let last_wining_num = (positive_solution - 1..=positive_solution + 1)
            .filter(|&t| self.beats_record(t))
            .last()
            .unwrap();
        last_wining_num - first_wining_num + 1
    }

    fn beats_record(&self, t: u32) -> bool {
        self.distance(t) > self.record
    }

    fn distance(&self, t: u32) -> u32 {
        self.time * t - t * t
    }
}

#[cfg(test)]
mod tests {
    use crate::competition::Competition;

    #[test]
    fn wining_move_count_example1() {
        assert_eq!(Competition::new(7, 9).wining_move_count(), 4);
    }

    #[test]
    fn wining_move_count_example2() {
        assert_eq!(Competition::new(15, 40).wining_move_count(), 8);
    }

    #[test]
    fn wining_move_count_example3() {
        assert_eq!(Competition::new(30, 200).wining_move_count(), 9);
    }
}
