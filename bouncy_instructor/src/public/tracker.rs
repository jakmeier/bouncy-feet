use crate::intern::pose_score::{best_fit_pose, ErrorDetails};
use crate::intern::skeleton_3d::Skeleton3d;
use crate::keypoints::Keypoints;
use crate::skeleton::Skeleton;
use crate::STATE;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Tracker {
    /// full keypoints as originally recorded
    /// invariant: ordered by timestamp
    history: Vec<(u32, Keypoints)>,
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

#[wasm_bindgen]
impl Tracker {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Tracker {
            // order by timestamp satisfied for empty list
            history: vec![],
            skeletons: vec![],
        }
    }

    #[wasm_bindgen(js_name = addKeypoints)]
    pub fn add_keypoints(&mut self, mut keypoints: Keypoints, timestamp: u32) -> Skeleton {
        if let Some(last) = self.history.last() {
            if last.0 >= timestamp {
                panic!("inserted data not strictly monotonically increasing");
            }
        }

        // It seems that the z coordinate is just not to scale with x and y in
        // the output of mediapipe. It should be adjusted when they are
        // converted to keypoints, i.e. BEFORE we see it in WASM. But it's
        // easier to fine tune the divisor inside Rust for now.
        keypoints
            .iter_mut()
            .for_each(|coordiante| coordiante.z /= 2.0);

        // modification preserves timestamp order if it was true before
        self.history.push((timestamp, keypoints));
        let skeleton_info = Skeleton3d::from_keypoints(&keypoints);
        let skeleton = skeleton_info.to_skeleton();
        self.skeletons.push(skeleton_info);
        skeleton
    }

    /// Fit frames in a time interval against all poses and return the best fit.
    #[wasm_bindgen(js_name = bestFitPose)]
    pub fn best_fit_pose(&self, start: u32, end: u32) -> Option<PoseApproximation> {
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
                timestamp: self.history[history_index].0,
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

        let skeleton = match self.history.binary_search_by(|el| el.0.cmp(&timestamp)) {
            Ok(i) | Err(i) => &self.skeletons[i],
        };
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
    pub fn export_frame(&self, timestamp: u32) -> String {
        let mut config = ron::ser::PrettyConfig::default();
        config.indentor = "  ".to_string();
        match self.history.binary_search_by(|f| f.0.cmp(&timestamp)) {
            Ok(i) | Err(i) => ron::ser::to_string_pretty(&self.history[i..i + 1], config).unwrap(),
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
impl LimbError {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }
}
