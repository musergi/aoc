use crate::hash::hash;

pub struct HashMap {
    boxes: Vec<Box>,
}

impl Default for HashMap {
    fn default() -> Self {
        let boxes = (0..256).into_iter().map(|_| Box::default()).collect();
        Self { boxes }
    }
}

impl HashMap {
    pub fn insert(&mut self, label: &str, focal_length: u8) {
        let box_index: usize = hash(label).into();
        self.boxes[box_index].insert(label, focal_length)
    }

    pub fn remove(&mut self, label: &str) {
        let box_index: usize = hash(label).into();
        self.boxes[box_index].remove(label);
    }

    pub fn focusing_power(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .map(|(idx, b)| (idx + 1) * b.power())
            .sum()
    }
}

#[derive(Default)]
struct Box {
    lenses: Vec<Lens>,
}

impl Box {
    fn insert(&mut self, label: &str, focal_length: u8) {
        if let Some(existing) = self.lenses.iter_mut().find(|lens| lens.name == label) {
            existing.focal_length = focal_length;
        } else {
            self.lenses.push(Lens::new(label, focal_length));
        }
    }

    fn remove(&mut self, label: &str) {
        if let Some(position) = self.lenses.iter().position(|lens| lens.name == label) {
            self.lenses.remove(position);
        }
    }

    fn power(&self) -> usize {
        self.lenses
            .iter()
            .enumerate()
            .map(|(idx, lens)| (idx + 1) * lens.focal_length as usize)
            .sum()
    }
}

struct Lens {
    name: String,
    focal_length: u8,
}

impl Lens {
    fn new(name: impl ToString, focal_length: u8) -> Self {
        let name = name.to_string();
        Self { name, focal_length }
    }
}

#[cfg(test)]
mod tests {
    use super::HashMap;

    #[test]
    fn test_additino() {
        let mut map = HashMap::default();
        map.insert("rn", 1);
        assert_eq!(map.boxes[0].lenses[0].name, "rn");
        assert_eq!(map.boxes[0].lenses[0].focal_length, 1);
    }

    #[test]
    fn test_non_removal() {
        let mut map = HashMap::default();
        map.insert("rn", 1);
        map.remove("cm");
        assert_eq!(map.boxes[0].lenses[0].name, "rn");
        assert_eq!(map.boxes[0].lenses[0].focal_length, 1);
    }

    #[test]
    fn test_second_addition() {
        let mut map = HashMap::default();
        map.insert("rn", 1);
        map.remove("cm");
        map.insert("qp", 3);
        assert_eq!(map.boxes[0].lenses[0].name, "rn");
        assert_eq!(map.boxes[0].lenses[0].focal_length, 1);
        assert_eq!(map.boxes[1].lenses[0].name, "qp");
        assert_eq!(map.boxes[1].lenses[0].focal_length, 3);
    }

    #[test]
    fn test_same_box_addition() {
        let mut map = HashMap::default();
        map.insert("rn", 1);
        map.remove("cm");
        map.insert("qp", 3);
        map.insert("cm", 2);
        assert_eq!(map.boxes[0].lenses[0].name, "rn");
        assert_eq!(map.boxes[0].lenses[0].focal_length, 1);
        assert_eq!(map.boxes[0].lenses[1].name, "cm");
        assert_eq!(map.boxes[0].lenses[1].focal_length, 2);
        assert_eq!(map.boxes[1].lenses[0].name, "qp");
        assert_eq!(map.boxes[1].lenses[0].focal_length, 3);
    }

    #[test]
    fn test_removal() {
        let mut map = HashMap::default();
        map.insert("rn", 1);
        map.remove("cm");
        map.insert("qp", 3);
        map.insert("cm", 2);
        map.remove("qp");
        assert_eq!(map.boxes[0].lenses[0].name, "rn");
        assert_eq!(map.boxes[0].lenses[0].focal_length, 1);
        assert_eq!(map.boxes[0].lenses[1].name, "cm");
        assert_eq!(map.boxes[0].lenses[1].focal_length, 2);
        assert!(map.boxes[1].lenses.is_empty());
    }

    #[test]
    fn test_replacement() {
        let mut map = HashMap::default();
        map.insert("rn", 3);
        map.insert("rn", 2);
        assert_eq!(map.boxes[0].lenses[0].name, "rn");
        assert_eq!(map.boxes[0].lenses[0].focal_length, 2);
    }
}
