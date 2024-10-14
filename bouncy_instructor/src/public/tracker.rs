mod detection_output;
mod frame_output;
mod pose_output;
mod step_output;

pub use detection_output::{DetectionFailureReason, DetectionResult, PoseHint};
pub use pose_output::PoseApproximation;
pub use step_output::DetectedStep;

use super::renderable::RenderableSkeleton;
use super::wrapper::skeleton_wrapper::SkeletonWrapper;
use crate::intern::dance_detector::{DanceDetector, DetectionState};
use crate::intern::skeleton_3d::{Direction, Skeleton3d};
use crate::intern::tracker_dance_collection::{ForeignCollectionError, TrackerDanceCollection};
use crate::keypoints::{Cartesian3d, Keypoints};
use crate::skeleton::{Cartesian2d, Skeleton};
use crate::{AudioEffect, StepInfo};
use std::rc::Rc;
use wasm_bindgen::prelude::wasm_bindgen;

type Timestamp = f64;

/// A Tracker gathers skeletons over time and passes it on to a DanceDetector.
#[wasm_bindgen]
pub struct Tracker {
    pub(crate) db: Rc<TrackerDanceCollection>,

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
        let db = TrackerDanceCollection::default();
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
    pub(crate) fn new(
        db: impl Into<Rc<TrackerDanceCollection>>,
        target_step: Option<StepInfo>,
        tracked_beats: Option<u32>,
    ) -> Self {
        Tracker {
            db: db.into(),
            detector: DanceDetector::new(target_step, tracked_beats),
            ..Default::default()
        }
    }

    /// Create a tracker for all known steps.
    #[wasm_bindgen(constructor)]
    pub fn new_from_global_collection() -> Self {
        let db = crate::STATE.with_borrow(|state| Rc::clone(&state.global_db.tracker_view));
        Tracker::new(db, None, None)
    }

    /// Track one specific step, by name, including its variations (with the same name).
    #[wasm_bindgen(js_name = "StepTracker")]
    pub fn new_step_tracker(step_name: String) -> Result<Tracker, ForeignCollectionError> {
        let mut db = TrackerDanceCollection::default();
        crate::STATE.with_borrow(|state| {
            for step in state.global_db.tracker_view.steps_by_name(&step_name) {
                db.add_foreign_step(&state.global_db.tracker_view, &step.id)?;
            }
            // also add idle steps, those should always be included in a tracker
            for step in state.global_db.tracker_view.idle_steps() {
                db.add_foreign_step(&state.global_db.tracker_view, &step.id)?;
            }
            Ok(())
        })?;
        Ok(Tracker::new(db, None, None))
    }

    /// Track one specific step, by ID, excluding its variations (with the same name).
    ///
    /// This is not intended for general dance detection but rather for a
    /// specific training session without much regard for timing etc.
    #[wasm_bindgen(js_name = "UniqueStepTracker")]
    pub fn new_unique_step_tracker(step_id: String) -> Result<Tracker, ForeignCollectionError> {
        let mut db = TrackerDanceCollection::default();
        crate::STATE.with_borrow(|state| {
            db.add_foreign_pose_by_id(&state.global_db.tracker_view, "standing-straight-side");
            db.add_foreign_pose_by_id(&state.global_db.tracker_view, "standing-straight-front");
            db.add_foreign_step(&state.global_db.tracker_view, &step_id)?;
            Ok(())
        })?;
        let step = db.step(&step_id).cloned().expect("just added the step");
        let step_info = StepInfo::from_step(step, &db);

        Ok(Tracker::new(db, Some(step_info), None))
    }

    #[wasm_bindgen(js_name = "finishTracking")]
    pub fn finish_tracking(&mut self) {
        let now = *self.timestamps.last().unwrap_or(&0.0);
        self.detector
            .transition_to_state(DetectionState::TrackingDone, now)
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
        let now = *self.timestamps.last().unwrap_or(&0.0);
        let db = &self.db;
        let skeletons = &self.skeletons;
        self.detector.tick(now, db, skeletons)
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
    pub fn detection_state(&self) -> ReadableDetectionState {
        self.detector.detection_state_store.get_store().into()
    }

    #[wasm_bindgen(getter, js_name = trackedBeats)]
    pub fn tracked_beats(&self) -> Option<u32> {
        self.detector.tracked_beats
    }

    #[wasm_bindgen(js_name = nextHalfBeat)]
    pub fn next_half_beat(&self, now: Option<Timestamp>) -> Timestamp {
        let now = now.unwrap_or_else(|| *self.timestamps.last().unwrap_or(&0.0));
        self.detector.next_pose_time(now)
    }

    #[wasm_bindgen(getter, js_name = timeBetweenPoses)]
    pub fn time_between_poses(&self) -> f64 {
        self.detector.time_between_poses()
    }

    #[wasm_bindgen(js_name = nextAudioEffect)]
    pub fn next_audio_effect(&mut self) -> Option<AudioEffect> {
        self.detector.ui_events.next_audio()
    }

    /// Return a skeleton for a pose.
    #[wasm_bindgen(js_name = poseSkeleton)]
    pub fn pose_skeleton(&self, id: String) -> Option<Skeleton> {
        let index = self.db.pose_by_id(&id)?;
        let pose = &self.db.poses()[index];
        // TODO: set correct direction
        let direction = Direction::East;
        Some(Skeleton::from_pose(pose, &self.db, direction))
    }

    /// Return a skeleton that's expected now.
    ///
    /// Only implemented to work properly for trackers of unique steps.
    ///
    /// (experimenting with live instructor, I probably want to change this when cleaning up the impl)
    #[wasm_bindgen(js_name = expectedPoseSkeleton)]
    pub fn expected_pose_skeleton(&self) -> Skeleton {
        let beat = self.detector.num_detected_poses();
        self.pose_skeleton_at_beat(beat as i32)
    }

    #[wasm_bindgen(js_name = beat)]
    pub fn beat(&self, t: f64) -> i32 {
        self.detector.timestamp_to_beat(t) as i32
            - self
                .detector
                .timestamp_to_beat(self.detector.detection_state_start) as i32
    }

    #[wasm_bindgen(js_name = poseSkeletonAtBeat)]
    pub fn pose_skeleton_at_beat(&self, beat: i32) -> Skeleton {
        let step_info = self.detector.tracked_step();

        match usize::try_from(beat) {
            Ok(beat) => step_info.skeleton(beat),
            _else => step_info.skeleton(0).resting_pose(),
        }
    }

    #[wasm_bindgen(js_name = expectedPoseBodyShift)]
    pub fn expected_pose_body_shift(&self) -> Cartesian2d {
        let beat = self.detector.num_detected_poses();
        self.pose_body_shift_at_beat(beat)
    }

    #[wasm_bindgen(js_name = poseBodyShiftAtBeat)]
    pub fn pose_body_shift_at_beat(&self, beat: usize) -> Cartesian2d {
        let step_info = self.detector.tracked_step();
        step_info.body_shift(beat)
    }

    #[wasm_bindgen(getter, js_name = lastDetection)]
    pub fn last_detection(&self) -> DetectionResult {
        self.detector.detected.clone()
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

    // Note: this probably will eventually replace all uses of `skeleton_at`
    #[wasm_bindgen(js_name = skeletonWrapperAt)]
    pub fn skeleton_wrapper_at(&self, timestamp: Timestamp) -> Option<SkeletonWrapper> {
        let i = self.timestamps.partition_point(|t| *t < timestamp);
        self.keypoints.get(i).map(|kp| SkeletonWrapper::new(*kp))
    }

    /// The original keypoints rendered as skeleton, at the given time frame.
    #[wasm_bindgen(js_name = renderedKeypointsAt)]
    pub fn rendered_keypoints_at(
        &self,
        timestamp: Timestamp,
        width: f32,
        height: f32,
    ) -> Option<RenderableSkeleton> {
        let i = self.timestamps.partition_point(|t| *t < timestamp);
        self.skeletons
            .get(i)
            .map(|skeleton_info| skeleton_info.keypoints_skeleton(width, height))
    }

    #[wasm_bindgen(getter, js_name = halfSpeed)]
    pub fn half_speed(&self) -> bool {
        self.detector.half_speed
    }
}

#[wasm_bindgen(typescript_custom_section)]
const TYPESCRIPT_TYPES: &str = r#"
import type { Readable } from "svelte/store";

type ReadableDetectionState = Readable<DetectionState>;
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ReadableDetectionState")]
    pub type ReadableDetectionState;
}
