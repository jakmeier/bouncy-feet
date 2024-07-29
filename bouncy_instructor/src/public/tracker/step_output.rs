use super::pose_output::PoseApproximation;
use super::Timestamp;
use wasm_bindgen::prelude::wasm_bindgen;

/// A step detected on a video feed, ready for JS code to render.
#[derive(Debug, Clone)]
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

    #[wasm_bindgen(getter)]
    pub fn bpm(&self) -> f32 {
        let duration = (self.end - self.start) as f32;
        let intervals = self.poses.len() - 1;
        //   1 s / beatDuration
        // = 60'000 ms  / (duration / intervals / 2)
        // = 60'000 ms * 2 * intervals / duration
        30_000.0 * intervals as f32 / duration
    }
}
