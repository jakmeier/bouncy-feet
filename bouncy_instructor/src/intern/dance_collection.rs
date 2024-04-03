//! Types and code for keeping a list of poses to detect across method calls.
//!
//! Poses are defined with regard to a list of limb definitions, in order to
//! avoid storing the limb definitions repeatedly. The downside is that each
//! data set (one data set per video frame) on its own lacks context. You have
//! to combine it with the list of poses. This module contains the context
//!  structs required for this.

use super::dance::Dance;
use super::geom::SignedAngle;
use super::pose::{BodyPartOrdering, BodyPoint, Limb, LimbPosition, Pose, PoseDirection};
use super::skeleton_3d::Direction;
use super::step::Step;
use crate::parsing::ParseFileError;
use crate::pose_file::PoseZ;
use crate::skeleton::Cartesian2d;
use crate::step_file::{self, Orientation};
use crate::{dance_file, pose_file, AddDanceError, AddStepError};

/// List of registered poses/steps/dances to recognize during tracking.
///
/// Each pose is a description of a body position. This includes the exact
/// desired position, a name, and most importantly, the formulas for computing
/// an error.
///
/// Actual poses are defined in external files and loaded in at runtime. Here
/// data they are stored in the most convenient way, which will see many
/// refactorings over time.
#[derive(Clone)]
pub(crate) struct DanceCollection {
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

    steps: Vec<Step>,
    dances: Vec<Dance>,
}

/// For accessing LimbPositionDatabase::limbs
#[derive(Clone, Copy, Debug)]
pub(crate) struct LimbIndex(usize);

#[derive(Debug)]
pub(crate) enum AddPoseError {
    MissingMirror(String),
}

impl Default for DanceCollection {
    fn default() -> Self {
        Self {
            poses: vec![],
            names: vec![],
            limbs: Limb::base_limbs(),
            limb_names: Limb::base_limb_names(),
            steps: Default::default(),
            dances: Default::default(),
        }
    }
}

impl DanceCollection {
    pub(crate) fn add_poses(
        &mut self,
        poses: Vec<crate::pose_file::Pose>,
    ) -> Result<(), AddPoseError> {
        for pose in poses {
            let new_pose = if !pose.mirror_of.is_empty() {
                if let Some(i) = self.pose_by_id(&pose.mirror_of) {
                    self.pose_mirror(i)
                } else {
                    return Err(AddPoseError::MissingMirror(pose.mirror_of.clone()));
                }
            } else {
                self.new_pose(
                    pose.direction,
                    pose.limbs,
                    Cartesian2d::new(-pose.x_shift, -pose.y_shift),
                    SignedAngle::degree(pose.turn_shoulder as f32),
                    SignedAngle::degree(pose.turn_hip as f32),
                    pose.z,
                )
            };
            self.poses.push(new_pose);
            self.names.push(pose.name);
        }
        Ok(())
    }

    /// Copies a pose from a different dance collection
    fn add_foreign_pose(&mut self, other: &Self, pose_index: usize) -> usize {
        let pose = &other.poses()[pose_index];

        let limbs = pose
            .limbs
            .iter()
            .map(|limb| {
                let index = self.find_or_insert_limb(other.limbs[limb.limb.0]);
                LimbPosition {
                    limb: index,
                    target: limb.target.clone(),
                }
            })
            .collect();

        let new_pose = Pose::new(
            pose.direction,
            limbs,
            pose.shift,
            pose.turn_shoulder,
            pose.turn_hip,
            pose.z_absolute.clone(),
            pose.z_order.clone(),
        );

        let new_index = self.poses.len();
        self.poses.push(new_pose);
        self.names.push(other.pose_name(pose_index).to_owned());
        new_index
    }

    /// Take data from a pose definition and produce a Pose.
    fn new_pose(
        &mut self,
        direction: pose_file::PoseDirection,
        limbs: Vec<pose_file::LimbPosition>,
        shift: Cartesian2d,
        turn_shoulder: SignedAngle,
        turn_hip: SignedAngle,
        z: PoseZ,
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
        let z_order = z.order.into_iter().map(From::from).collect();
        let z_absolute = z.absolute.into_iter().map(|(k, v)| (k.into(), v)).collect();
        Pose::new(
            direction.into(),
            limbs,
            shift,
            turn_shoulder,
            turn_hip,
            z_absolute,
            z_order,
        )
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
        let z_absolute = self.poses[i]
            .z_absolute
            .iter()
            .map(|(body_point, &value)| (body_point.mirror(), value))
            .collect();
        let z_order = self.poses[i]
            .z_order
            .iter()
            .map(From::from)
            .map(BodyPartOrdering::mirror)
            .collect();
        let other = &self.poses[i];
        Pose::new(
            direction,
            limbs,
            other.shift.mirror(),
            other.turn_shoulder.mirror(),
            other.turn_hip.mirror(),
            z_absolute,
            z_order,
        )
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

    pub(crate) fn limb(&self, index: LimbIndex) -> &Limb {
        &self.limbs[index.as_usize()]
    }

    pub(crate) fn limb_name(&self, i: LimbIndex) -> &str {
        &self.limb_names[i.0]
    }

    pub(crate) fn poses(&self) -> &[Pose] {
        &self.poses
    }

    pub(crate) fn steps(&self) -> &[Step] {
        &self.steps
    }

    pub(crate) fn dances(&self) -> &[Dance] {
        &self.dances
    }

    pub(crate) fn add_steps(&mut self, steps: &[step_file::Step]) -> Result<(), AddStepError> {
        for def in steps {
            let poses = def
                .keyframes
                .iter()
                .map(|frame| {
                    self.pose_by_id(&frame.pose)
                        .ok_or_else(|| AddStepError::MissingPose(frame.pose.clone()))
                })
                .collect::<Result<_, _>>()?;

            let directions = def
                .keyframes
                .iter()
                .map(|frame| match frame.orientation {
                    Orientation::ToCamera => Direction::North,
                    Orientation::Right => Direction::East,
                    Orientation::Away => Direction::South,
                    Orientation::Left => Direction::West,
                    Orientation::Any => Direction::Unknown,
                })
                .collect();

            let pivots = def
                .keyframes
                .iter()
                .map(|frame| BodyPoint::from(frame.pivot.clone()))
                .collect();

            let new_step = Step {
                id: def.id.clone(),
                name: def.name.clone(),
                variation: def.variation.clone(),
                poses,
                directions,
                pivots,
            };
            self.steps.push(new_step);
        }
        Ok(())
    }

    /// Copies a step from a different dance collection, including poses.
    pub(crate) fn add_foreign_step(
        &mut self,
        other: &Self,
        id: &str,
    ) -> Result<(), ForeignCollectionError> {
        let step = other
            .step(id)
            .ok_or_else(|| ForeignCollectionError::MissingStep(id.to_owned()))?;
        let poses = step
            .poses
            .iter()
            .map(|&foreign_pose_index| {
                let pose_id = other.pose_name(foreign_pose_index);
                self.pose_by_id(pose_id)
                    .unwrap_or_else(|| self.add_foreign_pose(other, foreign_pose_index))
            })
            .collect();
        let new_step = Step {
            id: step.id.clone(),
            name: step.name.clone(),
            variation: step.variation.clone(),
            poses,
            directions: step.directions.clone(),
            pivots: step.pivots.clone(),
        };
        self.steps.push(new_step);
        Ok(())
    }

    pub(crate) fn add_dances(
        &mut self,
        dances: Vec<dance_file::Dance>,
    ) -> Result<(), AddDanceError> {
        for def in dances {
            if let Some(missing) = def
                .steps
                .iter()
                .find(|step_id| self.step(step_id).is_none())
            {
                return Err(AddDanceError::MissingStep(missing.clone()));
            }

            let dance = Dance {
                id: def.id,
                step_ids: def.steps,
            };
            self.dances.push(dance);
        }
        Ok(())
    }

    pub(crate) fn step(&self, id: &str) -> Option<&Step> {
        self.steps.iter().find(|step| step.id == id)
    }

    pub(crate) fn steps_by_name<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &Step> {
        self.steps.iter().filter(move |step| step.name == name)
    }

    pub(crate) fn idle_steps(&self) -> impl Iterator<Item = &Step> {
        self.steps.iter().filter(|step| step.id.contains("idle"))
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
            Cartesian2d::default(),
            SignedAngle::ZERO,
            SignedAngle::ZERO,
            Default::default(),
            Default::default(),
        )];

        Self {
            poses,
            names: vec!["test_pose".into()],
            limbs: vec![pose_file::Limb::LeftThigh.into()],
            limb_names: vec!["test_limb".into()],
            ..Default::default()
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

#[derive(thiserror::Error, Debug)]
pub enum ForeignCollectionError {
    #[error("internal error, could not find {0}")]
    MissingStep(String),
}

impl From<ForeignCollectionError> for wasm_bindgen::JsValue {
    fn from(value: ForeignCollectionError) -> Self {
        format!("{value}").into()
    }
}
