use super::skeleton_3d::Direction;

mod detection;

#[derive(Clone)]
pub(crate) struct Step {
    pub id: String,
    pub name: String,
    pub variation: Option<String>,
    pub poses: Vec<usize>,
    #[allow(dead_code)]
    pub directions: Vec<Direction>,
}
