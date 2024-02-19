use crate::module_type::ModuleType;

pub struct ModuleLine {
    pub name: String,
    pub module_type: ModuleType,
}

impl ModuleLine {
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl std::str::FromStr for ModuleLine {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, output) = s.split_once(" -> ").ok_or("missing module separator")?;
        let (name, module_type) = match name.strip_prefix('%') {
            Some(name) => (
                name.to_string(),
                ModuleType::FlipFlop(output.split(", ").map(|c| c.to_string()).collect()),
            ),
            None => match name.strip_prefix('&') {
                Some(name) => (
                    name.to_string(),
                    ModuleType::Conjunction(output.split(", ").map(|c| c.to_string()).collect()),
                ),
                None => (
                    name.to_string(),
                    ModuleType::Broadcaster(output.split(", ").map(|c| c.to_string()).collect()),
                ),
            },
        };
        Ok(ModuleLine { name, module_type })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_broadcaster_module() {
        let s = "broadcaster -> a, b, c";
        let module: ModuleLine = s.parse().unwrap();
        assert_eq!(module.name.as_str(), "broadcaster");
        match module.module_type {
            ModuleType::Broadcaster(connections) => {
                assert!(connections.iter().any(|s| s == "a"));
                assert!(connections.iter().any(|s| s == "b"));
                assert!(connections.iter().any(|s| s == "c"));
            }
            _ => panic!("invalid type"),
        }
    }

    #[test]
    fn parse_flip_flop_module() {
        let s = "%a -> b";
        let module: ModuleLine = s.parse().unwrap();
        assert_eq!(module.name.as_str(), "a");
        match module.module_type {
            ModuleType::FlipFlop(connection) => assert_eq!(connection, vec!["b"]),
            _ => panic!("invalid type"),
        }
    }

    #[test]
    fn parse_conjunction_module() {
        let s = "&inv -> a, b";
        let module: ModuleLine = s.parse().unwrap();
        assert_eq!(module.name.as_str(), "inv");
        match module.module_type {
            ModuleType::Conjunction(connection) => assert_eq!(connection, vec!["a", "b"]),
            _ => panic!("invalid type"),
        }
    }
}
