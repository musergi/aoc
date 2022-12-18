use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Packet {
    List(Vec<Packet>),
    Integer(i32),
}

impl Packet {
    pub fn first() -> Packet {
        Packet::List(vec![Packet::List(vec![Packet::Integer(2)])])
    }

    pub fn second() -> Packet {
        Packet::List(vec![Packet::List(vec![Packet::Integer(6)])])
    }
}

#[derive(Debug)]
pub enum PacketParseError {
    UnclosedBracket(String),
    ExcesiveClosing(String),
    NumberWithoutList(String),
    ParseIntError(ParseIntError),
}

impl From<ParseIntError> for PacketParseError {
    fn from(err: ParseIntError) -> Self {
        PacketParseError::ParseIntError(err)
    }
}

impl FromStr for Packet {
    type Err = PacketParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res = None;
        let mut stack = Vec::new();
        for frag in s.split(",") {
            let open_count = frag.chars().take_while(|c| *c == '[').count();
            for _ in 0..open_count {
                stack.push(Packet::List(Vec::new()));
            }
            let close_count = frag.chars().rev().take_while(|c| *c == ']').count();
            if let Packet::List(l) = stack
                .last_mut()
                .ok_or(PacketParseError::NumberWithoutList(s.to_string()))?
            {
                let substr = &frag[open_count..(frag.len() - close_count)];
                if !substr.is_empty() {
                    l.push(Packet::Integer(substr.parse::<i32>()?))
                }
            }
            for _ in 0..close_count {
                let val = stack
                    .pop()
                    .ok_or(PacketParseError::UnclosedBracket(s.to_string()))?;
                if let Some(Packet::List(l)) = stack.last_mut() {
                    l.push(val)
                } else {
                    if res.is_some() {
                        return Err(PacketParseError::ExcesiveClosing(s.to_string()));
                    } else {
                        res = Some(val);
                    }
                }
            }
        }
        res.ok_or(PacketParseError::UnclosedBracket(s.to_string()))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let res =  match (self, other) {
            (Packet::List(l), Packet::List(r)) => l.cmp(r),
            (Packet::List(l), Packet::Integer(r)) => l.cmp(&vec![Packet::Integer(r.clone())]),
            (Packet::Integer(l), Packet::List(r)) => vec![Packet::Integer(l.clone())].cmp(r),
            (Packet::Integer(l), Packet::Integer(r)) => l.cmp(r),
        };
        Some(res)
    }
}