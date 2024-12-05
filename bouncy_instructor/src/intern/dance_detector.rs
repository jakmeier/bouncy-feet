use svelte_store::Readable;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::tracker::PoseApproximation;
use crate::ui_event::UiEvents;
use crate::{DetectionFailureReason, DetectionResult, PoseHint, StepInfo};

use super::pose::PoseDirection;
use super::skeleton_3d::Skeleton3d;
use super::tracker_dance_collection::TrackerDanceCollection;

type Timestamp = f64;

/// Contains all information about a dance to be detected and has an interface
/// to be used by a Tracker to match tracked skeletons to it.
#[wasm_bindgen]
pub(crate) struct DanceDetector {
    // config
    pub(crate) bpm: f32,
    pub(crate) half_speed: bool,
    pub(crate) error_threshold: f32,
    /// When this is set, pose detection happens on the beat only.
    pub(crate) beat_alignment: Option<Timestamp>,
    /// Enforce that a pose is evaluated on beat, regardless of how well it matches.
    pub(crate) force_beat: bool,
    /// How much time before or after the actual beat a pose can be to be
    /// considered on beat
    pub(crate) beat_tolerance: f64,
    /// How long it takes from a movement of the person on camera to be visible
    /// in an image.
    pub(crate) camera_input_delay: f64,
    /// The expected step for unique step tracking
    pub(crate) target_step: Option<StepInfo>,
    /// How many beats to track for, counting only in LiveTracking state.
    pub(crate) tracked_beats: Option<u32>,

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
    /// No longer tracking but the results of the previous tracking are
    /// available.
    TrackingDone = 5,
}

impl Default for DanceDetector {
    fn default() -> Self {
        Self {
            bpm: 120.0,
            half_speed: false,
            error_threshold: 0.05,
            detected: DetectionResult::default(),
            target_step: None,
            tracked_beats: None,
            beat_alignment: None,
            force_beat: false,
            // TODO: should this depend on bpm and system info?
            beat_tolerance: 250.0,
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
        }
    }
}

impl DanceDetector {
    pub(crate) fn new(target_step: Option<StepInfo>, tracked_beats: Option<u32>) -> Self {
        Self {
            target_step,
            tracked_beats,
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
                if let Some(target) = &self.target_step {
                    if let Some(skeleton) = skeletons.last() {
                        let resting_pose_idx = if target.skeletons[0].sideway {
                            db.pose_by_id("standing-straight-side")
                                .expect("missing resting pose")
                        } else {
                            db.pose_by_id("standing-straight-front")
                                .expect("missing side resting pose")
                        };
                        let resting_pose = &db.poses()[resting_pose_idx];
                        let error_details = resting_pose.skeleton_error(skeleton);
                        if error_details.error_score() < 0.05 {
                            self.transition_to_state(DetectionState::CountDown, now);
                            self.emit_countdown_audio(now);
                        }
                    }
                }
            }
            DetectionState::CountDown => {
                let time_between_poses = self.time_between_poses();
                if now > self.detection_state_start + (time_between_poses * 8.0).floor() {
                    // the change to the next state must happen BEFORE it
                    // actually starts, to give time to the animation
                    let actual_start = self.next_pose_time(now + time_between_poses);
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
                self.last_evaluation = now;
                if let Some(num_beats) = self.tracked_beats {
                    if self.num_detected_poses() as u32 >= num_beats * self.poses_per_beat() {
                        self.transition_to_state(DetectionState::TrackingDone, now);
                    }
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
        let start_t = last_step.map_or(0.0, |step| step.end);

        // skip at least a quarter beat
        let min_delay = self.time_between_poses() / 2.0;
        if pose_timestamp < start_t + min_delay {
            return self
                .detected
                .clone()
                .with_failure_reason(DetectionFailureReason::TooEarly);
        }

        // check we are on beat, if aligned to beat
        let num_detected_poses = self.num_detected_poses();
        let time_delta = self.time_between_poses();
        let first_beat = self.next_pose_time(self.detection_state_start + (time_delta / 2.0));
        let expected_next_pose =
            first_beat + (num_detected_poses as f64 * time_delta).round() + self.camera_input_delay;
        if self.force_beat && pose_timestamp < expected_next_pose - self.beat_tolerance {
            return self
                .detected
                .clone()
                .with_failure_reason(DetectionFailureReason::NotOnBeat);
        }

        let step_info = self.tracked_step();
        let step = db.step(&step_info.id()).expect("tracked step must exist");

        let pose_idx = step.poses[num_detected_poses % step.poses.len()];
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
                if pose_timestamp > expected_next_pose + self.beat_tolerance {
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

    /// Return how many poses have been detected so far.
    pub(crate) fn num_detected_poses(&self) -> usize {
        let full = if let Some(target_step) = &self.target_step {
            self.detected.steps.len() * target_step.beats()
        } else {
            self.detected
                .steps
                .iter()
                .map(|step| step.poses.len())
                .sum()
        };
        let partial = if let Some(partial_step) = &self.detected.partial {
            partial_step.poses.len()
        } else {
            0
        };
        full + partial
    }

    /// Returns the single tracked step.
    ///
    /// (experimenting with live instructor, I probably want to change this when cleaning up the impl)
    pub(crate) fn tracked_step(&self) -> StepInfo {
        self.target_step.clone().expect("requires target_step")
    }

    pub(crate) fn add_pose(&mut self, pose: PoseApproximation) {
        self.detected.add_pose(pose);
        self.on_beat_candidates.clear();
        if let Some(target_step) = &self.target_step {
            self.detected.match_step(target_step);
        }
    }

    pub(crate) fn transition_to_state(&mut self, state: DetectionState, t: Timestamp) {
        self.detection_state = state;
        self.detection_state_start = t;
        self.detection_state_store.set(state);
    }

    pub(crate) fn time_between_poses(&self) -> f64 {
        if self.half_speed {
            2.0 * 30_000.0 / self.bpm as f64
        } else {
            30_000.0 / self.bpm as f64
        }
    }

    #[inline]
    pub(crate) fn poses_per_beat(&self) -> u32 {
        if self.half_speed {
            1
        } else {
            2
        }
    }

    pub(crate) fn timestamp_to_beat(&self, t: Timestamp) -> u32 {
        let t0 = self.next_pose_time(self.beat_alignment.unwrap_or(0.0));
        let pose_duration = self.time_between_poses();
        ((t - t0) / pose_duration).floor() as u32
    }

    pub(crate) fn next_pose_time(&self, not_before: Timestamp) -> Timestamp {
        let t0 = self.beat_alignment.unwrap_or(0.0);
        let pose_duration = self.time_between_poses();
        let poses = ((not_before - t0) / pose_duration).ceil();
        t0 + poses * pose_duration
    }

    pub(crate) fn emit_countdown_audio(&mut self, not_before: Timestamp) {
        let beat = self.time_between_poses();
        let next_beat = self.next_pose_time(not_before);

        self.ui_events.add_audio(next_beat, "and".to_owned());
        self.ui_events.add_audio(next_beat + beat, "one".to_owned());
        self.ui_events
            .add_audio(next_beat + 3.0 * beat, "two".to_owned());
        self.ui_events
            .add_audio(next_beat + 5.0 * beat, "one".to_owned());
        self.ui_events
            .add_audio(next_beat + 6.0 * beat, "two".to_owned());
        self.ui_events
            .add_audio(next_beat + 7.0 * beat, "three".to_owned());
        self.ui_events
            .add_audio(next_beat + 8.0 * beat, "four".to_owned());
    }
}
