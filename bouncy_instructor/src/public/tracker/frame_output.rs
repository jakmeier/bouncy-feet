use super::{Keypoints, Timestamp, Tracker};
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
        config.indentor = "  ".to_string().into();
        match self
            .timestamps
            .binary_search_by(|probe| f64::total_cmp(probe, &timestamp))
        {
            Ok(i) | Err(i) => ExportedFrame {
                keypoints: ron::ser::to_string_pretty(
                    &[(self.timestamps[i], &self.keypoints[i])],
                    config.clone(),
                )
                .unwrap(),
                pose: ron::ser::to_string_pretty(
                    &crate::pose_file::Pose::from_with_db(&self.skeletons[i], &self.db),
                    config,
                )
                .unwrap(),
            },
        }
    }

    #[wasm_bindgen(js_name = exportKeypoints)]
    pub fn export_keypoints(&self) -> String {
        let mut config = ron::ser::PrettyConfig::default();
        config.indentor = "  ".to_string().into();

        let timestamp_keypoint_tuples: Vec<(Timestamp, &Keypoints)> = self
            .timestamps
            .iter()
            .copied()
            .zip(self.keypoints.iter())
            .collect();

        ron::ser::to_string_pretty(&timestamp_keypoint_tuples, config).unwrap()
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
