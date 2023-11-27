use crate::keypoints::Keypoints;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Tracker {
    /// full keypoints as originally recorded
    /// invariant: ordered by timestamp
    history: Vec<(u32, Keypoints)>,
    /// tracked limbs
    limb_angles: Vec<Vec<f32>>,
}

/// The result of fitting keypoints to poses.
#[wasm_bindgen]
pub struct PoseApproximation {
    name: String,
    pub error: f32,
    pub timestamp: u32,
}

#[wasm_bindgen]
impl Tracker {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Tracker {
            // order by timestamp satisfied for empty list
            history: vec![],
            limb_angles: vec![],
        }
    }

    #[wasm_bindgen(js_name = addKeypoints)]
    pub fn add_keypoints(&mut self, keypoints: Keypoints, timestamp: u32) {
        if let Some(last) = self.history.last() {
            if last.0 >= timestamp {
                panic!("inserted data not strictly monotonically increasing");
            }
        }
        // modification preserves timestamp order if it was true before
        self.history.push((timestamp, keypoints));
        let limbs = super::STATE.with(|state| state.borrow().db.angles_from_keypoints(&keypoints));
        self.limb_angles.push(limbs);
    }

    #[wasm_bindgen(js_name = bestFitPosition)]
    pub fn best_fit_position(&self, start: u32, end: u32) -> Option<PoseApproximation> {
        let first = self.history.partition_point(|(t, _kp)| *t < start);
        let last = self.history.partition_point(|(t, _kp)| *t <= end);
        if first == last {
            return None;
        }
        let result = crate::STATE.with_borrow(|state| {
            if state.db.is_empty() {
                return None;
            }
            let mut error = f32::INFINITY;
            let mut pose_index = 0;
            let mut history_index = 0;

            for i in first..last {
                let (err, pose) = state.db.fit(&self.limb_angles[i]);
                if err < error {
                    error = err;
                    pose_index = pose;
                    history_index = i;
                }
            }
            Some(PoseApproximation {
                name: state.db.pose_name(pose_index).to_owned(),
                error,
                timestamp: self.history[history_index].0,
            })
        })?;
        Some(result)
    }
}
#[wasm_bindgen]
impl PoseApproximation {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }
}
