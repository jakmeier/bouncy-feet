use crate::intern::pose_score::{best_fit_pose, ErrorDetails};
use crate::public::Tracker;
use crate::tracker::PoseApproximation;

type Timestamp = u32;

impl Tracker {
    pub(crate) fn best_fit_pose_impl(
        &self,
        first: usize,
        last: usize,
    ) -> Option<PoseApproximation> {
        let result = crate::STATE.with_borrow(|state| {
            if state.db.is_empty() {
                return None;
            }
            let mut error = f32::INFINITY;
            let mut error_details = ErrorDetails::default();
            let mut pose_index = 0;
            let mut history_index = 0;

            for i in first..last {
                let (err, details, pose) = best_fit_pose(&self.skeletons[i], state.db.poses());
                if err < error {
                    error = err;
                    error_details = details;
                    pose_index = pose;
                    history_index = i;
                }
            }
            Some(PoseApproximation {
                name: state.db.pose_name(pose_index).to_owned(),
                error,
                timestamp: self.timestamps[history_index],
                error_details,
            })
        })?;
        Some(result)
    }

    /// Find the best matching approximation of the given pose in the given range.
    pub(crate) fn find_pose(
        &self,
        pose_index: usize,
        start: Timestamp,
        end: Timestamp,
    ) -> Option<PoseApproximation> {
        let first = self.timestamps.partition_point(|t| *t < start);
        let last = self.timestamps.partition_point(|t| *t <= end);
        if first == last {
            return None;
        }

        crate::STATE.with_borrow(|state| {
            if state.db.is_empty() {
                return None;
            }
            let mut best_error = f32::INFINITY;
            let mut best_details = ErrorDetails::default();
            let mut history_index = 0;

            for i in first..last {
                let pose = &state.db.poses()[pose_index];
                let skeleton = &self.skeletons[i];
                if pose.direction != skeleton.direction().into() {
                    continue;
                }

                let details = pose.error(&skeleton.angles());
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
                    name: state.db.pose_name(pose_index).to_owned(),
                    error: best_error,
                    timestamp: self.timestamps[history_index],
                    error_details: best_details,
                })
            }
        })
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

        crate::STATE.with_borrow(|state| {
            let angles = skeleton.angles();
            state
                .db
                .poses()
                .iter()
                .enumerate()
                .map(|(pose_index, pose)| {
                    let details = pose.error(&angles);
                    PoseApproximation {
                        name: state.db.pose_name(pose_index).to_owned(),
                        error: details.error_score(),
                        timestamp,
                        error_details: details,
                    }
                })
                .collect()
        })
    }
}
