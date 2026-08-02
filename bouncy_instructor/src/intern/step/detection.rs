use crate::intern::tracker_dance_collection::TrackerDanceCollection;
use crate::public::tracker::{DetectionResult, PoseApproximation};
use crate::tracker::DetectedStep;
use crate::{StepInfo, Tracker};

impl DetectedStep {
    pub(crate) fn new(step_id: String, step_name: String, poses: Vec<PoseApproximation>) -> Self {
        Self {
            step_id,
            step_name,
            start: poses.first().map(|p| p.timestamp).unwrap_or(0.0),
            end: poses.last().map(|p| p.timestamp).unwrap_or(0.0),
            error: poses.iter().map(|p| p.error).sum::<f32>() / poses.len() as f32,
            poses,
        }
    }

    pub(crate) fn update_stats(&mut self) {
        self.start = self.poses.first().map(|p| p.timestamp).unwrap_or(0.0);
        self.end = self.poses.last().map(|p| p.timestamp).unwrap_or(0.0);
        self.error = self.poses.iter().map(|p| p.error).sum::<f32>() / self.poses.len() as f32;
    }
}

impl DetectionResult {
    /// Note: make sure to restore cursor after calling this
    pub(crate) fn add_pose(&mut self, pose: PoseApproximation) {
        if let Some(partial) = &mut self.partial {
            partial.poses.push(pose);
            partial.update_stats();
        } else {
            let mut new_partial =
                DetectedStep::new(String::new(), "unknown".to_owned(), vec![pose]);
            new_partial.update_stats();
            self.partial = Some(new_partial);
        }
    }

    /// Note: make sure to restore cursor after calling this
    pub(crate) fn match_step(&mut self, target_step: &StepInfo) {
        if let (target, Some(partial)) = (target_step, &mut self.partial) {
            if target.skeletons.len() == partial.poses.len() {
                let mut full_step = self.partial.take().unwrap();
                full_step.step_id = target_step.id();
                full_step.step_name = target_step.name();
                self.steps.push(full_step);
            }
        }
    }
}

impl Tracker {
    /// Find the first complete step that can be matched in the given range.
    pub(crate) fn find_first_step(&self, mut start: usize, end: usize) -> Option<DetectedStep> {
        let end = end.min(self.timestamps.len());
        if start >= end {
            return None;
        }
        let dt = 60_000.0 / self.detector.bpm;
        let min_dt = (dt * 0.5).round() as f64;
        let max_dt = (dt * 1.5).round() as f64;
        // A tracker containing only variations of one dance already knows the
        // intended dance. When choosing among different dance names, every
        // keyframe must be a strong match.
        let mut non_idle_steps = self
            .db
            .steps()
            .iter()
            .filter(|step| !step.name.contains("Idle"));
        let first_step_name = non_idle_steps.next().map(|step| &step.name);
        let require_all_pose_matches = first_step_name
            .is_some_and(|first_name| non_idle_steps.any(|step| step.name != *first_name));

        // Only return idle if no non-idle step is found.
        let mut idle_result = None;
        while start < end {
            if !require_all_pose_matches {
                if let Some(step) = self.detect_step(start, min_dt, max_dt, &self.db) {
                    if step.poses.first().is_some_and(|pose| pose.error <= 0.2) {
                        if !step.step_name.contains("Idle") {
                            return Some(step);
                        }
                        if idle_result.is_none() {
                            idle_result = Some(step);
                        }
                    }
                }
                start += 1;
                continue;
            }

            let start_t = self.timestamps[start];
            let end_t = (start_t + max_dt).min(self.timestamps[end - 1]);

            // Seed a sequence from the strongest pose in a bounded look-ahead
            // window.
            if let Some(first_pose) = self.best_fit_pose(start_t, end_t) {
                if first_pose.error <= 0.2 {
                    let candidate_start = start
                        + self.timestamps[start..end]
                            .partition_point(|timestamp| *timestamp < first_pose.timestamp);
                    if let Some(step) = self.detect_step(candidate_start, min_dt, max_dt, &self.db)
                    {
                        if step.poses.iter().all(|pose| pose.error <= 0.2) {
                            if !step.step_name.contains("Idle") {
                                return Some(step);
                            }
                            if idle_result.is_none() {
                                idle_result = Some(step);
                            }
                        }
                    }
                    start = candidate_start + 1;
                    continue;
                }
            }

            // No strong pose in this window, skip it as a whole.
            start += self.timestamps[start..end]
                .partition_point(|timestamp| *timestamp < end_t)
                .max(1);
        }

        idle_result
    }

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
    fn detect_step(
        &self,
        start: usize,
        min_dt: f64,
        max_dt: f64,
        db: &TrackerDanceCollection,
    ) -> Option<DetectedStep> {
        let expected_dt = (min_dt + max_dt) / 2.0;
        let mut best_score = f32::INFINITY;
        let mut best_pose_error = f32::INFINITY;
        let mut result = None;

        for step in db.steps() {
            if let Some(detection) = self.detect_specific_step(start, min_dt, max_dt, step, db) {
                let score = step_match_score(&detection, expected_dt);
                if score < best_score {
                    // hack: only overwrite with non-idle step with idle step if we are sure
                    if !step.name.contains("Idle")
                        || result
                            .as_ref()
                            .is_some_and(|r: &DetectedStep| r.step_name.contains("Idle"))
                        || (best_pose_error > 0.2 && detection.error < 0.075)
                    {
                        best_score = score;
                        best_pose_error = detection.error;
                        result = Some(detection);
                    }
                }
            }
        }
        result
    }

    fn detect_specific_step(
        &self,
        start: usize,
        min_dt: f64,
        max_dt: f64,
        step: &crate::intern::step::Step,
        db: &TrackerDanceCollection,
    ) -> Option<DetectedStep> {
        let expected_dt = (min_dt + max_dt) / 2.0;
        let mut pose_matches = Vec::with_capacity(step.poses.len());
        let mut start_t = self.timestamps[start];
        let mut end_t = start_t + max_dt;
        let mut expected_t = start_t;

        for pose in &step.poses {
            let pose_match = self.find_pose(*pose, start_t, end_t, expected_t, db)?;
            expected_t = pose_match.timestamp + expected_dt;
            start_t = pose_match.timestamp + min_dt;
            end_t = pose_match.timestamp + max_dt;
            pose_matches.push(pose_match);
        }

        Some(DetectedStep::new(
            step.id.clone(),
            step.name.clone(),
            pose_matches,
        ))
    }

    /// Detect the finite sequence configured by a teacher. Returning `None`
    /// means this tracker is freestyle and should use unconstrained detection.
    pub(crate) fn detect_configured_steps(
        &self,
        mut start: usize,
        end: usize,
    ) -> Option<Vec<DetectedStep>> {
        let sequence = self.detector.teacher.tracked_step_sequence()?;
        let mut detections = Vec::with_capacity(sequence.len());
        let mut next_scheduled_start = None;

        for (target, subbeats_per_pose) in sequence {
            let expected_dt = self.detector.subbeat_time() * subbeats_per_pose as f64;
            let detection = if let Some(scheduled_start) = next_scheduled_start {
                self.find_specific_step_near(start, end, &target, expected_dt, scheduled_start)
            } else {
                self.find_first_specific_step(start, end, &target, expected_dt)
            };
            let Some(detection) = detection else {
                break;
            };

            // Anchor the sequence to its first observed movement. Anchoring to
            // each previous match would accumulate frame-selection drift.
            next_scheduled_start = Some(
                next_scheduled_start.unwrap_or(detection.start)
                    + expected_dt * target.num_poses() as f64,
            );
            start += self.timestamps[start..end]
                .partition_point(|timestamp| *timestamp <= detection.end);
            detections.push(detection);
        }
        Some(detections)
    }

    fn find_specific_step_near(
        &self,
        start: usize,
        end: usize,
        target: &StepInfo,
        expected_dt: f64,
        scheduled_start: f64,
    ) -> Option<DetectedStep> {
        const START_TIMING_WEIGHT: f32 = 0.5;
        let earliest = scheduled_start - expected_dt * 0.5;
        let latest = scheduled_start + expected_dt * 0.5;
        let first = start + self.timestamps[start..end].partition_point(|t| *t < earliest);
        let last = start + self.timestamps[start..end].partition_point(|t| *t <= latest);
        let step = self.db.step(&target.id())?;
        let min_dt = (expected_dt * 0.5).round();
        let max_dt = (expected_dt * 1.5).round();
        let mut best = None;
        let mut best_score = f32::INFINITY;

        for candidate_start in first..last {
            let Some(detection) =
                self.detect_specific_step(candidate_start, min_dt, max_dt, step, &self.db)
            else {
                continue;
            };
            if detection.poses.first().is_none_or(|pose| pose.error >= 1.0)
                || detection.start > latest
            {
                continue;
            }
            let start_error = ((detection.start - scheduled_start).abs() / expected_dt) as f32;
            let score =
                step_match_score(&detection, expected_dt) + START_TIMING_WEIGHT * start_error;
            if score < best_score {
                best_score = score;
                best = Some(detection);
            }
        }
        best
    }

    fn find_first_specific_step(
        &self,
        mut start: usize,
        end: usize,
        target: &StepInfo,
        expected_dt: f64,
    ) -> Option<DetectedStep> {
        let step = self.db.step(&target.id())?;
        let min_dt = (expected_dt * 0.5).round();
        let max_dt = (expected_dt * 1.5).round();

        while start < end {
            if let Some(detection) =
                self.detect_specific_step(start, min_dt, max_dt, step, &self.db)
            {
                if detection.poses.first().is_some_and(|pose| pose.error < 1.0) {
                    return Some(detection);
                }
            }
            start += 1;
        }
        None
    }
}

/// Keep the public pose error spatial-only, while using rhythm to rank otherwise
/// plausible complete steps.
fn step_match_score(step: &DetectedStep, expected_dt: f64) -> f32 {
    const TIMING_WEIGHT: f32 = 0.25;
    let timing_error = if step.poses.len() < 2 {
        0.0
    } else {
        step.poses
            .windows(2)
            .map(|poses| {
                ((poses[1].timestamp - poses[0].timestamp - expected_dt).abs() / expected_dt) as f32
            })
            .sum::<f32>()
            / (step.poses.len() - 1) as f32
    };
    step.error + TIMING_WEIGHT * timing_error
}

// Tests below check basic step detection has no major bugs. They are not
// designed for checking correctness, we rely on integration tests for that.
// Rather, they check that there are no infinite loop or panics when detecting
// simple cases.
#[cfg(test)]
mod tests {
    use crate::keypoints::Cartesian3d;
    use crate::{load_pose_str, load_step_str, Keypoints};

    use super::*;

    #[test]
    fn test_detect_dance_front() {
        let degrees = [0, 0, 45, 90];
        let times = [0., 100., 500., 1000.];
        let expected_steps = ["Test-Step-2"];

        check_detect_front_dance(&degrees, times, &expected_steps);
    }

    #[test]
    fn test_detect_dance_side_no_match() {
        let degrees = [0, 0, 45, 90];
        let times = [0., 100., 500., 1000.];
        let expected_steps = [];

        check_detect_side_dance(&degrees, times, &expected_steps);
    }

    #[test]
    fn test_detect_dance_side() {
        let degrees = [0, 0, 90, 45, 45, 90];
        let times = [0., 100., 1000., 2000., 3000., 4000.];
        let expected_steps = ["Test-Step-4"];

        check_detect_side_dance(&degrees, times, &expected_steps);
    }

    /// Test with two frames to skip at the start
    #[test]
    fn test_detect_dance_skip_start() {
        let degrees = [-45, -45, 0, 90];
        let times = [0., 100., 500., 1500.];
        let expected_steps = ["Test-Step-2"];

        check_detect_front_dance(&degrees, times, &expected_steps);
    }

    #[test]
    fn test_detect_dance_multi_step() {
        let degrees = [0, -90, 0, -90, -90, 90, 0, -90, 0, -90, 0, 90];
        let times = (0..degrees.len()).map(|n| (n * 1000) as f64);
        let expected_steps = [
            "Test-Step-1", // 0,-90
            "Test-Step-1", // 0,-90
            "Test-Step-3", // -90,90
            "Test-Step-1", // 0,-90
            "Test-Step-1", // 0,-90
            "Test-Step-2", // 0,90
        ];

        check_detect_front_dance(&degrees, times, &expected_steps);
    }

    /// Test with a couple of frames to skip at the start and many frames between key matches.
    #[test]
    fn test_detect_dance_many_frames() {
        let degrees = [
            -90, -45, -45, -45, 0, 0, 0, 0, 0, 0, 0, // 0 - 900ms
            0, 45, 45, 90, 90, 90, 90, 90, 90, 90, // 1000 - 1900ms
        ];
        let times = (0..degrees.len()).map(|n| (n * 100) as f64);
        let expected_steps = ["Test-Step-2"];

        check_detect_front_dance(&degrees, times, &expected_steps);
    }

    #[track_caller]
    fn check_detect_side_dance(
        degrees: &[i16],
        times: impl IntoIterator<Item = f64>,
        expected_steps: &[&str],
    ) {
        let kp = facing_right_keypoints();
        check_detect_dance(kp, degrees, times.into_iter(), expected_steps)
    }
    #[track_caller]
    fn check_detect_front_dance(
        degrees: &[i16],
        times: impl IntoIterator<Item = f64>,
        expected_steps: &[&str],
    ) {
        let kp = facing_camera_keypoints();
        check_detect_dance(kp, degrees, times.into_iter(), expected_steps)
    }

    #[track_caller]
    fn check_detect_dance(
        mut kp: Keypoints,
        degrees: &[i16],
        times: impl Iterator<Item = f64>,
        expected_steps: &[&str],
    ) {
        setup();

        let mut tracker = Tracker::new_from_global_collection();
        tracker.detector.bpm = 60.0;

        for (degree, time) in degrees.iter().zip(times) {
            match degree {
                0 => {
                    kp.left.knee = Cartesian3d::new(1.0, 0.0, 0.0);
                    kp.left.ankle = Cartesian3d::new(1.0, 1.0, 0.0);
                }
                -45 => {
                    kp.left.knee = Cartesian3d::new(1.0, 0.0, 0.0);
                    kp.left.ankle = Cartesian3d::new(2.0, 0.0, 0.0);
                }
                45 => {
                    kp.left.knee = Cartesian3d::new(1.0, 0.0, 0.0);
                    kp.left.ankle = Cartesian3d::new(0.0, 1.0, 0.0);
                }
                90 => {
                    kp.left.knee = Cartesian3d::new(1.0, 0.0, 0.0);
                    kp.left.ankle = Cartesian3d::new(0.5, 0.0, 0.0);
                }
                -90 => {
                    kp.left.knee = Cartesian3d::new(1.0, 0.0, 0.0);
                    kp.left.ankle = Cartesian3d::new(1.5, 0.0, 0.0);
                }
                other => panic!("{other}° not implemented in test setup"),
            }
            tracker.add_keypoints(kp, time);
        }

        let dance = tracker.detect_dance();
        println!("{dance:?}");
        let step_names = dance
            .steps()
            .into_iter()
            .map(|d| d.step_name)
            .collect::<Vec<_>>();
        let expected_steps = expected_steps
            .iter()
            .map(|s| s.to_owned())
            .collect::<Vec<_>>();
        assert_eq!(expected_steps, step_names);
    }

    fn facing_camera_keypoints() -> Keypoints {
        let mut kp0 = Keypoints::default();
        kp0.left.shoulder = Cartesian3d::new(1.0, -2.0, 0.0);
        kp0.right.shoulder = Cartesian3d::new(-1.0, -2.0, 0.0);
        kp0.left.hip = Cartesian3d::new(1.0, -1.0, 0.0);
        kp0.right.hip = Cartesian3d::new(-1.0, -1.0, 0.0);
        kp0
    }

    fn facing_right_keypoints() -> Keypoints {
        let mut kp0 = Keypoints::default();
        kp0.left.shoulder = Cartesian3d::new(0.0, -2.0, -1.0);
        kp0.right.shoulder = Cartesian3d::new(0.0, -2.0, 1.0);
        kp0.left.hip = Cartesian3d::new(0.0, -1.0, -1.0);
        kp0.right.hip = Cartesian3d::new(0.0, -1.0, 1.0);
        kp0
    }

    fn setup() {
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
            (
              name: "test-pose-4",
              direction: Right,
              limbs: [
                (limb: LeftShin, angle: 45, tolerance: 5, weight: 1.0),
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
                id: "0",
                keyframes: [
                (pose: "test-pose-0", orientation: ToCamera),
                (pose: "test-pose-2", orientation: ToCamera),
                ]
            ),
            (
              name: "Test-Step-2",
              id: "1",
              keyframes: [
                (pose: "test-pose-0", orientation: ToCamera),
                (pose: "test-pose-1", orientation: ToCamera),
              ]
            ),
            (
              name: "Test-Step-3",
              id: "2",
              keyframes: [
                (pose: "test-pose-2", orientation: ToCamera),
                (pose: "test-pose-1", orientation: ToCamera),
              ]
            ),
            (
              name: "Test-Step-4",
              id: "3",
              keyframes: [
                (pose: "test-pose-3", orientation: Right),
                (pose: "test-pose-4", orientation: Right),
                (pose: "test-pose-4", orientation: Right),
                (pose: "test-pose-3", orientation: Right),
              ]
            ),
          ]
        )"#,
            "test".to_owned(),
        )
        .unwrap();
    }
}
