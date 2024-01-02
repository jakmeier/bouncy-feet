//! Types and code for keeping a list of poses to detect across method calls.
//!
//! Poses are defined with regard to a list of limb definitions, in order to
//! avoid storing the limb definitions repeatedly. The downside is that each
//! data set (one data set per video frame) on its own lacks context. You have
//! to combine it with the list of poses. This module contains the context
//!  structs required for this.

use super::geom::SignedAngle;
use super::pose::{Limb, LimbPosition, Pose, PoseDirection};
use crate::parsing::ParseFileError;
use crate::pose_file;

/// List of registered poses to recognize during tracking.
///
/// Each pose is a description of a body position. This includes the exact
/// desired position, a name, and most importantly, the formulas for computing
/// an error.
///
/// Actual poses are defined in external files and loaded in at runtime. Here
/// data they are stored in the most convenient way, which will see many
/// refactoring over time.
///
/// Errors are always between 0 and 1, where 0 is a perfect match.
/// For now, the error formula is implicitly the same for all limbs.
pub(crate) struct LimbPositionDatabase {
    /// Pose definitions
    poses: Vec<Pose>,
    /// Pose names, shares the index with `.poses`
    names: Vec<String>,

    /// list of limbs to track, referenced by `LimbPosition.limb`.
    ///
    /// invariant: must contain only unique values
    /// contract: append only
    limbs: Vec<Limb>,
    limb_names: Vec<String>,
}

/// For accessing LimbPositionDatabase::limbs
#[derive(Clone, Copy, Debug)]
pub(crate) struct LimbIndex(usize);

#[derive(Debug)]
pub(crate) enum AddPoseError {
    MissingMirror(String),
}

impl Default for LimbPositionDatabase {
    fn default() -> Self {
        Self {
            poses: vec![],
            names: vec![],
            limbs: Limb::base_limbs(),
            limb_names: Limb::base_limb_names(),
        }
    }
}

impl LimbPositionDatabase {
    pub(crate) fn add(&mut self, poses: Vec<crate::pose_file::Pose>) -> Result<(), AddPoseError> {
        for pose in poses {
            let new_pose = if !pose.mirror_of.is_empty() {
                if let Some(i) = self.pose_by_id(&pose.mirror_of) {
                    self.pose_mirror(i)
                } else {
                    return Err(AddPoseError::MissingMirror(pose.mirror_of.clone()));
                }
            } else {
                self.new_pose(pose.direction, pose.limbs)
            };
            self.poses.push(new_pose);
            self.names.push(pose.name);
        }
        Ok(())
    }

    /// Take a list of limbs from a pose definition and produce a Pose.
    fn new_pose(
        &mut self,
        direction: pose_file::PoseDirection,
        limbs: Vec<pose_file::LimbPosition>,
    ) -> Pose {
        let limbs = limbs
            .into_iter()
            .map(|def| {
                let limb = Limb::from(def.limb);
                let index = self.find_or_insert_limb(limb);
                LimbPosition::new(
                    index,
                    SignedAngle::degree(def.angle as f32),
                    SignedAngle::degree(def.tolerance as f32),
                    def.weight,
                )
            })
            .collect();
        Pose::new(direction.into(), limbs)
    }

    /// Take an existing pose and produce a mirror pose of it.
    fn pose_mirror(&mut self, i: usize) -> Pose {
        let direction = self.poses[i].direction;
        let limbs = self.poses[i]
            .limbs
            .clone() // clone necessary to avoid double mutable borrow of limbs
            .into_iter()
            .map(|original_position| {
                let limb = self.limbs[original_position.limb.0].mirror();
                let index = self.find_or_insert_limb(limb);
                let target = match direction {
                    PoseDirection::Right => original_position.target,
                    PoseDirection::Front => {
                        // when left and right leg is switched, outside switches sign
                        original_position.target.mirror()
                    }
                };
                LimbPosition::from_limb_and_target(index, target)
            })
            .collect();
        Pose::new(direction, limbs)
    }

    fn find_or_insert_limb(&mut self, limb: Limb) -> LimbIndex {
        let index;
        if let Some(i) = self.limbs.iter().position(|l| *l == limb) {
            index = i;
        } else {
            index = self.limbs.len();
            self.limb_names.push(format!("{limb:?}"));
            self.limbs.push(limb);
        }
        LimbIndex(index)
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.poses.is_empty()
    }

    pub(crate) fn pose_by_id(&self, id: &str) -> Option<usize> {
        self.names.iter().position(|name| name == id)
    }

    pub(crate) fn pose_name(&self, i: usize) -> &str {
        &self.names[i]
    }

    pub(crate) fn limbs(&self) -> impl Iterator<Item = (LimbIndex, &Limb)> {
        (0..self.limbs.len()).map(LimbIndex).zip(self.limbs.iter())
    }

    pub(crate) fn limb_name(&self, i: LimbIndex) -> &str {
        &self.limb_names[i.0]
    }

    pub(crate) fn poses(&self) -> &[Pose] {
        &self.poses
    }

    /// A DB without default poses for testing where only one pose is needed.
    #[cfg(test)]
    pub(crate) fn test(target: super::geom::Angle3d) -> Self {
        let poses = vec![Pose::new(
            PoseDirection::Front,
            vec![LimbPosition::new(
                LimbIndex(0),
                target.project_2d(),
                SignedAngle::ZERO,
                1.0,
            )],
        )];

        Self {
            poses,
            names: vec!["test_pose".into()],
            limbs: vec![pose_file::Limb::LeftThigh.into()],
            limb_names: vec!["test_limb".into()],
        }
    }
}

impl Limb {
    pub(crate) const LEFT_THIGH: LimbIndex = LimbIndex(0);
    pub(crate) const LEFT_SHIN: LimbIndex = LimbIndex(1);
    pub(crate) const LEFT_FOOT: LimbIndex = LimbIndex(2);
    pub(crate) const LEFT_ARM: LimbIndex = LimbIndex(3);
    pub(crate) const LEFT_FOREARM: LimbIndex = LimbIndex(4);
    pub(crate) const RIGHT_THIGH: LimbIndex = LimbIndex(5);
    pub(crate) const RIGHT_SHIN: LimbIndex = LimbIndex(6);
    pub(crate) const RIGHT_FOOT: LimbIndex = LimbIndex(7);
    pub(crate) const RIGHT_ARM: LimbIndex = LimbIndex(8);
    pub(crate) const RIGHT_FOREARM: LimbIndex = LimbIndex(9);

    /// List of limbs that are always racked.
    /// They can be relied upon for rendering.
    pub(crate) fn base_limbs() -> Vec<Self> {
        vec![
            pose_file::Limb::LeftThigh.into(),
            pose_file::Limb::LeftShin.into(),
            pose_file::Limb::LeftFoot.into(),
            pose_file::Limb::LeftArm.into(),
            pose_file::Limb::LeftForearm.into(),
            pose_file::Limb::RightThigh.into(),
            pose_file::Limb::RightShin.into(),
            pose_file::Limb::RightFoot.into(),
            pose_file::Limb::RightArm.into(),
            pose_file::Limb::RightForearm.into(),
        ]
    }

    pub(crate) fn base_limb_names() -> Vec<String> {
        vec![
            "LeftThigh".into(),
            "LeftShin".into(),
            "LeftFoot".into(),
            "LeftArm".into(),
            "LeftForearm".into(),
            "RightThigh".into(),
            "RightShin".into(),
            "RightFoot".into(),
            "RightArm".into(),
            "RightForearm".into(),
        ]
    }
}

impl LimbIndex {
    pub(crate) fn as_usize(&self) -> usize {
        self.0
    }
}

impl From<LimbIndex> for usize {
    fn from(value: LimbIndex) -> Self {
        value.0
    }
}

impl From<AddPoseError> for ParseFileError {
    fn from(error: AddPoseError) -> Self {
        match error {
            AddPoseError::MissingMirror(id) => Self::UnknownPoseReference(id),
        }
    }
}
