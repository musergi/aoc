#[derive(Debug, PartialEq, Eq)]
pub enum Instruction<'a> {
    Insert { label: &'a str, focal_length: u8 },
    Remove { label: &'a str },
}

impl<'a> Instruction<'a> {
    pub fn parse(s: &'a str) -> Result<Self, &'static str> {
        s.split_once("=")
            .and_then(|(label, num)| {
                num.parse::<u8>()
                    .ok()
                    .map(|focal_length| Instruction::Insert {
                        label,
                        focal_length,
                    })
            })
            .or_else(|| {
                s.strip_suffix("-")
                    .map(|label| Instruction::Remove { label })
            })
            .ok_or_else(|| "invalid instruction")
    }
}

#[cfg(test)]
mod tests {
    use crate::instruction::Instruction;

    #[test]
    fn parse_insert() {
        let parsed = Instruction::parse("rn=1").unwrap();
        assert_eq!(
            parsed,
            Instruction::Insert {
                label: "rn",
                focal_length: 1
            }
        )
    }

    #[test]
    fn parse_remove() {
        let parsed = Instruction::parse("pm-").unwrap();
        assert_eq!(parsed, Instruction::Remove { label: "pm" })
    }
}
