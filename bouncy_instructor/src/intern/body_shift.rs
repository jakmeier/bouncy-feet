use crate::skeleton::{Cartesian2d, Skeleton};

use super::step::Step;
use super::tracker_dance_collection::TrackerDanceCollection;

/// Component for calculating how much the body shifts after N transitions.
#[derive(Debug, Clone)]
pub(crate) struct BodyShift {
    /// How far the whole body moves as defined in the pose.
    pose_body_shift: Vec<Cartesian2d>,
    /// How far the pivot between pairs of poses moved. Needs to be
    /// counterbalanced when computing the accumulated body shift.
    pivot_movement: Vec<Cartesian2d>,
    /// How far the whole body moves after applying the pose transitions.
    accumulated_body_shift: Vec<Cartesian2d>,
}

impl BodyShift {
    pub(crate) fn new() -> Self {
        Self {
            pose_body_shift: vec![],
            pivot_movement: vec![],
            accumulated_body_shift: vec![Cartesian2d::default()],
        }
    }

    pub(crate) fn add_step(
        &mut self,
        step: &Step,
        skeletons: &[Skeleton],
        db: &TrackerDanceCollection,
    ) {
        if step.poses.is_empty() {
            // nothing to add, nothing rto recompute after
            return;
        }
        for &pose in &step.poses {
            self.pose_body_shift.push(db.poses()[pose].shift);
        }

        // Compute pivot movements
        for (pivot, skeleton) in step.pivots[1..].iter().zip(skeletons.windows(2)) {
            let before = skeleton[0].position(*pivot);
            let after = skeleton[1].position(*pivot);
            let diff = after - before;
            // don't accumulate pivot on the y-axis - dancers usually can't levitate
            let x_diff = Cartesian2d::new(diff.x, 0.0);
            self.pivot_movement.push(x_diff);
        }
        // add last to first movement, too
        if !skeletons.is_empty() && !step.pivots.is_empty() {
            let pivot = step.pivots[0];
            let before = skeletons.last().unwrap().position(pivot);
            let after = skeletons[0].position(pivot);
            let diff = after - before;
            // don't accumulate pivot on the y-axis - dancers usually can't levitate
            let x_diff = Cartesian2d::new(diff.x, 0.0);
            self.pivot_movement.push(x_diff);
        }

        self.compute_accumulated();
    }

    fn compute_accumulated(&mut self) {
        self.accumulated_body_shift = vec![Cartesian2d::default()];

        // Compute how far the body shifts after 0,1,2,3... transitions.
        for pivot_diff in &self.pivot_movement {
            self.accumulated_body_shift
                .push(*self.accumulated_body_shift.last().unwrap() - *pivot_diff);
        }

        debug_assert_eq!(
            self.pose_body_shift.len() + 1,
            self.accumulated_body_shift.len()
        );
    }

    /// How much the body position deviates from the origin after N poses.
    pub(crate) fn after_pose(&self, pose: usize) -> Cartesian2d {
        if self.accumulated_body_shift.is_empty() {
            return Cartesian2d::default();
        }
        let len = self.pose_body_shift.len();
        if len == 0 {
            return Cartesian2d::default();
        }
        let n_full_turns = pose / len;
        let shift_full_turn = self
            .accumulated_body_shift
            .last()
            .copied()
            .unwrap_or_default();
        let pose_shift = self.pose_body_shift[pose % len];
        pose_shift + shift_full_turn * n_full_turns + self.accumulated_body_shift[pose % len]
    }

    pub(crate) fn add(&mut self, body_shift: BodyShift) {
        self.pose_body_shift.extend(body_shift.pose_body_shift);
        self.pivot_movement.extend(body_shift.pivot_movement);
        self.compute_accumulated();
    }
}

impl Default for BodyShift {
    fn default() -> Self {
        Self::new()
    }
}
