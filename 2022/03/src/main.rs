use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug, Clone)]
struct Rucksack {
    content: String,
}

#[derive(Debug)]
enum RucksackError {
    InvalidLength,
    ImpossibleIntersection(usize),
    CastError,
}

trait CommonChar {
    fn get_common_char(&self) -> Result<char, RucksackError>;
    fn get_priority(&self) -> Result<u32, RucksackError> {
        let c = self.get_common_char()?;
        if c.is_ascii_lowercase() {
            Ok(c as u32 - 'a' as u32 + 1)
        } else if c.is_ascii_uppercase() {
            Ok(c as u32 - 'A' as u32 + 27)
        } else {
            Err(RucksackError::CastError)
        }
    }
}

impl Rucksack {
    fn new(content: String) -> Result<Rucksack, RucksackError> {
        if content.len() % 2 != 0 {
            Err(RucksackError::InvalidLength)
        } else {
            Ok(Rucksack { content })
        }
    }

    fn get_set(&self) -> HashSet<char> {
        self.content.chars().into_iter().collect()
    }
}

impl CommonChar for Rucksack {
    fn get_common_char(&self) -> Result<char, RucksackError> {
        let split_size = self.content.len() / 2;
        let char_vec = self.content.chars().collect::<Vec<_>>();
        let first = char_vec[..split_size]
            .iter()
            .cloned()
            .collect::<HashSet<_>>();
        let second = char_vec[split_size..]
            .iter()
            .cloned()
            .collect::<HashSet<_>>();
        let intersection = first.intersection(&second).cloned().collect::<Vec<_>>();
        if intersection.len() != 1 {
            Err(RucksackError::ImpossibleIntersection(intersection.len()))
        } else {
            Ok(*intersection.get(0).unwrap())
        }
    }
}

struct ElfGroup {
    rucksacks: [Rucksack; 3],
}

impl ElfGroup {
    fn from_ruckstacks<I>(rucksacks: I) -> Vec<ElfGroup>
    where
        I: IntoIterator<Item = Rucksack>,
    {
        let mut groups = Vec::new();
        let mut new: Vec<_> = Vec::new();
        for rucksack in rucksacks {
            new.push(rucksack);
            if new.len() == 3 {
                groups.push(ElfGroup {
                    rucksacks: [new.pop().unwrap(), new.pop().unwrap(), new.pop().unwrap()],
                })
            }
        }
        groups
    }
}

impl CommonChar for ElfGroup {
    fn get_common_char(&self) -> Result<char, RucksackError> {
        let intersection = self
            .rucksacks
            .iter()
            .map(|r| r.get_set())
            .reduce(|a, b| a.intersection(&b).cloned().collect::<HashSet<_>>())
            .ok_or(RucksackError::ImpossibleIntersection(0))?;
        if intersection.len() != 1 {
            Err(RucksackError::ImpossibleIntersection(intersection.len()))
        } else {
            Ok(*intersection.iter().next().unwrap())
        }
    }
}

fn main() {
    let f = File::open("assets/input.txt").expect("File");
    let rucksacks = io::BufReader::new(f)
        .lines()
        .map(|l| Rucksack::new(l.expect("Line").to_string()).expect("Rucksack"))
        .collect::<Vec<_>>();
    let total = rucksacks
        .iter()
        .map(|r| r.get_priority().expect("Priority"))
        .reduce(|a, b| a + b)
        .unwrap_or(0);
    println!("Total priority: {}", total);
    let total = ElfGroup::from_ruckstacks(rucksacks)
        .iter()
        .map(|g| g.get_priority().expect("Priority"))
        .reduce(|a, b| a + b)
        .unwrap_or(0);
    println!("Group total: {}", total);
}
