use svelte_store::Readable;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::tracker::PoseApproximation;
use crate::ui_event::UiEvents;
use crate::{DetectionFailureReason, DetectionResult, PoseHint, StepInfo};

use super::dance_collection::DanceCollection;
use super::pose::PoseDirection;
use super::skeleton_3d::Skeleton3d;

type Timestamp = u64;

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
    /// The expected step for unique step tracking
    pub(crate) target_step: Option<StepInfo>,
    /// How many beats to track for, counting only in LiveTracking state.
    pub(crate) tracked_beats: Option<u32>,

    // state
    /// Data about detection so far.
    pub(crate) detected: DetectionResult,
    /// State machine of the detector.
    detection_state: DetectionState,
    /// A svelte store that can be subscribed to for state updates.
    pub(crate) detection_state_store: Readable<DetectionState>,
    /// When the tracker entered the current state.
    pub(crate) detection_state_start: Timestamp,
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
            detection_state: DetectionState::Init,
            detection_state_store: Readable::new(DetectionState::Init),
            detection_state_start: 0,
            ui_events: UiEvents::default(),
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
    }

    /// Make progress, depending on detection state and new data added since last tick.
    pub(crate) fn tick(
        &mut self,
        now: Timestamp,
        db: &DanceCollection,
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
                        if error_details.error_score() < 0.001 {
                            self.transition_to_state(DetectionState::CountDown, now);
                            self.emit_countdown_audio(now);
                        }
                    }
                }
            }
            DetectionState::CountDown => {
                if now
                    > self.detection_state_start + (self.time_between_poses() * 16.0).floor() as u64
                {
                    self.transition_to_state(DetectionState::LiveTracking, now);
                }
            }
            DetectionState::LiveTracking => {
                if let Some(num_beats) = self.tracked_beats {
                    let end = self.detection_state_start
                        + (1 + num_beats) as u64 * (self.time_between_poses() * 2.0).round() as u64;
                    if now >= end {
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
        db: &DanceCollection,
        skeleton: &Skeleton3d,
        now: Timestamp,
    ) -> DetectionResult {
        let prev_detection = &mut self.detected;
        let end_t = now;
        let last_step = prev_detection
            .partial
            .as_ref()
            .or_else(|| prev_detection.steps.last());
        let start_t = last_step.map_or(0, |step| step.end);

        // skip at least a quarter beat
        let min_delay = (self.time_between_poses() / 2.0).round() as u64;
        if end_t < start_t + min_delay {
            return self
                .detected
                .clone()
                .with_failure_reason(DetectionFailureReason::TooEarly);
        }

        // check we are on beat, if aligned to beat
        if let Some(first_beat) = self.beat_alignment {
            let half_beat = self.time_between_poses();
            let t_total = end_t - first_beat;
            let relative_beat_distance = (t_total as f32 / half_beat).fract();
            if relative_beat_distance > 0.2 {
                return self
                    .detected
                    .clone()
                    .with_failure_reason(DetectionFailureReason::NotOnBeat);
            }
        }

        let beat = self.num_detected_poses();
        let step_info = self.tracked_step();
        let step = db.step(&step_info.id()).expect("tracked step must exist");

        let pose_idx = step.poses[beat % step.poses.len()];
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
            name: db.pose_name(pose_idx).to_owned(),
            error,
            timestamp: end_t,
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
            // Despite the error, if forced, the pose should still be added to the detection.
            if self.force_beat {
                self.add_pose(pose_approximation.clone());
                self.detected.pose_misses += 1;
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
        let full = self.detected.steps.len()
            * self
                .target_step
                .as_ref()
                .expect("must have target step")
                .beats();

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
        if let Some(target_step) = &self.target_step {
            self.detected.match_step(target_step);
        }
    }

    pub(crate) fn transition_to_state(&mut self, state: DetectionState, t: Timestamp) {
        self.detection_state = state;
        self.detection_state_start = t;
        self.detection_state_store.set(state);
    }

    pub(crate) fn time_between_poses(&self) -> f32 {
        if self.half_speed {
            2.0 * 30_000.0 / self.bpm
        } else {
            30_000.0 / self.bpm
        }
    }

    pub(crate) fn next_pose_time(&self, not_before: Timestamp) -> Timestamp {
        let t0 = self.beat_alignment.unwrap_or(0);
        let half_beat_duration = self.time_between_poses().round() as u64;
        let half_beats = (not_before - t0 + half_beat_duration - 1) / half_beat_duration;
        t0 + half_beats * half_beat_duration
    }

    pub(crate) fn emit_countdown_audio(&mut self, not_before: Timestamp) {
        let beat = (2.0 * self.time_between_poses()).round() as u64;
        let next_beat = self.next_pose_time(not_before);

        self.ui_events.add_audio(next_beat, "and".to_owned());
        self.ui_events.add_audio(next_beat + beat, "one".to_owned());
        self.ui_events
            .add_audio(next_beat + 3 * beat, "two".to_owned());
        self.ui_events
            .add_audio(next_beat + 5 * beat, "one".to_owned());
        self.ui_events
            .add_audio(next_beat + 6 * beat, "two".to_owned());
        self.ui_events
            .add_audio(next_beat + 7 * beat, "three".to_owned());
        self.ui_events
            .add_audio(next_beat + 8 * beat, "four".to_owned());
    }
}
