#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SpringCondition {
    Operational,
    Damaged,
}

impl SpringCondition {
    pub fn new(value: char) -> Result<Option<SpringCondition>, &'static str> {
        match value {
            '.' => Ok(Some(SpringCondition::Operational)),
            '#' => Ok(Some(SpringCondition::Damaged)),
            '?' => Ok(None),
            _ => Err("invalid condition char"),
        }
    }
}
