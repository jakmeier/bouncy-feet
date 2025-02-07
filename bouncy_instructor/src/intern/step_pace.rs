/// At what pace the poses of a step are matched to beats.
///
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct StepPace {
    /// The normal pace is one pose per subbeat, meaning 2 poses on a full
    /// beat. This gives 8 poses on a full 4/4 count.
    ///
    /// Invariant: never zero
    subbeats_per_pose: u32,
}

impl StepPace {
    fn new(subbeats_per_pose: u32) -> Self {
        // (I1) `subbeats_per_pose` i` not zero
        assert_ne!(subbeats_per_pose, 0);
        Self {
            subbeats_per_pose,
        }
    }

    // Two poses per full beat.
    pub(crate) fn normal() -> Self {
        Self::new(1)
    }

    // One pose per full beat.
    pub(crate) fn half_speed() -> Self {
        Self::new(2)
    }

    // One pose per two full beats.
    pub(crate) fn quarter_speed() -> Self {
        Self::new(4)
    }

    /// Get the pose index after N mini-beats on the given pace.
    ///
    /// Full beat: only count 1,2,3,4 (used for bpm calculation)
    /// Subbeat: also count the "and" between
    pub(crate) fn pose_at_subbeat(&self, subbeat: u32) -> u32 {
        // Safe division due to invariant (I1)
        // rounding down
        subbeat / self.subbeats_per_pose
    }
}
