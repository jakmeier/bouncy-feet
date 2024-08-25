use wasm_bindgen::prelude::wasm_bindgen;

use crate::tracker::PoseApproximation;
use crate::{DetectionFailureReason, DetectionResult, PoseHint, StepInfo};

use super::dance_collection::DanceCollection;
use super::pose::PoseDirection;
use super::skeleton_3d::Skeleton3d;

type Timestamp = u32;

/// Contains all information about a dance to be detected and has an interface
/// to be used by a Tracker to match tracked skeletons to it.
#[wasm_bindgen]
pub(crate) struct DanceDetector {
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

impl Default for DanceDetector {
    fn default() -> Self {
        Self {
            bpm: 120.0,
            error_threshold: 0.05,
            intermediate_result: None,
            last_error: None,
            beat_alignment: None,
            force_beat: false,
        }
    }
}

impl DanceDetector {
    pub(crate) fn new(intermediate_result: Option<DetectionResult>) -> Self {
        Self {
            intermediate_result,
            ..Default::default()
        }
    }

    pub(crate) fn clear(&mut self) {
        if let Some(intermediate) = self.intermediate_result.as_mut() {
            intermediate.partial = None;
            intermediate.steps.clear();
        }
        self.last_error = None;
    }

    /// Take a previous detection and try adding one more pose to it.
    pub fn detect_next_pose(
        &mut self,
        db: &DanceCollection,
        skeleton: &Skeleton3d,
        now: Timestamp,
    ) -> DetectionResult {
        let prev_detection = self
            .intermediate_result
            .as_ref()
            .expect("requires intermediate_result");

        let end_t = now;
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

        let beat = self.expected_pose_beat();
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
            pose.error(&original_angles, skeleton.positions())
        } else {
            pose.error(skeleton.angles(), skeleton.positions())
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
            self.last_error = None;
        } else {
            let hint = {
                if has_z_error {
                    PoseHint::ZOrder
                } else {
                    let left_right_pose = db.pose_left_right_switched(pose_idx);
                    let lr_error = left_right_pose.error(skeleton.angles(), skeleton.positions());
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

    /// Return a pose that's expected next.
    ///
    /// Only implemented to work properly for trackers of unique steps.
    ///
    /// returns a beat number
    /// (experimenting with live instructor, I probably want to change this when cleaning up the impl)
    pub(crate) fn expected_pose_beat(&self) -> usize {
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
    pub(crate) fn tracked_step(&self) -> StepInfo {
        let detection = self
            .intermediate_result
            .as_ref()
            .expect("requires intermediate_result");
        detection.target_step.clone().expect("requires target_step")
    }

    pub(crate) fn add_pose(&mut self, pose: PoseApproximation) {
        let detection = self
            .intermediate_result
            .as_mut()
            .expect("requires intermediate_result");
        detection.add_pose(pose);
        detection.update_partial();
    }
}
