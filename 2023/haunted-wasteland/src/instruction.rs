pub enum Instruction {
    Left,
    Right,
}

impl TryFrom<char> for Instruction {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'R' => Ok(Instruction::Right),
            'L' => Ok(Instruction::Left),
            _ => Err("invalid instruction"),
        }
    }
}
