mod detection_output;
mod frame_output;
mod pose_output;
mod step_output;

use detection_output::DetectionFailureReason;
pub use detection_output::DetectionResult;
pub use pose_output::PoseApproximation;
pub use step_output::DetectedStep;

use crate::intern::dance_collection::{DanceCollection, ForeignCollectionError};
use crate::intern::pose::PoseDirection;
use crate::intern::skeleton_3d::Skeleton3d;
use crate::keypoints::{Cartesian3d, Keypoints};
use crate::skeleton::{Cartesian2d, Skeleton};
use crate::StepInfo;
use std::rc::Rc;
use wasm_bindgen::prelude::wasm_bindgen;

type Timestamp = u32;

#[wasm_bindgen]
pub struct Tracker {
    pub(crate) db: Rc<DanceCollection>,

    /// invariant: ordered by timestamp
    pub(crate) timestamps: Vec<Timestamp>,
    /// full keypoints as originally recorded
    pub(crate) keypoints: Vec<Keypoints>,
    /// tracked limbs
    pub(crate) skeletons: Vec<Skeleton3d>,

    // below are "Dance Detector" fields, maybe it should be its own struct?
    pub(crate) bpm: f32,
    pub(crate) error_threshold: f32,
    // todo: head and tail for what was already detected, to not re-compute all every time
    /// (experimenting with live instructor, I probably want to change this when cleaning up the impl)
    /// only set for unique step tracking
    pub(crate) intermediate_result: Option<DetectionResult>,
    pub(crate) last_error: Option<(PoseHint, PoseApproximation)>,

    /// When this is set, pose detection happens on the beat only.
    pub(crate) beat_alignment: Option<Timestamp>,
    /// Enforce that a pose is evaluated on beat, regardless of how well it matches.
    pub(crate) force_beat: bool,
}

#[wasm_bindgen]
pub struct Skeletons {
    pub front: Skeleton,
    pub side: Skeleton,
}

/// Best guess for what the dancer needs to change to fit the pose.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub enum PoseHint {
    DontKnow,
    LeftRight,
    ZOrder,
    WrongDirection,
}

impl Default for Tracker {
    fn default() -> Self {
        let db = DanceCollection::default();
        Tracker {
            db: Rc::new(db),
            // order by timestamp satisfied for empty list
            timestamps: vec![],
            keypoints: vec![],
            skeletons: vec![],
            bpm: 120.0,
            error_threshold: 0.05,
            intermediate_result: None,
            last_error: None,
            beat_alignment: None,
            force_beat: false,
        }
    }
}

#[wasm_bindgen]
impl Tracker {
    pub(crate) fn new(
        db: impl Into<Rc<DanceCollection>>,
        intermediate_result: Option<DetectionResult>,
    ) -> Self {
        Tracker {
            db: db.into(),
            intermediate_result,
            ..Default::default()
        }
    }

    /// Create a tracker for all known steps.
    #[wasm_bindgen(constructor)]
    pub fn new_from_global_collection() -> Self {
        let db = crate::STATE.with_borrow(|state| Rc::clone(&state.db));
        Tracker::new(db, None)
    }

    /// Track one specific step, by name, including its variations (with the same name).
    #[wasm_bindgen(js_name = "StepTracker")]
    pub fn new_step_tracker(step_name: String) -> Result<Tracker, ForeignCollectionError> {
        let mut db = DanceCollection::default();
        crate::STATE.with_borrow(|state| {
            for step in state.db.steps_by_name(&step_name) {
                db.add_foreign_step(&state.db, &step.id)?;
            }
            // also add idle steps, those should always be included in a tracker
            for step in state.db.idle_steps() {
                db.add_foreign_step(&state.db, &step.id)?;
            }
            Ok(())
        })?;
        Ok(Tracker::new(db, None))
    }

    /// Track one specific step, by ID, excluding its variations (with the same name).
    ///
    /// This is not intended for general dance detection but rather for a
    /// specific training session without much regard for timing etc.
    #[wasm_bindgen(js_name = "UniqueStepTracker")]
    pub fn new_unique_step_tracker(step_id: String) -> Result<Tracker, ForeignCollectionError> {
        let mut db = DanceCollection::default();
        crate::STATE.with_borrow(|state| {
            db.add_foreign_step(&state.db, &step_id)?;
            Ok(())
        })?;
        let step = db.step(&step_id).cloned().expect("just added the step");
        let step_info = StepInfo::from_step(step, &db);

        let intermediate_result = DetectionResult::init_for_unique_step_tracker(step_info);
        Ok(Tracker::new(db, Some(intermediate_result)))
    }

    pub fn clear(&mut self) {
        self.keypoints.clear();
        self.timestamps.clear();
        self.skeletons.clear();
        if let Some(intermediate) = self.intermediate_result.as_mut() {
            intermediate.partial = None;
            intermediate.steps.clear();
        }
        self.last_error = None;
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
        let skeleton_info = Skeleton3d::from_keypoints(&keypoints, &self.db);
        let front = skeleton_info.to_skeleton(0.0);
        let side = skeleton_info.to_skeleton(90.0);
        self.skeletons.push(skeleton_info);
        Skeletons { front, side }
    }

    #[wasm_bindgen(js_name = setBpm)]
    pub fn set_bpm(&mut self, bpm: f32) {
        self.bpm = bpm;
    }

    #[wasm_bindgen(js_name = alignBeat)]
    pub fn align_beat(&mut self, first_beat: Timestamp) {
        self.beat_alignment = Some(first_beat);
    }

    #[wasm_bindgen(js_name = enforceBeat)]
    pub fn enforce_beat(&mut self, yes: bool) {
        self.force_beat = yes;
    }

    #[wasm_bindgen(js_name = setErrorThreshold)]
    pub fn set_error_threshold(&mut self, error_threshold: f32) {
        self.error_threshold = error_threshold;
    }

    /// Goes over all data and detects the best fitting dance.
    ///
    /// There is no re-use or consistency between calls. It always starts at 0
    /// and computes the global best fit.
    ///
    /// Use [`Tracker::detect_next_pose`] for incremental detection.
    #[wasm_bindgen(js_name = detectDance)]
    pub fn detect_dance(&self) -> DetectionResult {
        let mut start = 0;
        let end = self.timestamps.len();

        let mut out = vec![];
        while let Some(step) = self.find_first_step(start, end) {
            let end_t = step.end;
            start = start + self.timestamps[start..end].partition_point(|t| *t <= end_t);
            out.push(step);
        }
        DetectionResult::new(out)
    }

    /// Take a previous detection and try adding one more pose to it.
    ///
    /// For now this only looks at the very last frame, but this is an
    /// implementation detail. Callers should assume it reads everything since
    /// the last detected step.
    #[wasm_bindgen(js_name = detectNextPose)]
    pub fn detect_next_pose(&mut self) -> DetectionResult {
        let prev_detection = self
            .intermediate_result
            .as_ref()
            .expect("requires intermediate_result");

        let end_t = *self.timestamps.last().unwrap_or(&0);
        let last_step = prev_detection
            .partial
            .as_ref()
            .or_else(|| prev_detection.steps.last());
        let start_t = last_step.map_or(0, |step| step.end);

        // skip at least a quarter beat
        let min_delay = (1000.0 / (self.bpm * 4.0 / 60.0)).round() as u32;
        if end_t < start_t + min_delay {
            return prev_detection
                .clone()
                .with_failure_reason(DetectionFailureReason::TooEarly);
        }

        // check we are on beat, if aligned to beat
        if let Some(first_beat) = self.beat_alignment {
            let half_beat = 1000.0 / (self.bpm * 2.0 / 60.0);
            let t_total = end_t - first_beat;
            let relative_beat_distance = (t_total as f32 / half_beat).fract();
            if relative_beat_distance > 0.2 {
                return prev_detection
                    .clone()
                    .with_failure_reason(DetectionFailureReason::NotOnBeat);
            }
        }

        if let Some(tracked_skeleton) = self.skeletons.last() {
            let beat = self.expected_pose_beat();
            let step_info = self.tracked_step();
            let step = self
                .db
                .step(&step_info.id())
                .expect("tracked step must exist");

            let pose_idx = step.poses[beat % step.poses.len()];
            let pose = &self.db.poses()[pose_idx];

            // If we detected a different direction than the expected one, it
            // might make more sense to compare with the original angles rather
            // than the normalized angles.
            let has_direction_error = !pose
                .direction
                .matches_direction(tracked_skeleton.direction());
            let error_details = if has_direction_error && pose.direction == PoseDirection::Front {
                let original_angles = tracked_skeleton.original_angles();
                pose.error(&original_angles, tracked_skeleton.positions())
            } else {
                pose.error(tracked_skeleton.angles(), tracked_skeleton.positions())
            };

            let error = error_details.error_score();
            let has_z_error = !error_details.z_order_errors.is_empty();
            let pose_approximation = PoseApproximation {
                name: self.db.pose_name(pose_idx).to_owned(),
                error,
                timestamp: end_t,
                error_details,
            };
            if !has_z_error && error < self.error_threshold {
                self.add_pose(pose_approximation);
                self.last_error = None;
            } else {
                let hint = {
                    if has_z_error {
                        PoseHint::ZOrder
                    } else {
                        let left_right_pose = self.db.pose_left_right_switched(pose_idx);
                        let lr_error = left_right_pose
                            .error(tracked_skeleton.angles(), tracked_skeleton.positions());
                        let lr_error_score = lr_error.error_score();
                        // TODO: fine-tune 0.5
                        if lr_error_score < error * 0.5 {
                            PoseHint::LeftRight
                        } else if has_direction_error {
                            PoseHint::WrongDirection
                        } else {
                            PoseHint::DontKnow
                        }
                    }
                };
                self.last_error = Some((hint, pose_approximation));
            }
        }

        let mut detection_result = self.intermediate_result.clone().unwrap();
        if let Some((_hint, pose_approximation)) = &self.last_error {
            // Provide extra information about why there is an error, which is a
            // high pose error score if we haven't returned earlier.
            detection_result =
                detection_result.with_failure_reason(DetectionFailureReason::WrongPose);
            // Despite the error, if forced, the pose should still be added to the detection.
            if self.force_beat {
                self.add_pose(pose_approximation.clone());
            }
        }
        detection_result
    }

    #[wasm_bindgen(js_name = poseHint)]
    pub fn pose_hint(&self) -> PoseHint {
        match &self.last_error {
            Some((hint, _err)) => *hint,
            None => PoseHint::DontKnow,
        }
    }

    #[wasm_bindgen(js_name = currentPoseError)]
    pub fn current_pose_error(&self) -> Option<PoseApproximation> {
        self.last_error.as_ref().map(|(_hint, err)| err.clone())
    }

    /// Return a skeleton that's expected next.
    ///
    /// Only implemented to work properly for trackers of unique steps.
    ///
    /// (experimenting with live instructor, I probably want to change this when cleaning up the impl)
    #[wasm_bindgen(js_name = expectedPoseSkeleton)]
    pub fn expected_pose_skeleton(&self) -> Skeleton {
        let beat = self.expected_pose_beat();
        let step_info = self.tracked_step();
        step_info.skeleton(beat)
    }

    #[wasm_bindgen(js_name = expectedPoseBodyShift)]
    pub fn expected_pose_body_shift(&self) -> Cartesian2d {
        let beat = self.expected_pose_beat();
        let step_info = self.tracked_step();
        step_info.body_shift(beat)
    }

    #[wasm_bindgen(js_name = numDetectedPoses)]
    pub fn num_detected_poses(&self) -> u32 {
        if let Some(detection) = &self.intermediate_result {
            let mut out = 0;
            for step in &detection.steps {
                out += step.poses.len();
            }
            if let Some(partial) = &detection.partial {
                out += partial.poses.len();
            }
            out as u32
        } else {
            0
        }
    }

    #[wasm_bindgen(js_name = hipPosition)]
    pub fn hip_position(&self, timestamp: Timestamp) -> Cartesian3d {
        let i = self.timestamps.partition_point(|t| *t < timestamp);
        self.keypoints
            .get(i)
            .map(|kp| (kp.left.hip + kp.right.hip) * 0.5)
            .unwrap_or_default()
    }

    /// Fit frames in a time interval against all poses and return the best fit.
    ///
    /// This API is exported mostly for debugging. To extract fitted dances, use
    /// `detect_dance` instead.
    #[wasm_bindgen(js_name = bestFitPose)]
    pub fn best_fit_pose(
        &self,
        start: Timestamp,
        end: Timestamp,
    ) -> Option<pose_output::PoseApproximation> {
        let first = self.timestamps.partition_point(|t| *t < start);
        let last = self.timestamps.partition_point(|t| *t <= end);
        if first == last {
            return None;
        }
        self.best_fit_pose_impl(first, last)
    }

    /// Fit a single frame against all poses and return all errors
    #[wasm_bindgen(js_name = allPoseErrors)]
    pub fn all_pose_errors(&self, timestamp: Timestamp) -> Vec<pose_output::PoseApproximation> {
        self.all_pose_approximations(timestamp)
    }

    #[wasm_bindgen(js_name = skeletonAt)]
    pub fn skeleton_at(&self, timestamp: Timestamp) -> Option<Skeleton> {
        let i = self.timestamps.partition_point(|t| *t < timestamp);
        self.skeletons
            .get(i)
            .map(|skeleton_info| skeleton_info.to_skeleton(0.0))
    }
}

impl Tracker {
    /// Return a pose that's expected next.
    ///
    /// Only implemented to work properly for trackers of unique steps.
    ///
    /// returns a beat number
    /// (experimenting with live instructor, I probably want to change this when cleaning up the impl)
    fn expected_pose_beat(&self) -> usize {
        let detection = self
            .intermediate_result
            .as_ref()
            .expect("requires intermediate_result");

        let full = detection.steps.len()
            * detection
                .target_step
                .as_ref()
                .expect("must have target step")
                .beats();

        let partial = if let Some(partial_step) = &detection.partial {
            partial_step.poses.len()
        } else {
            0
        };
        full + partial
    }

    /// Returns the single tracked step.
    ///
    /// (experimenting with live instructor, I probably want to change this when cleaning up the impl)
    fn tracked_step(&self) -> StepInfo {
        let detection = self
            .intermediate_result
            .as_ref()
            .expect("requires intermediate_result");
        detection.target_step.clone().expect("requires target_step")
    }
}
