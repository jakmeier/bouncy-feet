use crate::intern::dance_collection::DanceCollection;
use crate::intern::pose_score::{best_fit_pose, ErrorDetails};
use crate::public::Tracker;
use crate::tracker::PoseApproximation;

type Timestamp = u32;

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
            name: self.db.pose_name(pose_index).to_owned(),
            error,
            timestamp: self.timestamps[history_index],
            error_details,
        })
    }

    /// Find the best matching approximation of the given pose in the given range.
    pub(crate) fn find_pose(
        &self,
        pose_index: usize,
        start: Timestamp,
        end: Timestamp,
        db: &DanceCollection,
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
        let mut best_details = ErrorDetails::default();
        let mut history_index = 0;

        for i in first..last {
            let pose = &db.poses()[pose_index];
            let skeleton = &self.skeletons[i];
            if pose.direction != skeleton.direction().into() {
                continue;
            }

            let details = pose.skeleton_error(skeleton);
            let error = details.error_score();
            if error < best_error {
                best_error = error;
                best_details = details;
                history_index = i;
            }
        }
        if best_error >= 1.0 {
            // pose not even close to be found
            None
        } else {
            Some(PoseApproximation {
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

        let skeleton = match self.timestamps.binary_search(&timestamp) {
            Ok(i) | Err(i) => &self.skeletons[i],
        };

        // for debugging, quite useful for now
        for (angle, name) in skeleton
            .angles()
            .iter()
            .zip(crate::intern::pose::Limb::base_limb_names())
        {
            crate::println!("{name}: {angle:?}");
        }

        self.db
            .poses()
            .iter()
            .enumerate()
            .map(|(pose_index, pose)| {
                let details = pose.skeleton_error(skeleton);
                PoseApproximation {
                    name: self.db.pose_name(pose_index).to_owned(),
                    error: details.error_score(),
                    timestamp,
                    error_details: details,
                }
            })
            .collect()
    }
}
