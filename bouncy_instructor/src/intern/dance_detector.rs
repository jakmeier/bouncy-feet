use svelte_store::Readable;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::tracker::{DanceCursor, PoseApproximation, TeacherView};
use crate::ui_event::UiEvents;
use crate::{DetectionFailureReason, DetectionResult, PoseHint, StepInfo};

use super::pose::PoseDirection;
use super::skeleton_3d::Skeleton3d;
use super::step_pace::StepPace;
use super::teacher::Teacher;
use super::tracker_dance_collection::TrackerDanceCollection;

type Timestamp = f64;

/// Contains all information about a dance to be detected and has an interface
/// to be used by a Tracker to match tracked skeletons to it.
#[wasm_bindgen]
pub(crate) struct DanceDetector {
    // config
    pub(crate) bpm: f32,
    pub(crate) error_threshold: f32,
    /// When this is set, pose detection happens on the beat only.
    pub(crate) beat_alignment: Option<Timestamp>,
    /// The timestamp of when the beat zero was.
    pub(crate) beat_zero: Option<Timestamp>,
    /// Enforce that a pose is evaluated on beat, regardless of how well it matches.
    pub(crate) force_beat: bool,
    /// How long it takes from a movement of the person on camera to be visible
    /// in an image.
    pub(crate) camera_input_delay: f64,

    /// picks steps, switches between views, etc
    pub(crate) teacher: Teacher,

    // state
    /// Data about detection so far.
    pub(crate) detected: DetectionResult,
    /// Within the configured tolerance, all skeletons timed on the beat are
    /// collected here until the tolerance time frame is surpassed or a good
    /// match is found.
    on_beat_candidates: Vec<PoseApproximation>,
    /// State machine of the detector.
    detection_state: DetectionState,
    /// A svelte store that can be subscribed to for state updates.
    pub(crate) detection_state_store: Readable<DetectionState>,
    /// When the tracker entered the current state.
    pub(crate) detection_state_start: Timestamp,
    /// The timestamp of the last skeleton evaluated, to avoid duplicated work on tick.
    pub(crate) last_evaluation: Timestamp,
    pub(crate) ui_events: UiEvents,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum DetectionState {
    /// Neutral state, not detecting anything.
    Init = 1,
    /// Dance is positioning themselves, detecting the idle position.
    Positioning = 2,
    /// About to go over to live tracking, playing a countdown audio.
    CountDown = 3,
    /// Tracking current movements.
    LiveTracking = 4,
    /// The instructor is showing the next moving.
    InstructorDemo = 5,
    /// No longer tracking but the results of the previous tracking are
    /// available.
    TrackingDone = 6,
}

impl Default for DanceDetector {
    fn default() -> Self {
        Self {
            bpm: 120.0,
            error_threshold: 0.075,
            detected: DetectionResult::default(),
            beat_alignment: None,
            beat_zero: None,
            force_beat: false,
            // This is what I measured on my desktop with my webcam by looking
            // at the timestamps of claps timed on the audio output. Basically,
            // I measured the remaining error in timing after I have considered
            // audio output latency and computational overhead. TODO: have some
            // kind of estimate run on the device Note: Set to a small value for
            // now, it seems more stable to underestimate and then just use a
            // large tolerance.
            camera_input_delay: 50.0,
            detection_state: DetectionState::Init,
            detection_state_store: Readable::new(DetectionState::Init),
            detection_state_start: 0.0,
            last_evaluation: -0.1,
            ui_events: UiEvents::default(),
            on_beat_candidates: vec![],
            teacher: Default::default(),
        }
    }
}

impl DanceDetector {
    pub(crate) fn new(target_step: Option<StepInfo>, tracked_beats: Option<u32>) -> Self {
        let mut teacher = Teacher::default();

        if let Some(step) = target_step {
            teacher.add_step(step, 1, StepPace::half_speed());
        } else {
            let beats = tracked_beats.unwrap_or(64);
            teacher.add_freestyle(beats);
        }

        Self::new_from_teacher(teacher)
    }

    pub(crate) fn new_from_teacher(teacher: Teacher) -> Self {
        Self {
            teacher,
            ..Default::default()
        }
    }

    pub(crate) fn clear(&mut self) {
        self.detected.partial = None;
        self.detected.steps.clear();
        self.detected.last_error = None;
        self.detected.failure_reason = None;
        self.detected.pose_matches = 0;
        self.detected.pose_misses = 0;
        self.beat_zero = None;
        self.transition_to_state(DetectionState::Init, self.detection_state_start);
    }

    /// Make progress, depending on detection state and new data added since last tick.
    pub(crate) fn tick(
        &mut self,
        now: Timestamp,
        db: &TrackerDanceCollection,
        skeletons: &[Skeleton3d],
    ) -> DetectionResult {
        match self.detection_state {
            DetectionState::Init => {
                self.transition_to_state(DetectionState::Positioning, now);
            }
            DetectionState::Positioning => {
                if let Some((target, _)) = self.teacher.step_at_subbeat(0) {
                    if let Some(skeleton) = skeletons.last() {
                        let resting_pose_idx = if target.skeleton(0).sideway {
                            db.pose_by_id("standing-straight-side")
                                .expect("missing resting pose")
                        } else {
                            db.pose_by_id("standing-straight-front")
                                .expect("missing side resting pose")
                        };
                        let resting_pose = &db.poses()[resting_pose_idx];
                        let error_details = resting_pose.skeleton_error(skeleton);
                        if error_details.error_score() < 0.075 {
                            self.transition_to_state(DetectionState::CountDown, now);
                        }
                    }
                }
            }
            DetectionState::CountDown => {
                let time_between_poses = self.subbeat_time();
                if now > self.detection_state_start + (time_between_poses * 15.0).floor() {
                    let beat_zero = self.next_subbeat_timestamp(now);
                    self.beat_zero = Some(beat_zero);
                    // the change to the next state must happen BEFORE it
                    // actually starts, to give time to the animation
                    let actual_start = beat_zero + time_between_poses;
                    self.transition_to_state(DetectionState::LiveTracking, actual_start);
                }
            }
            DetectionState::LiveTracking => {
                if self.last_evaluation == now {
                    return self
                        .detected
                        .clone()
                        .with_failure_reason(DetectionFailureReason::NoNewData);
                }

                // Finish activity when the teacher is done.
                self.last_evaluation = now;
                let subbeat = self.subbeat(now);
                if self.teacher.is_done(subbeat) {
                    self.transition_to_state(DetectionState::TrackingDone, now);
                }
                // Change state to "InstructorDemo" if there is currently no tracking going on.
                else if !self.teacher.is_tracking_at_subbeat(subbeat) {
                    self.transition_to_state(DetectionState::InstructorDemo, now);
                }

                if let Some(skeleton) = skeletons.last() {
                    return self.detect_next_pose(db, skeleton, now);
                } else {
                    return self
                        .detected
                        .clone()
                        .with_failure_reason(DetectionFailureReason::NoData);
                }
            }
            DetectionState::InstructorDemo => {
                let subbeat = self.subbeat(now);
                if self.teacher.is_tracking_at_subbeat(subbeat) {
                    self.transition_to_state(DetectionState::LiveTracking, now);
                }
            }
            DetectionState::TrackingDone => (),
        }
        self.detected
            .clone()
            .with_failure_reason(DetectionFailureReason::DetectionDisabled)
    }

    /// Take a previous detection and try adding one more pose to it.
    pub fn detect_next_pose(
        &mut self,
        db: &TrackerDanceCollection,
        skeleton: &Skeleton3d,
        pose_timestamp: Timestamp,
    ) -> DetectionResult {
        let prev_detection = &mut self.detected;
        let last_step = prev_detection
            .partial
            .as_ref()
            .or_else(|| prev_detection.steps.last());
        let prev_t = last_step.map_or(0.0, |step| step.end);

        // skip at least a quarter beat
        let min_delay = self.subbeat_time() / 2.0;
        if pose_timestamp < prev_t + min_delay {
            return self
                .detected
                .clone()
                .with_failure_reason(DetectionFailureReason::TooEarly);
        }

        // check we are on beat, if aligned to beat
        let time_delta = self.subbeat_time();
        // let first_beat =
        //     self.next_subbeat_timestamp(self.detection_state_start + (time_delta / 2.0));
        let first_beat = self.next_subbeat_timestamp(self.detection_state_start);
        let next_subbeat = self.recorded_subbeats();

        let expected_next_pose_t =
            first_beat + (next_subbeat as f64 * time_delta) + self.camera_input_delay;
        if self.force_beat && pose_timestamp < expected_next_pose_t - self.beat_tolerance() {
            return self
                .detected
                .clone()
                .with_failure_reason(DetectionFailureReason::NotOnBeat);
        }
        let cursor = self.teacher.cursor_at_subbeat(next_subbeat);

        // look up step data we expect to match
        let step_id = match self.step(&cursor) {
            Some(step_info) => step_info.id(),
            None => {
                // TODO: To support freestyle, this should match against any step
                return self
                    .detected
                    .clone()
                    .with_failure_reason(DetectionFailureReason::NoTrackingTarget);
            }
        };
        let step = db.step(&step_id).expect("tracked step must exist");

        let pose_idx = step.poses[cursor.pose_index % step.poses.len()];
        let pose = &db.poses()[pose_idx];

        // If we detected a different direction than the expected one, it
        // might make more sense to compare with the original angles rather
        // than the normalized angles.
        let has_direction_error = !pose.direction.matches_direction(skeleton.direction());
        let error_details = if has_direction_error && pose.direction == PoseDirection::Front {
            let original_angles = skeleton.original_angles();
            pose.error(&original_angles, skeleton.positions(), skeleton.direction())
        } else {
            pose.skeleton_error(skeleton)
        };

        let error = error_details.error_score();
        let has_z_error = !error_details.z_order_errors.is_empty();
        let pose_approximation = PoseApproximation {
            id: db.pose_id(pose_idx).to_owned(),
            name: db.pose_name(pose_idx).to_owned(),
            error,
            timestamp: pose_timestamp,
            error_details,
        };
        if !has_z_error && error < self.error_threshold {
            self.add_pose(pose_approximation);
            self.detected.last_error = None;
            self.detected.pose_matches += 1;
        } else {
            let hint = {
                if has_z_error {
                    PoseHint::ZOrder
                } else {
                    let left_right_pose = db.pose_left_right_switched(pose_idx);
                    let lr_error = left_right_pose.skeleton_error(skeleton);
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
            // Despite the error, if forced, a pose should still be added to the
            // detection after the tolerated deviation. In that case, select the
            // smallest error in the tolerated range.
            if self.force_beat {
                self.on_beat_candidates.push(pose_approximation.clone());
                if pose_timestamp > expected_next_pose_t + self.beat_tolerance() {
                    let closest_fit = self
                        .on_beat_candidates
                        .drain(..)
                        .min_by(|a, b| f32::total_cmp(&a.error, &b.error));
                    // just added a pose above, min() can't be empty
                    let pose_approximation =
                        closest_fit.expect("on_beat_candidates shouldn't be empty");
                    self.add_pose(pose_approximation);
                    self.detected.pose_misses += 1;
                }
            }
            self.detected.last_error = Some((hint, pose_approximation));
        }

        let mut detection_result = self.detected.clone();
        if self.detected.last_error.is_some() {
            // Provide extra information about why there is an error, which is a
            // high pose error score if we haven't returned earlier.
            detection_result =
                detection_result.with_failure_reason(DetectionFailureReason::WrongPose);
        }
        detection_result
    }

    pub(crate) fn current_view(&mut self, t: Timestamp) -> TeacherView {
        match self.detection_state {
            DetectionState::Init | DetectionState::Positioning => TeacherView::CameraOnly,
            DetectionState::CountDown
            | DetectionState::LiveTracking
            | DetectionState::InstructorDemo => {
                let subbeat = self.subbeat(t);
                self.teacher.ui_view_at_subbeat(subbeat)
            }
            DetectionState::TrackingDone => TeacherView::Off,
        }
    }

    /// Returns the tracked step and subbeat for a given subbeat after tracking started.
    pub(crate) fn tracked_step_with_remainder(&self, subbeat: u32) -> Option<(&StepInfo, u32)> {
        self.teacher.step_at_subbeat(subbeat)
    }

    /// Calculate the subbeat since the current detection phase
    pub(crate) fn subbeat(&self, t: f64) -> u32 {
        self.timestamp_to_subbeat_from_zero(t)
            .saturating_sub(self.timestamp_to_subbeat_from_zero(self.detection_state_start))
    }

    pub(crate) fn step(&self, cursor: &DanceCursor) -> Option<&StepInfo> {
        self.teacher.step(cursor)
    }

    pub(crate) fn add_pose(&mut self, pose: PoseApproximation) {
        let pose_duration = self
            .teacher
            .pose_duration(&self.detected.cursor)
            .unwrap_or_else(|| {
                crate::println!("warn: adding more poses than were expected");
                1
            });
        let prev_subbeat = self.detected.cursor.subbeat;
        let new_subbeat = prev_subbeat + pose_duration;

        self.detected.add_pose(pose);

        self.on_beat_candidates.clear();
        if let Some((target_step, _beat)) = &self.teacher.step_at_subbeat(prev_subbeat) {
            self.detected.match_step(target_step);
        }

        let new_cursor = self.teacher.cursor_at_subbeat(new_subbeat);
        self.detected.cursor = new_cursor;
    }

    pub(crate) fn transition_to_state(&mut self, state: DetectionState, t: Timestamp) {
        self.detection_state = state;
        self.detection_state_start = t;
        self.detection_state_store.set(state);

        match state {
            DetectionState::Init => (),
            DetectionState::Positioning => (),
            DetectionState::CountDown => self.emit_countdown_audio(t),
            DetectionState::LiveTracking => (),
            DetectionState::InstructorDemo => (),
            DetectionState::TrackingDone => (),
        }
    }

    pub(crate) fn subbeat_time(&self) -> f64 {
        30_000.0 / self.bpm as f64
    }

    /// How much time before or after the actual beat a pose can be to be
    /// considered on beat
    pub(crate) fn beat_tolerance(&self) -> f64 {
        // TODO: vary this by current pace
        // (This is somewhat high to make it easier to get good scores)
        self.subbeat_time() * 1.5
    }

    pub(crate) fn recorded_subbeats(&self) -> u32 {
        self.detected.cursor.subbeat
    }

    /// Attention: This is not from the start of tracking but from the beat alignment
    fn timestamp_to_subbeat_from_zero(&self, t: Timestamp) -> u32 {
        let t0 = self.beat_zero.unwrap_or(
            self.beat_alignment
                .unwrap_or(self.next_subbeat_timestamp(0.0)),
        );
        let pose_duration = self.subbeat_time();
        ((t - t0) / pose_duration).floor().max(0.0) as u32
    }

    pub(crate) fn next_subbeat_timestamp(&self, not_before: Timestamp) -> Timestamp {
        let t0 = self.beat_alignment.unwrap_or(0.0);
        let subbeat_duration = self.subbeat_time();
        let subbeats = ((not_before - t0) / subbeat_duration).ceil();
        t0 + subbeats * subbeat_duration
    }

    pub(crate) fn emit_countdown_audio(&mut self, not_before: Timestamp) {
        let beat = self.subbeat_time();
        let next_beat = self.next_subbeat_timestamp(not_before);
        // long enough to not clear too early
        let text_dur = 8.0 * beat;

        self.ui_events.add_audio(next_beat, "and".to_owned());

        self.ui_events.add_audio(next_beat + beat, "one".to_owned());
        self.ui_events
            .add_text(next_beat + beat, "4".to_owned(), text_dur);

        self.ui_events
            .add_audio(next_beat + 5.0 * beat, "two".to_owned());
        self.ui_events
            .add_text(next_beat + 5.0 * beat, "3".to_owned(), text_dur);

        self.ui_events
            .add_audio(next_beat + 9.0 * beat, "one".to_owned());
        self.ui_events
            .add_text(next_beat + 9.0 * beat, "2".to_owned(), text_dur);

        self.ui_events
            .add_audio(next_beat + 11.0 * beat, "two".to_owned());
        self.ui_events
            .add_audio(next_beat + 13.0 * beat, "three".to_owned());
        self.ui_events
            .add_text(next_beat + 13.0 * beat, "1".to_owned(), beat);

        self.ui_events
            .add_audio(next_beat + 15.0 * beat, "four".to_owned());
    }

    /// For debugging pruposes, set the state directly.
    pub(crate) fn dev_set_state(&mut self, state: DetectionState, t: Timestamp) {
        self.transition_to_state(state, t);
    }
}
