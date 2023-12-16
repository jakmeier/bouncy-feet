use super::{Timestamp, Tracker};
use crate::STATE;
use wasm_bindgen::prelude::wasm_bindgen;

/// Information of a recorded frame in RON format.
///
/// Can be useful for creating new poses, new keypoint inputs for tests, or just
/// for general debugging.
#[wasm_bindgen]
pub struct ExportedFrame {
    pub(crate) keypoints: String,
    pub(crate) pose: String,
}

#[wasm_bindgen]
impl Tracker {
    #[wasm_bindgen(js_name = exportFrame)]
    pub fn export_frame(&self, timestamp: Timestamp) -> ExportedFrame {
        let mut config = ron::ser::PrettyConfig::default();
        config.indentor = "  ".to_string();
        match self.timestamps.binary_search(&timestamp) {
            Ok(i) | Err(i) => ExportedFrame {
                keypoints: ron::ser::to_string_pretty(
                    &[(self.timestamps[i], &self.keypoints[i])],
                    config.clone(),
                )
                .unwrap(),
                pose: STATE.with_borrow(|state| {
                    ron::ser::to_string_pretty(
                        &crate::pose_file::Pose::from_with_db(&self.skeletons[i], &state.db),
                        config,
                    )
                    .unwrap()
                }),
            },
        }
    }
}

#[wasm_bindgen]
impl ExportedFrame {
    #[wasm_bindgen(getter)]
    pub fn pose(&self) -> String {
        self.pose.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn keypoints(&self) -> String {
        self.keypoints.clone()
    }
}
