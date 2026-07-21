use super::pose_output::PoseApproximation;
use super::Timestamp;
use wasm_bindgen::prelude::wasm_bindgen;

/// A step detected on a video feed, ready for JS code to render.
#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct DetectedStep {
    pub(crate) step_id: String,
    pub(crate) step_name: String,
    pub(crate) poses: Vec<PoseApproximation>,
    pub start: Timestamp,
    pub end: Timestamp,
    pub error: f32,
}

#[wasm_bindgen]
impl DetectedStep {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.step_id.clone()
    }

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
        self.bpm_with_subbeats_per_move(1)
    }

    #[wasm_bindgen(js_name = bpmWithSubbeatsPerMove)]
    pub fn bpm_with_subbeats_per_move(&self, subbeats_per_move: u8) -> f32 {
        assert_ne!(subbeats_per_move, 0);
        let duration = (self.end - self.start) as f32;
        let intervals = self.poses.len() - 1;
        calculate_bpm(duration, intervals, subbeats_per_move)
    }
}

fn calculate_bpm(duration: f32, intervals: usize, subbeats_per_move: u8) -> f32 {
    30_000.0 * intervals as f32 * subbeats_per_move as f32 / duration
}

#[cfg(test)]
mod tests {
    use super::calculate_bpm;

    #[test]
    fn bpm_scales_with_subbeats_per_move() {
        assert_eq!(calculate_bpm(1_000.0, 2, 1), 60.0);
        assert_eq!(calculate_bpm(1_000.0, 2, 2), 120.0);
        assert_eq!(calculate_bpm(1_000.0, 2, 4), 240.0);
    }
}
