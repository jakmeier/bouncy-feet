mod detection_output;
mod frame_output;
mod pose_output;
mod step_output;
mod teacher_output;

pub use detection_output::{DetectionFailureReason, DetectionResult, PoseHint};
pub use pose_output::PoseApproximation;
pub use step_output::DetectedStep;
pub use teacher_output::DanceCursor;
pub use teacher_output::TeacherView;

use super::renderable::RenderableSkeleton;
use super::wrapper::skeleton_wrapper::SkeletonWrapper;
use super::TextEffect;
use crate::intern::dance_detector::{DanceDetector, DetectionState};
use crate::intern::skeleton_3d::{Direction, Skeleton3d};
use crate::intern::step_pace::StepPace;
use crate::intern::teacher::Teacher;
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

    pub(crate) fn new_from_teacher(
        db: impl Into<Rc<TrackerDanceCollection>>,
        teacher: Teacher,
    ) -> Self {
        Tracker {
            db: db.into(),
            detector: DanceDetector::new_from_teacher(teacher),
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

    /// Mix a warmup with the given steps, by name.
    ///
    ///
    #[wasm_bindgen(js_name = "WarmUp")]
    pub fn new_warm_up(
        step_names: Vec<String>,
        num_beats: u32,
    ) -> Result<Tracker, ForeignCollectionError> {
        assert_ne!(
            0,
            step_names.len(),
            "must have at least one step for warmup"
        );
        let mut db = TrackerDanceCollection::default();
        let mut teacher = Teacher::default();
        // TODO: Allow different paces
        let pace1 = StepPace::half_speed();
        let pace2 = StepPace::normal();

        // Warmup structure: Go through all steps in order, then do the first
        // again, after which the warmup is done.
        let fair_beats_per_step = num_beats / (step_names.len() + 1) as u32;
        // Each step should be repeated for some time to follow more easily.
        // If there are too many steps, not all will be shown but still tracked.
        let min_beats_per_step = 8;
        let beats_per_step = fair_beats_per_step.max(min_beats_per_step);

        crate::STATE.with_borrow(|state| {
            let mut beats_to_fill = num_beats - beats_per_step;
            // add idle steps to DB, those should always be included in a tracker
            for step in state.global_db.tracker_view.idle_steps() {
                db.add_foreign_step(&state.global_db.tracker_view, &step.id)?;
            }
            // add all tracked steps to the DB and also to the teacher
            for step_name in &step_names {
                // For now, only show one variation per step.
                let mut step_added = false;
                for step in state.global_db.tracker_view.steps_by_name(step_name) {
                    db.add_foreign_step(&state.global_db.tracker_view, &step.id)?;
                    let local_step = db.step(&step.id).expect("just added step");
                    if !step_added && beats_to_fill >= beats_per_step {
                        let slow = beats_per_step / 2;
                        let fast = beats_per_step - slow;
                        teacher.add_warmup(
                            StepInfo::from_step(local_step.clone(), &db),
                            slow,
                            pace1,
                        );
                        teacher.add_warmup(
                            StepInfo::from_step(local_step.clone(), &db),
                            fast,
                            pace2,
                        );
                        step_added = true;
                        beats_to_fill -= beats_per_step;
                    }
                }
            }
            // Add the first step again to the teacher, as a sign to that it is about to finish.
            let Some(first_step) = db.steps_by_name(&step_names[0]).next() else {
                panic!("Just added step");
            };
            teacher.add_warmup(
                StepInfo::from_step(first_step.clone(), &db),
                beats_per_step,
                pace1,
            );
            Ok(())
        })?;

        Ok(Tracker::new_from_teacher(db, teacher))
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

    #[wasm_bindgen(js_name = currentView)]
    pub fn current_view(&mut self, t: Timestamp) -> TeacherView {
        self.detector.current_view(t)
    }

    #[wasm_bindgen(getter, js_name = detectionState)]
    pub fn detection_state(&self) -> ReadableDetectionState {
        self.detector.detection_state_store.get_store().into()
    }

    #[wasm_bindgen(getter, js_name = trackedSubbeats)]
    pub fn tracked_subbeats(&self) -> u32 {
        self.detector.teacher.tracked_subbeats()
    }

    #[wasm_bindgen(js_name = nextSubbeat)]
    pub fn next_sub_beat(&self, now: Option<Timestamp>) -> Timestamp {
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

    #[wasm_bindgen(js_name = nextTextEffect)]
    pub fn next_text_effect(&mut self, after: Timestamp) -> Option<TextEffect> {
        self.detector.ui_events.next_text(after)
    }

    /// How long the tracked activity is in total, measured in milliseconds.
    #[wasm_bindgen(js_name = duration)]
    pub fn duration(&mut self) -> f64 {
        self.time_between_poses() * self.tracked_subbeats() as f64
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
        self.pose_skeleton_at_subbeat(beat as i32)
    }

    #[wasm_bindgen(js_name = expectedJumpHeight)]
    pub fn expected_jump_height(&self) -> f32 {
        let beat = self.detector.num_detected_poses();
        self.jump_height_at_subbeat(beat as i32)
    }

    pub fn subbeat(&self, t: f64) -> i32 {
        self.detector.timestamp_to_subbeat(t) as i32
            - self
                .detector
                .timestamp_to_subbeat(self.detector.detection_state_start) as i32
    }

    pub fn cursor(&self, t: f64) -> DanceCursor {
        let subbeat = self.subbeat(t);
        self.cursor_at_subbeat(subbeat)
    }

    #[wasm_bindgen(js_name = cursorAtSubbeat)]
    pub fn cursor_at_subbeat(&self, subbeat: i32) -> DanceCursor {
        self.detector
            .teacher
            .cursor_at_subbeat(u32::try_from(subbeat).unwrap_or(0))
    }

    #[wasm_bindgen(js_name = poseSkeletonAt)]
    pub fn pose_skeleton_at(&self, cursor: &DanceCursor) -> Skeleton {
        self.detector
            .step(cursor)
            .map(|step| step.skeleton(cursor.pose_index))
            .unwrap_or_else(|| Skeleton::resting(false))
    }

    // TODO: replace all usage with at cursor calls
    fn pose_skeleton_at_subbeat(&self, subbeat: i32) -> Skeleton {
        u32::try_from(subbeat)
            .ok()
            .and_then(|s| self.detector.tracked_step_with_remainder(s))
            .map(|(step, beat)| step.skeleton(beat as usize))
            .unwrap_or_else(|| Skeleton::resting(false))
    }

    #[wasm_bindgen(js_name = jumpHeight)]
    pub fn jump_height(&self, cursor: &DanceCursor) -> f32 {
        self.detector
            .step(cursor)
            .and_then(|step| step.jump_height(cursor.pose_index))
            .unwrap_or(0.0)
    }

    // TODO: replace all usage with at cursor calls
    fn jump_height_at_subbeat(&self, subbeat: i32) -> f32 {
        u32::try_from(subbeat)
            .ok()
            .and_then(|s| self.detector.tracked_step_with_remainder(s))
            .and_then(|(step, beat)| step.jump_height(beat as usize))
            .unwrap_or(1.0)
    }

    #[wasm_bindgen(js_name = expectedPoseBodyShift)]
    pub fn expected_pose_body_shift(&self) -> Cartesian2d {
        // TODO: Consider dealing with different paces per tracked step.
        let subbeat = self.detector.num_detected_poses();
        self.pose_body_shift_at_subbeat(subbeat)
    }

    #[wasm_bindgen(js_name = poseBodyShift)]
    pub fn pose_body_shift(&self, cursor: &DanceCursor) -> Cartesian2d {
        // TOOD: maybe this could be done more efficiently
        self.pose_body_shift_at_subbeat(cursor.subbeat as usize)
    }

    // TODO: replace all usage with at cursor calls
    fn pose_body_shift_at_subbeat(&self, beat: usize) -> Cartesian2d {
        self.detector
            .teacher
            .pose_body_shift_at_subbeat(beat as u32)
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

    #[wasm_bindgen(js_name = devSetState)]
    pub fn dev_set_state(&mut self, state: DetectionState, timestamp: Timestamp) {
        self.detector.dev_set_state(state, timestamp);
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
