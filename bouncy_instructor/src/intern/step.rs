use super::skeleton_3d::Direction;

mod detection;

pub(crate) struct Step {
    pub name: String,
    pub poses: Vec<usize>,
    #[allow(dead_code)]
    pub directions: Vec<Direction>,
}
