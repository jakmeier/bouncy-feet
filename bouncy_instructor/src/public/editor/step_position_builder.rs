use wasm_bindgen::prelude::wasm_bindgen;

use crate::intern::content_collection::ContentCollection;
use crate::pose_file::BodyPoint;
use crate::step_file::{Orientation, StepPosition};
use crate::wrapper::pose_wrapper::PoseWrapper;

/// Used in the editor to add and edit poses of a step.
#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct StepPositionBuilder {
    pub(crate) pose: PoseWrapper,
    /// specify how the pose should be oriented
    pub(crate) orientation: Orientation,
    /// Which body part shall remain fixed, while the rest of the body moves.
    ///
    /// The pivot defines defines how to transition into this pose. On the first
    /// pose of a step, this is relevant for looping.
    pub(crate) pivot: BodyPoint,
    /// How high to jump when transitioning into this step, relative to the full
    /// body size.
    pub(crate) jump_height: Option<f32>,
}

#[wasm_bindgen]

impl StepPositionBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new(pose: &PoseWrapper) -> Self {
        Self {
            pose: pose.clone(),
            orientation: Orientation::Right,
            pivot: BodyPoint::default_pivot(),
            jump_height: None,
        }
    }

    pub fn pose(&self) -> PoseWrapper {
        self.pose.clone()
    }

    // TODO
    // #[wasm_bindgen(js_name="setPivot")]
    // pub fn set_pivot(&mut self, ){}

    #[wasm_bindgen(getter, js_name = "jumpHeight")]
    pub fn jump_height(&self) -> Option<f32> {
        self.jump_height
    }

    #[wasm_bindgen(js_name = "setJumpHeight")]
    pub fn set_jump_height(&mut self, height: f32) {
        self.jump_height = Some(height);
    }

    #[wasm_bindgen(getter)]
    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    #[wasm_bindgen(js_name = "setOrientation")]
    pub fn set_orientation(&mut self, orientation: Orientation) {
        self.orientation = orientation
    }
}

impl From<&StepPositionBuilder> for StepPosition {
    fn from(position: &StepPositionBuilder) -> Self {
        StepPosition {
            pose: position.pose.id(),
            orientation: position.orientation,
            pivot: position.pivot.clone(),
            jump_height: position.jump_height,
        }
    }
}

impl StepPositionBuilder {
    pub(crate) fn try_from_step_position(
        pose: &StepPosition,
        db: &ContentCollection,
    ) -> Option<Self> {
        let pose_wrapper = db.pose_by_id(&pose.pose).cloned()?;
        Some(StepPositionBuilder {
            pose: pose_wrapper,
            orientation: pose.orientation,
            pivot: pose.pivot.clone(),
            jump_height: pose.jump_height,
        })
    }
}
