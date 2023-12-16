use crate::intern::pose_score::{best_fit_pose, ErrorDetails};
use crate::intern::skeleton_3d::Skeleton3d;
use crate::keypoints::Keypoints;
use crate::skeleton::Skeleton;
use crate::tracker::detected_step::DetectedStep;
use crate::STATE;
use wasm_bindgen::prelude::wasm_bindgen;

mod detected_step;

type Timestamp = u32;

#[wasm_bindgen]
pub struct Tracker {
    /// invariant: ordered by timestamp
    timestamps: Vec<Timestamp>,
    /// full keypoints as originally recorded
    keypoints: Vec<Keypoints>,
    /// tracked limbs
    skeletons: Vec<Skeleton3d>,

    // below are "Dance Detector" fields, maybe it should be its own struct?
    bpm: f32,
    // todo: head and tail for what was already detected, to not re-compute all every time
    // todo: active steps filter instead of global steps
}

/// The result of fitting keypoints to poses.
#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct PoseApproximation {
    /// ID defined in pose file
    name: String,
    /// Total error between 0.0 and 1.0.
    pub error: f32,
    /// Timestamp for which Keypoints were added
    pub timestamp: Timestamp,
    error_details: ErrorDetails,
}

/// Self-describing error score for a specific limbb
#[wasm_bindgen]
pub struct LimbError {
    name: String,
    pub error: f32,
    pub weight: f32,
}

// todo: maybe move to separate module
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
            bpm: 120.0,
        }
    }

    /// Insert keypoints of a new frame for tracking.
    ///
    /// This is the main method to insert data into the tracker.
    #[wasm_bindgen(js_name = addKeypoints)]
    pub fn add_keypoints(&mut self, keypoints: Keypoints, timestamp: Timestamp) -> Skeletons {
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

    #[wasm_bindgen(js_name = detectedDance)]
    pub fn detected_dance(&self) -> Vec<DetectedStep> {
        let mut start = 0;
        let end = self.timestamps.len();

        let mut out = vec![];
        while let Some(step) = self.find_first_step(start, end) {
            let end_t = step.poses.last().unwrap().timestamp;
            start = self.timestamps[start..end].partition_point(|t| *t <= end_t);
            out.push(step);
        }
        out
    }

    // todo: probably this function should be somewhere in ::intern::*?
    /// find the first step that can be matched in the given range
    fn find_first_step(&self, mut start: usize, end: usize) -> Option<DetectedStep> {
        if start + 1 > self.timestamps.len() {
            return None;
        }
        let end = end.min(self.timestamps.len());
        let dt = 60_000.0 / self.bpm;
        let min_dt = (dt * 0.5).round() as u32;
        let max_dt = (dt * 1.5).round() as u32;

        let threshold = 0.3;
        let pose_window_ms = max_dt;
        // let step_window_ms = dt;

        // pre-conditions:
        //   timestamps are sorted in increasing order
        // loop invariants:
        //   i0 start_t >= timestamps[start]
        //   i1 start_t <= timestamps[start+1]
        //   (assume `timestamps[n] = INFINITY` if n >= timestamps.len())
        // loop variant:
        //   v0 start increases every iteration
        let mut start_t = self.timestamps[start];
        // i0 holds because of above assignment, i1 because of the pre-condition
        while start < self.timestamps.len() {
            let end_t = (start_t + pose_window_ms).min(self.timestamps[end - 1]);
            let first_pose_candidate = self.best_fit_pose(start_t, end_t);
            if first_pose_candidate
                .as_ref()
                .is_some_and(|p| p.error <= threshold)
            {
                // a pose was found, let's try to find a step starting around that time frame
                let step_start_t = first_pose_candidate.unwrap().timestamp;
                let step_start_index =
                    start + self.timestamps[start..end].partition_point(|t| *t < step_start_t);
                if let Some(step) = self.detect_step(step_start_index, min_dt, max_dt) {
                    return Some(step);
                }
                // no step follows that pose, move search window to be after the pose just checked
                // TODO: should the pose be recorded and the search window moved by min_dt?
                // satisfies v0
                start = step_start_index + 1;
                // recover i0 and i1
                start_t = *self.timestamps.get(start).unwrap_or(&u32::MAX);
            } else {
                // couldn't even match a pose, shift search window by one window length
                start_t = start_t + pose_window_ms;
                // recover i0 and i1, plus:
                // start_t just increased
                // => timestamps[start] < start_t
                // => the partition point can't be 0
                // => satisfies v0
                start = start + self.timestamps[start..end].partition_point(|t| *t < start_t);
            }
        }

        None
    }

    // todo: probably this function should be somewhere in ::intern::*?
    /// Try to find a step after `start` with the given minimum and maximum beat
    /// durations.
    ///
    /// This could be done in two different ways. Either searching best fit
    /// poses first and checking if any step matches those exact poses. Or
    /// searching for each step if the respective poses could be fitted.
    /// The first way optimizes for minimal pose error, the second way optimizes
    /// the step error but requires more computation over all.
    ///
    /// The second way is substantially better because it makes it more likely
    /// to find a step. Otherwise, perhaps there is a slightly better match for
    /// a pose with a flat foot, while the intended step needed the pose on heels.
    fn detect_step(&self, start: usize, min_dt: u32, max_dt: u32) -> Option<DetectedStep> {
        let mut best_error = f32::INFINITY;
        let mut result = None;

        STATE.with_borrow(|state| {
            for step in &state.steps {
                let mut pose_matches = vec![];
                let mut start_t = self.timestamps[start];
                let mut end_t = start_t + max_dt;
                for pose in &step.poses {
                    // TODO: check for `pose` and check for threshold
                    if let Some(pose_match) = self.find_pose(*pose, start_t, end_t) {
                        start_t = pose_match.timestamp + min_dt;
                        end_t = pose_match.timestamp + max_dt;
                        pose_matches.push(pose_match);
                    } else {
                        pose_matches.clear();
                        break;
                    }
                }
                if !pose_matches.is_empty() {
                    let detection = DetectedStep::new(step.name.clone(), pose_matches);
                    if detection.error < best_error {
                        best_error = detection.error;
                        result = Some(detection);
                    }
                }
            }
        });
        result
    }

    /// Fit frames in a time interval against all poses and return the best fit.
    ///
    /// This API is exported mostly for debugging. To extract fitted dances, use
    /// `detected_dance` instead.
    #[wasm_bindgen(js_name = bestFitPose)]
    pub fn best_fit_pose(&self, start: Timestamp, end: Timestamp) -> Option<PoseApproximation> {
        let first = self.timestamps.partition_point(|t| *t < start);
        let last = self.timestamps.partition_point(|t| *t <= end);
        if first == last {
            return None;
        }
        self.best_fit_pose_impl(first, last)
    }

    fn best_fit_pose_impl(&self, first: usize, last: usize) -> Option<PoseApproximation> {
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

    /// Find the best matching approximation of the given pose in the given range.
    fn find_pose(
        &self,
        pose_index: usize,
        start: Timestamp,
        end: Timestamp,
    ) -> Option<PoseApproximation> {
        let first = self.timestamps.partition_point(|t| *t < start);
        let last = self.timestamps.partition_point(|t| *t <= end);
        if first == last {
            return None;
        }

        let result = crate::STATE.with_borrow(|state| {
            if state.db.is_empty() {
                return None;
            }
            let mut best_error = f32::INFINITY;
            let mut best_details = ErrorDetails::default();
            let mut history_index = 0;

            for i in first..last {
                let details = state.db.poses()[pose_index].error(&self.skeletons[i].angles());
                let error = details.error_score();
                if error < best_error {
                    best_error = error;
                    best_details = details;
                    history_index = i;
                }
            }
            Some(PoseApproximation {
                name: state.db.pose_name(pose_index).to_owned(),
                error: best_error,
                timestamp: self.timestamps[history_index],
                error_details: best_details,
            })
        })?;
        Some(result)
    }

    /// Fit a single frame against all poses and return all errors
    #[wasm_bindgen(js_name = allPoseErrors)]
    pub fn all_pose_errors(&self, timestamp: Timestamp) -> Vec<PoseApproximation> {
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

#[cfg(test)]
mod tests {
    use crate::keypoints::Cartesian3d;
    use crate::{load_pose_str, load_step_str};

    use super::*;

    #[test]
    fn test_detect_dance() {
        load_pose_str(
            r#"
        #![enable(implicit_some)]
        (
          version: 0,
          poses: [
            (
              name: "test-pose-0",
              direction: Front,
              limbs: [
                (limb: LeftShin, angle: 0, tolerance: 5, weight: 1.0),
              ]
            ),
            (
              name: "test-pose-1",
              direction: Front,
              limbs: [
                (limb: LeftShin, angle: 90, tolerance: 5, weight: 1.0),
              ]
            ),
            (
              name: "test-pose-2",
              direction: Front,
              limbs: [
                (limb: LeftShin, angle: -90, tolerance: 5, weight: 1.0),
              ]
            ),
            (
              name: "test-pose-3",
              direction: Right,
              limbs: [
                (limb: LeftShin, angle: 90, tolerance: 5, weight: 1.0),
              ]
            ),
          ]
        )"#,
        )
        .unwrap();

        load_step_str(
            r#"
        (
          version: 0,
          steps: [
            (
                name: "Test-Step-1",
                keyframes: [
                (pose: "test-pose-0", orientation: ToCamera),
                (pose: "test-pose-2", orientation: ToCamera),
                ]
            ),
            (
              name: "Test-Step-2",
              keyframes: [
                (pose: "test-pose-0", orientation: ToCamera),
                (pose: "test-pose-1", orientation: ToCamera),
              ]
            ),
            (
              name: "Test-Step-3",
              keyframes: [
                (pose: "test-pose-2", orientation: ToCamera),
                (pose: "test-pose-1", orientation: ToCamera),
              ]
            ),
          ]
        )"#,
        )
        .unwrap();

        let mut tracker = Tracker::new();

        let mut kp0 = Keypoints::default();

        // make skeleton face the camera
        kp0.left.shoulder = Cartesian3d::new(1.0, -2.0, 0.0);
        kp0.right.shoulder = Cartesian3d::new(-1.0, -2.0, 0.0);
        kp0.left.hip = Cartesian3d::new(1.0, -1.0, 0.0);
        kp0.right.hip = Cartesian3d::new(-1.0, -1.0, 0.0);

        kp0.left.knee = Cartesian3d::new(1.0, 0.0, 0.0);
        kp0.left.ankle = Cartesian3d::new(1.0, 1.0, 0.0);
        // 0°
        tracker.add_keypoints(kp0, 0);
        tracker.add_keypoints(kp0, 100);

        // 45°
        kp0.left.ankle = Cartesian3d::new(0.0, 1.0, 0.0);
        tracker.add_keypoints(kp0, 500);

        // 90°
        kp0.left.ankle = Cartesian3d::new(0.5, 0.0, 0.0);
        tracker.add_keypoints(kp0, 1000);

        tracker.bpm = 60.0;

        let dance = tracker.detected_dance();
        assert_eq!(dance.len(), 1);
        assert_eq!(dance[0].name(), "Test-Step-2");
    }
}
