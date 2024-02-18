#[derive(Clone)]
pub enum Value {
    Accept,
    Reject,
    Goto(String),
}

impl std::str::FromStr for Value {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Value::Accept,
            "R" => Value::Reject,
            other => Value::Goto(other.to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_accept() {
        let s = "A";
        let value: Value = s.parse().unwrap();
        assert!(matches!(value, Value::Accept));
    }

    #[test]
    fn parse_reject() {
        let s = "R";
        let value: Value = s.parse().unwrap();
        assert!(matches!(value, Value::Reject));
    }

    #[test]
    fn parse_goto1() {
        let s = "aaa";
        let value: Value = s.parse().unwrap();
        match value {
            Value::Goto(v) => assert_eq!(v, "aaa"),
            _ => panic!("invalid variant"),
        }
    }

    #[test]
    fn parse_goto2() {
        let s = "baa";
        let value: Value = s.parse().unwrap();
        match value {
            Value::Goto(v) => assert_eq!(v, "baa"),
            _ => panic!("invalid variant"),
        }
    }
}
