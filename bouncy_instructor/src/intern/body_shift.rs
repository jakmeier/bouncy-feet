use crate::skeleton::{Cartesian2d, Skeleton};

use super::dance_collection::DanceCollection;
use super::step::Step;

/// Component for calculating how much the body shifts after N transitions.
#[derive(Debug, Clone)]
pub(crate) struct BodyShift {
    /// How far the whole body moves as defined in the pose.
    pose_body_shift: Vec<Cartesian2d>,
    /// How far the whole body moves after applying the pose transitions.
    accumulated_body_shift: Vec<Cartesian2d>,
}

impl BodyShift {
    pub(crate) fn new() -> Self {
        Self {
            pose_body_shift: vec![],
            accumulated_body_shift: vec![Cartesian2d::default()],
        }
    }

    pub(crate) fn add_step(&mut self, step: &Step, skeletons: &[Skeleton], db: &DanceCollection) {
        for &pose in &step.poses {
            self.pose_body_shift.push(db.poses()[pose].shift);
        }

        // Compute how far the body shifts after 0,1,2,3... transitions.
        for (pivot, skeleton) in step.pivots[1..].iter().zip(skeletons.windows(2)) {
            let before = skeleton[0].position(*pivot);
            let after = skeleton[1].position(*pivot);
            let diff = after - before;
            self.accumulated_body_shift
                .push(*self.accumulated_body_shift.last().unwrap() - diff);
        }
        if !skeletons.is_empty() && !step.pivots.is_empty() {
            let pivot = step.pivots[0];
            let before = skeletons.last().unwrap().position(pivot);
            let after = skeletons[0].position(pivot);
            let diff = after - before;
            self.accumulated_body_shift
                .push(*self.accumulated_body_shift.last().unwrap() - diff);
        }

        debug_assert_eq!(
            self.pose_body_shift.len() + 1,
            self.accumulated_body_shift.len()
        );
    }

    /// How much the body position deviates from the origin after N beats.
    pub(crate) fn at_beat(&self, beat: usize) -> Cartesian2d {
        debug_assert!(!self.accumulated_body_shift.is_empty());
        let len = self.pose_body_shift.len();
        if len == 0 {
            return Cartesian2d::default();
        }
        let n_full_turns = beat / len;
        let shift_full_turn = self
            .accumulated_body_shift
            .last()
            .copied()
            .unwrap_or_default();
        let pose_shift = self.pose_body_shift[beat % len];
        pose_shift + shift_full_turn * n_full_turns + self.accumulated_body_shift[beat % len]
    }
}

impl Default for BodyShift {
    fn default() -> Self {
        Self::new()
    }
}
