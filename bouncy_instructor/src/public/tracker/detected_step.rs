use super::{PoseApproximation, Timestamp};
use wasm_bindgen::prelude::wasm_bindgen;

/// A step detected on a video feed, ready for JS code to render.
#[wasm_bindgen]
pub struct DetectedStep {
    step_name: String,
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

impl DetectedStep {
    pub(crate) fn new(step_name: String, poses: Vec<PoseApproximation>) -> Self {
        Self {
            step_name,
            start: poses.first().map(|p| p.timestamp).unwrap_or(0),
            end: poses.last().map(|p| p.timestamp).unwrap_or(0),
            error: poses.iter().map(|p| p.error).sum::<f32>() / poses.len() as f32,
            poses,
        }
    }
}
