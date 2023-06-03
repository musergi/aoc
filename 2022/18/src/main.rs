use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Boulder {
    x: i64,
    y: i64,
    z: i64,
}

impl From<&str> for Boulder {
    fn from(value: &str) -> Self {
        let mut it = value.split(",");
        let x: i64 = it.next().unwrap().parse().unwrap();
        let y: i64 = it.next().unwrap().parse().unwrap();
        let z: i64 = it.next().unwrap().parse().unwrap();
        Self { x, y, z }
    }
}

impl Boulder {
    fn get_adjacent(&self) -> HashSet<Boulder> {
        let Boulder { x, y, z } = self;
        let mut adjacents = HashSet::new();
        adjacents.insert(Boulder {
            x: x - 1,
            y: *y,
            z: *z,
        });
        adjacents.insert(Boulder {
            x: x + 1,
            y: *y,
            z: *z,
        });
        adjacents.insert(Boulder {
            x: *x,
            y: y - 1,
            z: *z,
        });
        adjacents.insert(Boulder {
            x: *x,
            y: y + 1,
            z: *z,
        });
        adjacents.insert(Boulder {
            x: *x,
            y: *y,
            z: z - 1,
        });
        adjacents.insert(Boulder {
            x: *x,
            y: *y,
            z: z + 1,
        });
        adjacents
    }
}

struct FreeFaceChecker {
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
    z_min: i64,
    z_max: i64,
    exterior: HashSet<Boulder>,
}

impl FreeFaceChecker {
    fn new(boulders: &HashSet<Boulder>) -> Self {
        Self {
            x_min: boulders.iter().map(|b| b.x).min().unwrap(),
            x_max: boulders.iter().map(|b| b.x).max().unwrap(),
            y_min: boulders.iter().map(|b| b.y).min().unwrap(),
            y_max: boulders.iter().map(|b| b.y).max().unwrap(),
            z_min: boulders.iter().map(|b| b.z).min().unwrap(),
            z_max: boulders.iter().map(|b| b.z).max().unwrap(),
            exterior: HashSet::new(),
        }
    }

    fn check(
        &mut self,
        boulder: &Boulder,
        boulders: &HashSet<Boulder>,
        visited: &mut HashSet<Boulder>,
    ) -> bool {
        match self.exterior.contains(boulder) {
            true => true,
            false => {
                if self.is_outside(boulder) {
                    true
                } else {
                    visited.insert(boulder.clone());
                    let is_exterior = boulder
                        .get_adjacent()
                        .difference(visited)
                        .cloned()
                        .collect::<HashSet<_>>()
                        .difference(boulders)
                        .any(|adjacent| self.check(adjacent, boulders, visited));
                    if is_exterior {
                        self.exterior.insert(boulder.clone());
                    }
                    is_exterior
                }
            }
        }
    }

    fn is_outside(&self, boulder: &Boulder) -> bool {
        boulder.x < self.x_min
            || boulder.x > self.x_max
            || boulder.y < self.y_min
            || boulder.y > self.y_max
            || boulder.z < self.z_min
            || boulder.z > self.z_max
    }
}

fn main() {
    let input = std::fs::read_to_string("assets/input.txt").unwrap();
    let boulders: HashSet<_> = input.lines().map(|line| Boulder::from(line)).collect();
    let mut exposed_faces = 0;
    let mut outside_exposed_faces = 0;
    let mut free_face_checker = FreeFaceChecker::new(&boulders);
    for boulder in boulders.iter() {
        let adjacents = boulder.get_adjacent();
        for free_face in adjacents.difference(&boulders) {
            if free_face_checker.check(free_face, &boulders, &mut HashSet::new()) {
                outside_exposed_faces += 1;
            }
            exposed_faces += 1;
        }
    }
    println!("Part 1: {}", exposed_faces);
    println!("Part 2: {}", outside_exposed_faces);
}
