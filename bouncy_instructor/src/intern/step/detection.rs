use crate::public::tracker::PoseApproximation;
use crate::tracker::DetectedStep;
use crate::{Tracker, STATE};

impl DetectedStep {
    pub(crate) fn new(step_name: String, poses: Vec<PoseApproximation>) -> Self {
        Self {
            step_name,
            start: poses.first().map(|p| p.timestamp).unwrap_or(0),
            end: poses.last().map(|p| p.timestamp).unwrap_or(0),
            error: poses.iter().map(|p| p.error).sum::<f32>() / poses.len() as f32,
            poses,
        }
    }
}

impl Tracker {
    /// find the first step that can be matched in the given range
    pub(crate) fn find_first_step(&self, mut start: usize, end: usize) -> Option<DetectedStep> {
        if start + 1 > self.timestamps.len() {
            return None;
        }
        let end = end.min(self.timestamps.len());
        let dt = 60_000.0 / self.bpm;
        let min_dt = (dt * 0.5).round() as u32;
        let max_dt = (dt * 1.5).round() as u32;

        let threshold = 0.2;
        let pose_window_ms = max_dt;
        // let step_window_ms = dt;

        // hack: only return idle if nothing else was found
        let mut idle_result = None;

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
                    if !step.step_name.contains("Idle") {
                        return Some(step);
                    } else if idle_result.is_none() {
                        idle_result = Some(step)
                    }
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
    fn detect_step(&self, start: usize, min_dt: u32, max_dt: u32) -> Option<DetectedStep> {
        let mut best_error = f32::INFINITY;
        let mut result = None;

        STATE.with_borrow(|state| {
            for step in &state.steps {
                let mut pose_matches = vec![];
                let mut start_t = self.timestamps[start];
                let mut end_t = start_t + max_dt;
                for pose in &step.poses {
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
                        // hack: only overwrite with non-idle step with idle step if we are sure
                        if !step.name.contains("Idle")
                            || result
                                .as_ref()
                                .is_some_and(|r: &DetectedStep| r.step_name.contains("Idle"))
                            || (best_error > 0.2 && detection.error < 0.075)
                        {
                            best_error = detection.error;
                            result = Some(detection);
                        }
                    }
                }
            }
        });
        result
    }
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
        let times = [0, 100, 500, 1000];
        let expected_steps = ["Test-Step-2"];

        check_detect_front_dance(&degrees, times, &expected_steps);
    }

    #[test]
    fn test_detect_dance_side_no_match() {
        let degrees = [0, 0, 45, 90];
        let times = [0, 100, 500, 1000];
        let expected_steps = [];

        check_detect_side_dance(&degrees, times, &expected_steps);
    }

    #[test]
    fn test_detect_dance_side() {
        let degrees = [0, 0, 90, 45, 45, 90];
        let times = [0, 100, 1000, 2000, 3000, 4000];
        let expected_steps = ["Test-Step-4"];

        check_detect_side_dance(&degrees, times, &expected_steps);
    }

    /// Test with two frames to skip at the start
    #[test]
    fn test_detect_dance_skip_start() {
        let degrees = [-45, -45, 0, 90];
        let times = [0, 100, 500, 1500];
        let expected_steps = ["Test-Step-2"];

        check_detect_front_dance(&degrees, times, &expected_steps);
    }

    #[test]
    fn test_detect_dance_multi_step() {
        let degrees = [0, -90, 0, -90, -90, 90, 0, -90, 0, -90, 0, 90];
        let times = (0..degrees.len()).map(|n| (n * 1000) as u32);
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
        let times = (0..degrees.len()).map(|n| (n * 100) as u32);
        let expected_steps = ["Test-Step-2"];

        check_detect_front_dance(&degrees, times, &expected_steps);
    }

    #[track_caller]
    fn check_detect_side_dance(
        degrees: &[i16],
        times: impl IntoIterator<Item = u32>,
        expected_steps: &[&str],
    ) {
        let kp = facing_right_keypoints();
        check_detect_dance(kp, degrees, times.into_iter(), expected_steps)
    }
    #[track_caller]
    fn check_detect_front_dance(
        degrees: &[i16],
        times: impl IntoIterator<Item = u32>,
        expected_steps: &[&str],
    ) {
        let kp = facing_camera_keypoints();
        check_detect_dance(kp, degrees, times.into_iter(), expected_steps)
    }

    #[track_caller]
    fn check_detect_dance(
        mut kp: Keypoints,
        degrees: &[i16],
        times: impl Iterator<Item = u32>,
        expected_steps: &[&str],
    ) {
        setup();

        let mut tracker = Tracker::new();
        tracker.bpm = 60.0;

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
        let step_names = dance.into_iter().map(|d| d.step_name).collect::<Vec<_>>();
        let expected_steps = expected_steps
            .into_iter()
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
        )
        .unwrap();
    }
}
