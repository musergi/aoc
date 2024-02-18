pub struct Part {
    pub x: u64,
    pub m: u64,
    pub a: u64,
    pub s: u64,
}

impl std::str::FromStr for Part {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ratings = s
            .strip_prefix('{')
            .ok_or("missing prefix")?
            .strip_suffix('}')
            .ok_or("missing suffix")?
            .split(',');
        let x = ratings
            .next()
            .ok_or("missing x")?
            .strip_prefix("x=")
            .ok_or("missing x prefix")?
            .parse()
            .map_err(|_| "nan")?;
        let m = ratings
            .next()
            .ok_or("missing m")?
            .strip_prefix("m=")
            .ok_or("missing m prefix")?
            .parse()
            .map_err(|_| "nan")?;
        let a = ratings
            .next()
            .ok_or("missing a")?
            .strip_prefix("a=")
            .ok_or("missing a prefix")?
            .parse()
            .map_err(|_| "nan")?;
        let s = ratings
            .next()
            .ok_or("missing s")?
            .strip_prefix("s=")
            .ok_or("missing s prefix")?
            .parse()
            .map_err(|_| "nan")?;
        Ok(Part { x, m, a, s })
    }
}
