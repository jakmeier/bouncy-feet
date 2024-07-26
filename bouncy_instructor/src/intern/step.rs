use std::ops::Deref;
use std::rc::Rc;

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
    pub source: StepSource,
}

/// Type-safe string to describe a step source.
///
/// Also includes performance improvements to avoid WASM internal clones and
/// WASM-to-JS expensive string conversions.
///
/// Derefs into a &str.
#[derive(Clone, Debug)]
pub(crate) struct StepSource {
    name: Rc<String>,
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
            source: self.source,
        }
    }
}

impl StepSource {
    pub fn new(name: String) -> Self {
        Self {
            name: Rc::new(name),
        }
    }
}

impl Deref for StepSource {
    type Target = str;

    fn deref(&self) -> &str {
        wasm_bindgen::intern(self.name.as_ref())
    }
}
