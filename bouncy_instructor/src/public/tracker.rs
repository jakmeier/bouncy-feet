use crate::intern::pose_score::{best_fit_pose, ErrorDetails};
use crate::intern::skeleton_3d::Skeleton3d;
use crate::keypoints::Keypoints;
use crate::skeleton::Skeleton;
use crate::STATE;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Tracker {
    /// invariant: ordered by timestamp
    timestamps: Vec<u32>,
    /// full keypoints as originally recorded
    keypoints: Vec<Keypoints>,
    /// tracked limbs
    skeletons: Vec<Skeleton3d>,
}

/// The result of fitting keypoints to poses.
#[wasm_bindgen]
pub struct PoseApproximation {
    /// ID defined in pose file
    name: String,
    /// Total error between 0.0 and 1.0.
    pub error: f32,
    /// Timestamp for which Keypoints were added
    pub timestamp: u32,
    error_details: ErrorDetails,
}

/// Self-describing error score for a specific limbb
#[wasm_bindgen]
pub struct LimbError {
    name: String,
    pub error: f32,
    pub weight: f32,
}

/// Information of a recorded frame in RON format.
///
/// Can be useful for creating new poses, new keypoint inputs for tests, or just
/// for general debugging.
#[wasm_bindgen]
pub struct ExportedFrame {
    keypoints: String,
    pose: String,
}

#[wasm_bindgen]
pub struct Skeletons {
    pub front: Skeleton,
    pub side: Skeleton,
}

#[wasm_bindgen]
impl Tracker {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Tracker {
            // order by timestamp satisfied for empty list
            timestamps: vec![],
            keypoints: vec![],
            skeletons: vec![],
        }
    }

    #[wasm_bindgen(js_name = addKeypoints)]
    pub fn add_keypoints(&mut self, keypoints: Keypoints, timestamp: u32) -> Skeletons {
        if let Some(last) = self.timestamps.last() {
            if *last >= timestamp {
                panic!("inserted data not strictly monotonically increasing");
            }
        }

        // modification preserves timestamp order if it was true before
        self.timestamps.push(timestamp);
        self.keypoints.push(keypoints);
        let skeleton_info = Skeleton3d::from_keypoints(&keypoints);
        let front = skeleton_info.to_skeleton(0.0, false);
        let side = skeleton_info.to_skeleton(90.0, true);
        self.skeletons.push(skeleton_info);
        Skeletons { front, side }
    }

    /// Fit frames in a time interval against all poses and return the best fit.
    #[wasm_bindgen(js_name = bestFitPose)]
    pub fn best_fit_pose(&self, start: u32, end: u32) -> Option<PoseApproximation> {
        let first = self.timestamps.partition_point(|t| *t < start);
        let last = self.timestamps.partition_point(|t| *t <= end);
        if first == last {
            return None;
        }
        let result = crate::STATE.with_borrow(|state| {
            if state.db.is_empty() {
                return None;
            }
            let mut error = f32::INFINITY;
            let mut error_details = ErrorDetails::default();
            let mut pose_index = 0;
            let mut history_index = 0;

            for i in first..last {
                let (err, details, pose) = best_fit_pose(&self.skeletons[i], state.db.poses());
                if err < error {
                    error = err;
                    error_details = details;
                    pose_index = pose;
                    history_index = i;
                }
            }
            Some(PoseApproximation {
                name: state.db.pose_name(pose_index).to_owned(),
                error,
                timestamp: self.timestamps[history_index],
                error_details,
            })
        })?;
        Some(result)
    }

    /// Fit a single frame against all poses and return all errors
    #[wasm_bindgen(js_name = allPoseErrors)]
    pub fn all_pose_errors(&self, timestamp: u32) -> Vec<PoseApproximation> {
        if self.skeletons.is_empty() {
            return vec![];
        }

        let skeleton = match self.timestamps.binary_search(&timestamp) {
            Ok(i) | Err(i) => &self.skeletons[i],
        };

        // for debugging, quite useful for now
        for (angle, name) in skeleton
            .angles()
            .iter()
            .zip(crate::intern::pose::Limb::base_limb_names())
        {
            crate::println!("{name}: {angle:?}");
        }

        crate::STATE.with_borrow(|state| {
            let angles = skeleton.angles();
            state
                .db
                .poses()
                .iter()
                .enumerate()
                .map(|(pose_index, pose)| {
                    let details = pose.error(&angles);
                    PoseApproximation {
                        name: state.db.pose_name(pose_index).to_owned(),
                        error: details.error_score(),
                        timestamp,
                        error_details: details,
                    }
                })
                .collect()
        })
    }

    #[wasm_bindgen(js_name = exportFrame)]
    pub fn export_frame(&self, timestamp: u32) -> ExportedFrame {
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
impl PoseApproximation {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// List all limbs, order by how well they fit, best fit first.
    #[wasm_bindgen(js_name = limbErrors)]
    pub fn limb_errors(&self) -> Vec<LimbError> {
        let increasing = true;
        let weighted = false;
        self.sorted_limb_errors_impl(increasing, weighted).collect()
    }

    /// List the `n` limbs with the highest error contribution to the pose error.
    #[wasm_bindgen(js_name = worstLimbs)]
    pub fn worst_limbs(&self, n: usize) -> Vec<LimbError> {
        let increasing = false;
        let weighted = true;
        self.sorted_limb_errors_impl(increasing, weighted)
            .take(n)
            .collect()
    }

    fn sorted_limb_errors_impl(
        &self,
        increasing: bool,
        weighted: bool,
    ) -> impl Iterator<Item = LimbError> + '_ {
        self.error_details
            .sorted_by_error(increasing, weighted)
            .into_iter()
            .map(|i| LimbError {
                name: STATE.with_borrow(|state| {
                    state.db.limb_name(self.error_details.limbs[i]).to_owned()
                }),
                error: self.error_details.errors[i],
                weight: self.error_details.weights[i],
            })
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

#[wasm_bindgen]
impl LimbError {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }
}
