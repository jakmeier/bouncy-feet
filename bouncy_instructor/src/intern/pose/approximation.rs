use crate::intern::pose_score::{best_fit_pose, ErrorDetails};
use crate::intern::tracker_dance_collection::TrackerDanceCollection;
use crate::public::Tracker;
use crate::tracker::PoseApproximation;

type Timestamp = f64;

impl Tracker {
    /// Find the least-error pose in a range of recorded frames.
    pub(crate) fn best_fit_pose_impl(
        &self,
        first: usize,
        last: usize,
    ) -> Option<PoseApproximation> {
        if self.db.is_empty() {
            return None;
        }
        let mut error = f32::INFINITY;
        let mut error_details = ErrorDetails::default();
        let mut pose_index = 0;
        let mut history_index = 0;

        for i in first..last {
            let (err, details, pose) = best_fit_pose(&self.skeletons[i], self.db.poses());
            if err < error {
                error = err;
                error_details = details;
                pose_index = pose;
                history_index = i;
            }
        }
        Some(PoseApproximation {
            id: self.db.pose_id(pose_index).to_owned(),
            name: self.db.pose_name(pose_index).to_owned(),
            error,
            timestamp: self.timestamps[history_index],
            error_details,
        })
    }

    /// Find the best matching approximation of the given pose in the given range.
    ///
    /// Combines spatial and timely errors. Spatial error are the acceptance
    /// criterion and the reported error, but similarly good frames are biased
    /// towards `expected_timestamp`.
    pub(crate) fn find_pose(
        &self,
        pose_index: usize,
        start: Timestamp,
        end: Timestamp,
        expected_timestamp: Timestamp,
        db: &TrackerDanceCollection,
    ) -> Option<PoseApproximation> {
        let first = self.timestamps.partition_point(|t| *t < start);
        let last = self.timestamps.partition_point(|t| *t <= end);
        if first == last {
            return None;
        }

        if db.is_empty() {
            return None;
        }
        let mut best_error = f32::INFINITY;
        let mut best_score = f32::INFINITY;
        let mut best_details = ErrorDetails::default();
        let mut history_index = 0;
        let timing_scale = (expected_timestamp - start)
            .abs()
            .max((end - expected_timestamp).abs())
            .max(f64::EPSILON);

        for i in first..last {
            let pose = &db.poses()[pose_index];
            let skeleton = &self.skeletons[i];
            if pose.direction != skeleton.direction().into() {
                continue;
            }

            let details = pose.skeleton_error(skeleton);
            let error = details.error_score();
            let timing_error =
                ((self.timestamps[i] - expected_timestamp).abs() / timing_scale) as f32;
            let score = timed_pose_score(error, timing_error);
            if score < best_score {
                best_error = error;
                best_score = score;
                best_details = details;
                history_index = i;
            }
        }
        if best_error >= 1.0 {
            // pose not even close to be found
            None
        } else {
            Some(PoseApproximation {
                id: db.pose_id(pose_index).to_owned(),
                name: db.pose_name(pose_index).to_owned(),
                error: best_error,
                timestamp: self.timestamps[history_index],
                error_details: best_details,
            })
        }
    }

    /// Fit a single frame against all poses and return all errors
    pub(crate) fn all_pose_approximations(&self, timestamp: Timestamp) -> Vec<PoseApproximation> {
        if self.skeletons.is_empty() {
            return vec![];
        }

        let skeleton = match self
            .timestamps
            .binary_search_by(|probe| f64::total_cmp(probe, &timestamp))
        {
            Ok(i) | Err(i) => &self.skeletons[i],
        };

        self.db
            .poses()
            .iter()
            .enumerate()
            .map(|(pose_index, pose)| {
                let details = pose.skeleton_error(skeleton);
                PoseApproximation {
                    id: self.db.pose_id(pose_index).to_owned(),
                    name: self.db.pose_name(pose_index).to_owned(),
                    error: details.error_score(),
                    timestamp,
                    error_details: details,
                }
            })
            .collect()
    }
}

fn timed_pose_score(pose_error: f32, timing_error: f32) -> f32 {
    const TIMING_WEIGHT: f32 = 0.25;
    pose_error + TIMING_WEIGHT * timing_error
}

#[cfg(test)]
mod timing_tests {
    use super::timed_pose_score;

    #[test]
    fn slightly_worse_pose_on_time_beats_closest_pose_off_time() {
        assert!(timed_pose_score(0.10, 0.0) < timed_pose_score(0.05, 1.0));
    }

    #[test]
    fn timing_does_not_hide_a_bad_pose() {
        assert!(timed_pose_score(0.50, 0.0) > timed_pose_score(0.05, 1.0));
    }
}
