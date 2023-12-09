use std::str::FromStr;

pub struct Line {
    pub source: String,
    pub left: String,
    pub right: String,
}

impl FromStr for Line {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (source, expr) = s.split_once(" = ").ok_or("no expression split")?;
        let (left, right) = expr
            .strip_prefix("(")
            .ok_or("missing open parenthesis")?
            .strip_suffix(")")
            .ok_or("missing close parenthesis")?
            .split_once(", ")
            .ok_or("missing left and right delimiter")?;
        let source = source.to_string();
        let left = left.to_string();
        let right = right.to_string();
        Ok(Line {
            source,
            left,
            right,
        })
    }
}
