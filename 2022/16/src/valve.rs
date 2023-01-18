use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(PartialEq, Eq, Clone)]
pub struct ValveId {
    first: char,
    second: char,
}

#[derive(Debug)]
pub enum ParseValveIdError {
    TooShort(usize),
    TooLong(usize),
}

impl FromStr for ValveId {
    type Err = ParseValveIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.chars();
        let first = it.next().ok_or(ParseValveIdError::TooShort(s.len()))?;
        let second = it.next().ok_or(ParseValveIdError::TooShort(s.len()))?;
        match it.next() {
            None => Ok(ValveId { first, second }),
            Some(_) => Err(ParseValveIdError::TooLong(s.len())),
        }
    }
}

impl fmt::Debug for ValveId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ValveId({}{})", self.first, self.second)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ValveLineInfo {
    pub subject: ValveId,
    pub flow: u32,
    pub destinations: Vec<ValveId>,
}

impl FromStr for ValveLineInfo {
    type Err = ParseValveLineInfoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (sensor, list) = split_two(s, ";").ok_or(ParseValveLineInfoError::InvalidFormat)?;
        let (subject_s, flow_s) = split_two(
            sensor
                .strip_prefix("Valve ")
                .ok_or(ParseValveLineInfoError::InvalidFormat)?,
            " has flow rate=",
        )
        .ok_or(ParseValveLineInfoError::InvalidFormat)?;
        let subject = subject_s.parse()?;
        let flow = flow_s.parse()?;
        let destinations = list
            .strip_prefix(" tunnels lead to valves ")
            .or(list.strip_prefix(" tunnel leads to valve "))
            .ok_or(ParseValveLineInfoError::InvalidFormat)?
            .split(", ")
            .map(|ids| ids.parse::<ValveId>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ValveLineInfo {
            subject,
            flow,
            destinations,
        })
    }
}

fn split_two<'a>(s: &'a str, dem: &str) -> Option<(&'a str, &'a str)> {
    let mut it = s.split(dem);
    let f = it.next()?;
    let s = it.next()?;
    match it.next() {
        None => Some((f, s)),
        Some(_) => None,
    }
}

#[derive(Debug)]
pub enum ParseValveLineInfoError {
    InvalidFormat,
    InvalidValveId(ParseValveIdError),
    InvalidFlow(ParseIntError),
}

impl From<ParseValveIdError> for ParseValveLineInfoError {
    fn from(err: ParseValveIdError) -> Self {
        ParseValveLineInfoError::InvalidValveId(err)
    }
}

impl From<ParseIntError> for ParseValveLineInfoError {
    fn from(err: ParseIntError) -> Self {
        ParseValveLineInfoError::InvalidFlow(err)
    }
}

#[derive(Debug)]
pub struct Valve {
    pub id: ValveId,
    pub flow: u32,
}

impl From<&ValveLineInfo> for Valve {
    fn from(line_info: &ValveLineInfo) -> Self {
        Valve {
            id: line_info.subject.clone(),
            flow: line_info.flow,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::split_two;
    use super::ParseValveIdError;
    use super::ParseValveLineInfoError;
    use super::Valve;
    use super::ValveId;
    use super::ValveLineInfo;

    #[test]
    fn test_parse_id() {
        let id = "AB".parse::<ValveId>().unwrap();
        assert_eq!(
            id,
            ValveId {
                first: 'A',
                second: 'B'
            }
        );
        let id = "CD".parse::<ValveId>().unwrap();
        assert_eq!(
            id,
            ValveId {
                first: 'C',
                second: 'D'
            }
        );
    }

    #[test]
    fn test_bad_parse_id() {
        match "".parse::<ValveId>() {
            Err(ParseValveIdError::TooShort(n)) => assert_eq!(n, 0),
            _ => panic!("Expected error not returned"),
        }
        match "A".parse::<ValveId>() {
            Err(ParseValveIdError::TooShort(n)) => assert_eq!(n, 1),
            _ => panic!("Expected error not returned"),
        }
        match "AAA".parse::<ValveId>() {
            Err(ParseValveIdError::TooLong(n)) => assert_eq!(n, 3),
            _ => panic!("Expected error not returned"),
        }
    }

    #[test]
    fn test_parse_valve_line() {
        let valve_line = "Valve II has flow rate=0; tunnels lead to valves AA, JJ"
            .parse::<ValveLineInfo>()
            .unwrap();
        assert_eq!(
            valve_line,
            ValveLineInfo {
                subject: ValveId {
                    first: 'I',
                    second: 'I'
                },
                flow: 0,
                destinations: vec![
                    ValveId {
                        first: 'A',
                        second: 'A',
                    },
                    ValveId {
                        first: 'J',
                        second: 'J',
                    }
                ]
            }
        );
        let valve_line = "Valve JJ has flow rate=21; tunnel leads to valve II"
            .parse::<ValveLineInfo>()
            .unwrap();
        assert_eq!(
            valve_line,
            ValveLineInfo {
                subject: ValveId {
                    first: 'J',
                    second: 'J'
                },
                flow: 21,
                destinations: vec![ValveId {
                    first: 'I',
                    second: 'I',
                }]
            }
        );
    }

    #[test]
    fn test_line_info_err() {
        match "II has flow rate=0; tunnels lead to valves AA, JJ".parse::<ValveLineInfo>() {
            Err(ParseValveLineInfoError::InvalidFormat) => (),
            _ => panic!("Expected error to occure"),
        }
        match "Valve II has flw rate=0; tunnels lead to valves AA, JJ".parse::<ValveLineInfo>() {
            Err(ParseValveLineInfoError::InvalidFormat) => (),
            _ => panic!("Expected error to occure"),
        }
        match "Valve II has flow rate=0 tunnels lead to valves AA, JJ".parse::<ValveLineInfo>() {
            Err(ParseValveLineInfoError::InvalidFormat) => (),
            _ => panic!("Expected error to occure"),
        }
        match "Valve II has flow rate=0; tunnels lead to vves AA, JJ".parse::<ValveLineInfo>() {
            Err(ParseValveLineInfoError::InvalidFormat) => (),
            _ => panic!("Expected error to occure"),
        }
        match "Valve I has flow rate=0; tunnels lead to valves AA, JJ".parse::<ValveLineInfo>() {
            Err(ParseValveLineInfoError::InvalidValveId(_)) => (),
            _ => panic!("Expected error to occure"),
        }
        match "Valve II has flow rate=0; tunnels lead to valves A, JJ".parse::<ValveLineInfo>() {
            Err(ParseValveLineInfoError::InvalidValveId(_)) => (),
            _ => panic!("Expected error to occure"),
        }
        match "Valve II has flow rate=3a4; tunnels lead to valves AA, JJ".parse::<ValveLineInfo>() {
            Err(ParseValveLineInfoError::InvalidFlow(_)) => (),
            _ => panic!("Expected error to occure"),
        }
    }

    #[test]
    fn test_split_two() {
        let r = split_two("A;B", ";").unwrap();
        assert_eq!(r, ("A", "B"));
        let r = split_two("A:B", ":").unwrap();
        assert_eq!(r, ("A", "B"));
        assert!(split_two("A;A;A", ";").is_none());
        assert!(split_two("A;A;A", ";").is_none());
    }

    fn test_valve_from_line() {
        let valve_id = ValveId {
            first: 'I',
            second: 'J',
        };
        let line = ValveLineInfo {
            subject: valve_id,
            flow: 34,
            destinations: vec![],
        };
        let valve = Valve::from(&line);
    }
}
