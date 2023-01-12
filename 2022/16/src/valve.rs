use std::fmt;
use std::str::FromStr;

#[derive(PartialEq, Eq)]
struct ValveId {
    first: char,
    second: char,
}

#[derive(Debug)]
enum ParseValveIdError {
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

struct ValveLineInfo {
    subject: ValveId,
    flow: u32,
    destinations: Vec<ValveId>,
}

impl FromStr for ValveLineInfo {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (sensor, list) = split_two(s, ";").unwrap();
        let (subject_s, flow_s) =
            split_two(sensor.strip_prefix("Valve ").unwrap(), " has flow rate=").unwrap();
        let subject = subject_s.parse().unwrap();
        let flow = flow_s.parse().unwrap();
        let destinations = list
            .strip_prefix(" tunnels lead to valves ")
            .or(list.strip_prefix(" tunnel leads to valve "))
            .unwrap()
            .split(", ")
            .map(|ids| ids.parse::<ValveId>())
            .collect::<Result<Vec<_>,_>>();
        Ok(ValveLineInfo {
            subject,
            flow,
            destinations: Vec::new(),
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

#[cfg(test)]
mod tests {
    use crate::valve::split_two;
    use crate::valve::ParseValveIdError;
    use crate::valve::ValveId;
    use crate::valve::ValveLineInfo;

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
            valve_line.subject,
            ValveId {
                first: 'I',
                second: 'I'
            }
        );
        assert_eq!(valve_line.flow, 0);
        let valve_line = "Valve JJ has flow rate=21; tunnel leads to valve II"
            .parse::<ValveLineInfo>()
            .unwrap();
        assert_eq!(
            valve_line.subject,
            ValveId {
                first: 'J',
                second: 'J'
            }
        );
        assert_eq!(valve_line.flow, 21);
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
}
