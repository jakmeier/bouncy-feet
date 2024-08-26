mod detection_output;
mod frame_output;
mod pose_output;
mod step_output;

pub use detection_output::{DetectionFailureReason, DetectionResult, PoseHint};
pub use pose_output::PoseApproximation;
pub use step_output::DetectedStep;

use crate::intern::dance_collection::{DanceCollection, ForeignCollectionError};
use crate::intern::dance_detector::{DanceDetector, DetectionState};
use crate::intern::skeleton_3d::Skeleton3d;
use crate::keypoints::{Cartesian3d, Keypoints};
use crate::skeleton::{Cartesian2d, Skeleton};
use crate::{AudioEffect, StepInfo};
use std::rc::Rc;
use wasm_bindgen::prelude::wasm_bindgen;

type Timestamp = u32;

/// A Tracker gathers skeletons over time and passes it on to a DanceDetector.
#[wasm_bindgen]
pub struct Tracker {
    pub(crate) db: Rc<DanceCollection>,

    /// invariant: ordered by timestamp
    pub(crate) timestamps: Vec<Timestamp>,
    /// full keypoints as originally recorded
    pub(crate) keypoints: Vec<Keypoints>,
    /// tracked limbs
    pub(crate) skeletons: Vec<Skeleton3d>,
    /// state for registered dance to detect
    pub(crate) detector: DanceDetector,
}

#[wasm_bindgen]
pub struct Skeletons {
    pub front: Skeleton,
    pub side: Skeleton,
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
            detector: Default::default(),
        }
    }
}

#[wasm_bindgen]
impl Tracker {
    pub(crate) fn new(db: impl Into<Rc<DanceCollection>>, target_step: Option<StepInfo>) -> Self {
        Tracker {
            db: db.into(),
            detector: DanceDetector::new(target_step),
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
            db.add_foreign_pose_by_id(&state.db, "standing-straight-side");
            db.add_foreign_pose_by_id(&state.db, "standing-straight-front");
            db.add_foreign_step(&state.db, &step_id)?;
            Ok(())
        })?;
        let step = db.step(&step_id).cloned().expect("just added the step");
        let step_info = StepInfo::from_step(step, &db);

        Ok(Tracker::new(db, Some(step_info)))
    }

    pub fn clear(&mut self) {
        self.keypoints.clear();
        self.timestamps.clear();
        self.skeletons.clear();
        self.detector.clear();
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
        self.detector.bpm = bpm;
    }

    #[wasm_bindgen(js_name = alignBeat)]
    pub fn align_beat(&mut self, first_beat: Timestamp) {
        self.detector.beat_alignment = Some(first_beat);
    }

    #[wasm_bindgen(js_name = enforceBeat)]
    pub fn enforce_beat(&mut self, yes: bool) {
        self.detector.force_beat = yes;
    }

    #[wasm_bindgen(js_name = setErrorThreshold)]
    pub fn set_error_threshold(&mut self, error_threshold: f32) {
        self.detector.error_threshold = error_threshold;
    }

    /// Goes over all data and detects the best fitting dance.
    ///
    /// There is no re-use or consistency between calls. It always starts at 0
    /// and computes the global best fit.
    ///
    /// Use [`Tracker::run_detection`] for incremental detection.
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

    #[wasm_bindgen(js_name = runDetection)]
    pub fn run_detection(&mut self) -> DetectionResult {
        let now = *self.timestamps.last().unwrap_or(&0);

        match self.detector.detection_state {
            DetectionState::Init => {
                self.detector
                    .transition_to_state(DetectionState::Positioning, now);
            }
            DetectionState::Positioning => {
                if let Some(target) = &self.detector.target_step {
                    if let Some(skeleton) = self.skeletons.last() {
                        let resting_pose_idx = if target.skeletons[0].sideway {
                            self.db
                                .pose_by_id("standing-straight-side")
                                .expect("missing resting pose")
                        } else {
                            self.db
                                .pose_by_id("standing-straight-front")
                                .expect("missing side resting pose")
                        };
                        let resting_pose = &self.db.poses()[resting_pose_idx];
                        let error_details = resting_pose.skeleton_error(skeleton);
                        if error_details.error_score() < 0.001 {
                            self.detector
                                .transition_to_state(DetectionState::CountDown, now);
                            self.detector.emit_countdown_audio(now);
                        }
                    }
                }
            }
            DetectionState::CountDown => {
                if now
                    > self.detector.detection_state_start
                        + (self.detector.half_beat_duration() * 16.0).floor() as u32
                {
                    self.detector
                        .transition_to_state(DetectionState::LiveTracking, now);
                }
            }
            DetectionState::LiveTracking => {
                if let Some(skeleton) = self.skeletons.last() {
                    return self.detector.detect_next_pose(&self.db, skeleton, now);
                } else {
                    return DetectionResult::default()
                        .with_failure_reason(DetectionFailureReason::NoData);
                }
                // TODO: allow setting a timer
            }
            DetectionState::TrackingDone => (),
        }
        DetectionResult::default().with_failure_reason(DetectionFailureReason::DetectionDisabled)
    }

    #[wasm_bindgen(js_name = poseHint)]
    pub fn pose_hint(&self) -> PoseHint {
        self.detector.detected.pose_hint()
    }

    #[wasm_bindgen(js_name = currentPoseError)]
    pub fn current_pose_error(&self) -> Option<PoseApproximation> {
        self.detector.detected.pose_error()
    }

    #[wasm_bindgen(getter, js_name = detectionState)]
    pub fn detection_state(&self) -> DetectionState {
        self.detector.detection_state
    }

    #[wasm_bindgen(js_name = nextAudioEffect)]
    pub fn next_audio_effect(&mut self) -> Option<AudioEffect> {
        self.detector.ui_events.next_audio()
    }

    /// Return a skeleton that's expected next.
    ///
    /// Only implemented to work properly for trackers of unique steps.
    ///
    /// (experimenting with live instructor, I probably want to change this when cleaning up the impl)
    #[wasm_bindgen(js_name = expectedPoseSkeleton)]
    pub fn expected_pose_skeleton(&self) -> Skeleton {
        let beat = self.detector.expected_pose_beat();
        let step_info = self.detector.tracked_step();
        step_info.skeleton(beat)
    }

    #[wasm_bindgen(js_name = expectedPoseBodyShift)]
    pub fn expected_pose_body_shift(&self) -> Cartesian2d {
        let beat = self.detector.expected_pose_beat();
        let step_info = self.detector.tracked_step();
        step_info.body_shift(beat)
    }

    #[wasm_bindgen(js_name = numDetectedPoses)]
    pub fn num_detected_poses(&self) -> u32 {
        let mut out = 0;
        for step in &self.detector.detected.steps {
            out += step.poses.len();
        }
        if let Some(partial) = &self.detector.detected.partial {
            out += partial.poses.len();
        }
        out as u32
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
