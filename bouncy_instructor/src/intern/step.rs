use super::pose::BodyPoint;
use super::skeleton_3d::Direction;

mod detection;

#[derive(Clone, Debug)]
pub(crate) struct Step {
    pub id: String,
    pub name: String,
    pub variation: Option<String>,
    pub poses: Vec<usize>,
    pub directions: Vec<Direction>,
    pub pivots: Vec<BodyPoint>,
}

impl Step {
    pub(crate) fn flipped(self) -> Self {
        Step {
            id: self.id,
            name: self.name,
            variation: self.variation,
            poses: self.poses,
            directions: self
                .directions
                .into_iter()
                .map(|dir| match dir {
                    Direction::North => Direction::South,
                    Direction::East => Direction::West,
                    Direction::South => Direction::North,
                    Direction::West => Direction::East,
                    Direction::Unknown => Direction::Unknown,
                })
                .collect(),
            pivots: self.pivots,
        }
    }
}
