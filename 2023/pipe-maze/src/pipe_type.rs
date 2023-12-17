use crate::direction::Direction;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PipeType {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Start,
}

impl PipeType {
    pub fn connections(&self) -> Option<[Direction; 2]> {
        match self {
            PipeType::Start => None,
            PipeType::NorthSouth => Some([Direction::North, Direction::South]),
            PipeType::EastWest => Some([Direction::East, Direction::West]),
            PipeType::NorthEast => Some([Direction::North, Direction::East]),
            PipeType::NorthWest => Some([Direction::North, Direction::West]),
            PipeType::SouthWest => Some([Direction::South, Direction::West]),
            PipeType::SouthEast => Some([Direction::South, Direction::East]),
        }
    }

    pub fn connects(&self, direction: &Direction) -> bool {
        let connection = direction.opposite();
        self.connections()
            .map(|connections| connections.contains(&connection))
            .unwrap_or(true)
    }

    pub fn next(&self, entry: &Direction) -> Option<Direction> {
        let opposite = entry.opposite();
        self.connections()?
            .into_iter()
            .find(|connection| *connection != opposite)
    }
}
