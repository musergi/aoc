pub struct Area {
    pub min: f64,
    pub max: f64,
}

impl Area {
    pub fn new(min: f64, max: f64) -> Area {
        Area { min, max }
    }
}
