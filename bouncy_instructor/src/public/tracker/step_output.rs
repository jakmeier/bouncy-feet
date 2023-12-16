use super::pose_output::PoseApproximation;
use super::Timestamp;
use wasm_bindgen::prelude::wasm_bindgen;

/// A step detected on a video feed, ready for JS code to render.
#[derive(Debug)]
#[wasm_bindgen]
pub struct DetectedStep {
    pub(crate) step_name: String,
    pub(crate) poses: Vec<PoseApproximation>,
    pub start: Timestamp,
    pub end: Timestamp,
    pub error: f32,
}

#[wasm_bindgen]
impl DetectedStep {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.step_name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn poses(&self) -> Vec<PoseApproximation> {
        self.poses.clone()
    }
}
