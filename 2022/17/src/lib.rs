enum Shift {
    Left,
    Right,
}

impl Shift {
    fn shift(&self, layer: u8) -> u8 {
        match self {
            Shift::Left => layer << 1,
            Shift::Right => layer >> 1,
        }
    }
}

pub struct Board {
    width: u8,
    spawn_offset: usize,
    data: Vec<u8>,
}

pub struct AddedShape {
    index: usize,
    shape: [u8; 4],
}

impl Board {
    fn new(width: u8, spawn_offset: usize) -> Board {
        Board {
            width,
            spawn_offset,
            data: Vec::new(),
        }
    }

    fn valid(&self, value: &u8) -> bool {
        let mask = (1u8 << self.width) - 1;
        *value & mask == *value
    }

    fn can_shift_right(&self, value: &u8) -> bool {
        value & 0b1 == 0
    }

    fn can_shift_left(&self, value: &u8) -> bool {
        let mask = (1u8 << self.width - 1) - 1;
        *value & mask == *value
    }

    fn can_shift(&self, value: &u8, shift: &Shift) -> bool {
        match shift {
            Shift::Left => self.can_shift_left(value),
            Shift::Right => self.can_shift_right(value),
        }
    }

    fn shift(&self, row: usize, layer: &u8, shift: &Shift) -> Option<u8> {
        if self.can_shift(layer, shift) {
            let shifted = shift.shift(*layer);
            if self
                .data
                .get(row)
                .map(|row| row & shifted == 0)
                .unwrap_or(true)
            {
                Some(shifted)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn add_shape(self, shape: [u8; 4]) -> AddedShape {
        let index = self.data.len() + self.spawn_offset;
        AddedShape { index, shape }
    }

    fn lower(&self, added_shape: &mut AddedShape) -> bool {
        let can_lower = added_shape.shape.iter().enumerate().all(|(offset, layer)| {
            self.data
                .get(added_shape.index + offset)
                .map(|row| row & layer == 0)
                .unwrap_or(true)
        });
        if can_lower {
            added_shape.index -= 1;
        }
        can_lower
    }

    fn attempt_shift(&self, added_shape: &mut AddedShape, shift: &Shift) {
        let can_shift = added_shape.shape.iter().enumerate().all(|(index, layer)| {
            let layer_row = added_shape.index + index;
            self.shift(layer_row, layer, shift).is_some()
        });
        if can_shift {
            for (index, layer) in added_shape.shape.iter_mut().enumerate() {
                let layer_row = added_shape.index + index;
                *layer = self
                    .shift(layer_row, layer, shift)
                    .expect("Already checked");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Board;

    #[test]
    fn test_valid() {
        let b = Board::new(3, 1);
        assert!(b.valid(0b00000111));
        assert!(b.valid(0b00000001));
        assert!(!b.valid(0b00001000));
        assert!(!b.valid(0b00010000));
    }

    #[test]
    fn test_can_shift_right() {
        let b = Board::new(3, 1);
        assert!(!b.can_shift_right(0b00000111));
        assert!(!b.can_shift_right(0b00000001));
        assert!(b.can_shift_right(0b0000010));
        assert!(b.can_shift_right(0b0000100));
    }

    #[test]
    fn test_can_shift_left() {
        let b = Board::new(3, 1);
        assert!(b.can_shift_left(0b00000011));
        assert!(b.can_shift_left(0b00000001));
        assert!(b.can_shift_left(0b00000010));
        assert!(!b.can_shift_left(0b00000110));
        assert!(!b.can_shift_left(0b00000100));
        assert!(!b.can_shift_left(0b00000111));
    }
}
