use super::skeleton_3d::Direction;

mod detection;

pub(crate) struct Step {
    pub name: String,
    pub poses: Vec<usize>,
    pub directions: Vec<Direction>,
}
