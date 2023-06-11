#[derive(Debug, Clone)]
struct Blueprint {
    ore_cost: Cost,
    clay_cost: Cost,
    obsidian_cost: Cost,
    geode_cost: Cost,
}

impl Blueprint {
    fn max_geodes(&self, depth: u8) -> u32 {
        let mut stack = Vec::new();
        stack.push(State::default());
        let mut best: Option<State> = None;
        while let Some(state) = stack.pop() {
            if state.minute == depth {
                if let Some(best_state) = best {
                    if state.geodes > best_state.geodes {
                        println!("New best: {:?}", state);
                        best = Some(state);
                    } else {
                        best = Some(best_state);
                    }
                } else {
                    best = Some(state);
                }
            } else {
                match &best {
                    Some(best) => {
                        if state.can_improve(best.geodes, depth) {
                            let mut new_states = state.get_states(&self);
                            stack.append(&mut new_states);
                        }
                    }
                    None => {
                        let mut new_states = state.get_states(&self);
                        stack.append(&mut new_states);
                    }
                }
            }
        }
        best.unwrap().geodes
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct CachedState {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geodes: u32,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
}

impl From<State> for CachedState {
    fn from(
        State {
            ore,
            clay,
            obsidian,
            geodes,
            ore_robots,
            clay_robots,
            obsidian_robots,
            geode_robots,
            ..
        }: State,
    ) -> Self {
        Self {
            ore,
            clay,
            obsidian,
            geodes,
            ore_robots,
            clay_robots,
            obsidian_robots,
            geode_robots,
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    minute: u8,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geodes: u32,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            minute: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }
    }
}

impl State {
    fn get_states(&self, blueprint: &Blueprint) -> Vec<State> {
        let mut states = Vec::new();
        let template = self.do_nothing_state();
        states.push(template.clone());
        if self.can_build(&blueprint.ore_cost) && self.ore_makes_sense(blueprint) {
            let mut new = template.with_cost(&blueprint.ore_cost);
            new.ore_robots += 1;
            states.push(new);
        }
        if self.can_build(&blueprint.clay_cost) && self.clay_makes_sense(blueprint) {
            let mut new = template.with_cost(&blueprint.clay_cost);
            new.clay_robots += 1;
            states.push(new);
        }
        if self.can_build(&blueprint.obsidian_cost) && self.obsidian_make_sense(blueprint) {
            let mut new = template.with_cost(&blueprint.obsidian_cost);
            new.obsidian_robots += 1;
            states.push(new);
        }
        if self.can_build(&blueprint.geode_cost) {
            let mut new = template.with_cost(&blueprint.geode_cost);
            new.geode_robots += 1;
            states.push(new);
        }
        states
    }

    fn ore_makes_sense(&self, blueprint: &Blueprint) -> bool {
        let max_ore_cost = [
            &blueprint.ore_cost,
            &blueprint.clay_cost,
            &blueprint.obsidian_cost,
            &blueprint.geode_cost,
        ]
        .into_iter()
        .map(|cost| cost.ore)
        .max()
        .unwrap();
        self.ore_robots < max_ore_cost
    }

    fn clay_makes_sense(&self, blueprint: &Blueprint) -> bool {
        self.clay_robots < blueprint.obsidian_cost.clay
    }

    fn obsidian_make_sense(&self, blueprint: &Blueprint) -> bool {
        self.obsidian_robots < blueprint.geode_cost.obsidian
    }

    fn can_improve(&self, geodes: u32, max_minutes: u8) -> bool {
        let remaining = (max_minutes - self.minute) as u32;
        let current_robots_profit = self.geode_robots * remaining;
        // We assume we can build a robot each turn
        let new_robots_profit = remaining * (remaining - 1) / 2;
        self.geodes + current_robots_profit + new_robots_profit > geodes
    }

    fn with_cost(&self, cost: &Cost) -> Self {
        let mut new = self.clone();
        new.ore -= cost.ore;
        new.clay -= cost.clay;
        new.obsidian -= cost.obsidian;
        new
    }

    fn can_build(&self, cost: &Cost) -> bool {
        self.ore >= cost.ore && self.clay >= cost.clay && self.obsidian >= cost.obsidian
    }

    fn do_nothing_state(&self) -> Self {
        let mut template = self.clone();
        template.minute += 1;
        template.ore += self.ore_robots;
        template.clay += self.clay_robots;
        template.obsidian += self.obsidian_robots;
        template.geodes += self.geode_robots;
        template
    }
}

impl From<&str> for Blueprint {
    fn from(value: &str) -> Self {
        let mut it = value.split(":");
        it.next().unwrap();
        let mut it = it.next().unwrap().trim().split(".");
        let ore_cost = Cost::from(it.next().unwrap().trim());
        let clay_cost = Cost::from(it.next().unwrap().trim());
        let obsidian_cost = Cost::from(it.next().unwrap().trim());
        let geode_cost = Cost::from(it.next().unwrap().trim());
        Self {
            ore_cost,
            clay_cost,
            obsidian_cost,
            geode_cost,
        }
    }
}

#[derive(Debug, Clone)]
struct Cost {
    ore: u32,
    clay: u32,
    obsidian: u32,
}

impl From<&str> for Cost {
    fn from(value: &str) -> Self {
        let mut it = value.split_whitespace();
        it.next().unwrap();
        it.next().unwrap();
        it.next().unwrap();
        it.next().unwrap();
        let ore = it.next().unwrap().parse().unwrap();
        let mut cost = Cost {
            ore,
            clay: 0,
            obsidian: 0,
        };
        if let Some(additional) = Additional::new(it.collect()) {
            match additional {
                Additional::Clay(v) => cost.clay = v,
                Additional::Obsidian(v) => cost.obsidian = v,
            };
        }
        cost
    }
}

enum Additional {
    Clay(u32),
    Obsidian(u32),
}

impl Additional {
    fn new(value: Vec<&str>) -> Option<Self> {
        let v = value.get(2)?.parse().unwrap();
        Some(match *value.get(3).unwrap() {
            "clay" => Self::Clay(v),
            "obsidian" => Self::Obsidian(v),
            _ => panic!("Unknown material"),
        })
    }
}

fn main() {
    let s = std::fs::read_to_string("assets/input.txt").unwrap();
    let blueprints: Vec<_> = s.lines().map(|line| Blueprint::from(line)).collect();
    let part1: u32 = blueprints
        .iter()
        .enumerate()
        .map(|(idx, blueprint)| (idx + 1) as u32 * blueprint.max_geodes(24))
        .sum();
    println!("Part 1: {}", part1);
    let part2 = blueprints
        .iter()
        .take(3)
        .map(|blueprint| blueprint.max_geodes(32))
        .reduce(|a, b| a * b)
        .unwrap();
    println!("Part 2: {}", part2);
}
