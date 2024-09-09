//! Types and code for representing poses internally.
//!
//! Note that some types have a sibling type in [`pose_file`] for
//! serialization. Code for conversion from that format into the format in this
//! file is included here.

mod approximation;
mod conversion;

use super::dance_collection::{DanceCollection, LimbIndex};
use super::geom::SignedAngle;
use super::pose_score::AngleTarget;
use super::skeleton_3d::Direction;
use crate::intern::geom::Angle3d;
use crate::public::keypoints::Cartesian3d;
use crate::skeleton::Cartesian2d;
use crate::Keypoints;
use std::collections::HashMap;
use strum::IntoEnumIterator;

#[derive(Clone, Debug)]
pub(crate) struct Pose {
    pub(crate) direction: PoseDirection,
    pub(crate) limbs: Vec<LimbPosition>,
    pub(crate) shift: Cartesian2d,
    pub(crate) turn_shoulder: SignedAngle,
    pub(crate) turn_hip: SignedAngle,
    pub(crate) z_absolute: HashMap<BodyPoint, f32>,
    pub(crate) z_order: Vec<BodyPartOrdering>,
}

#[derive(Clone, Debug)]
pub(crate) struct LimbPosition {
    /// index to stored limbs
    pub(crate) limb: LimbIndex,
    /// position and error computation for this limb position
    pub(crate) target: AngleTarget,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub(crate) struct Limb {
    pub(crate) start: BodyPoint,
    pub(crate) end: BodyPoint,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub(crate) struct BodyPoint {
    pub(crate) side: BodySide,
    pub(crate) part: BodyPart,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) struct BodyPartOrdering {
    pub(crate) forward: BodyPoint,
    pub(crate) backward: BodyPoint,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub(crate) enum BodySide {
    Left,
    Right,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, strum::EnumIter)]
pub(crate) enum BodyPart {
    Shoulder,
    Hip,
    Knee,
    Ankle,
    Elbow,
    Wrist,
    Heel,
    Toes,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum PoseDirection {
    /// Dancer faces the camera.
    Front,
    /// Dancer faces to their right. (Left in non-mirrored video.)
    Right,
}

impl BodyPoint {
    pub(crate) fn keypoint(&self, kp: &Keypoints) -> Cartesian3d {
        let side = match self.side {
            BodySide::Left => kp.left,
            BodySide::Right => kp.right,
        };
        match self.part {
            BodyPart::Shoulder => side.shoulder,
            BodyPart::Hip => side.hip,
            BodyPart::Knee => side.knee,
            BodyPart::Ankle => side.ankle,
            BodyPart::Elbow => side.elbow,
            BodyPart::Wrist => side.wrist,
            BodyPart::Heel => side.heel,
            BodyPart::Toes => side.toes,
        }
    }

    pub(crate) fn mirror(&self) -> Self {
        Self {
            side: match self.side {
                BodySide::Left => BodySide::Right,
                BodySide::Right => BodySide::Left,
            },
            part: self.part,
        }
    }

    pub(crate) fn iter() -> impl Iterator<Item = Self> {
        let left = BodyPart::iter().map(|part| Self {
            side: BodySide::Left,
            part,
        });
        let right = BodyPart::iter().map(|part| Self {
            side: BodySide::Right,
            part,
        });
        left.chain(right)
    }

    pub(crate) fn attached_limbs<'a>(
        &'a self,
        db: &'a DanceCollection,
    ) -> impl Iterator<Item = (LimbIndex, &'a Limb)> {
        db.limbs()
            .filter(|(_index, limb)| limb.start == *self || limb.end == *self)
    }
}

impl Pose {
    pub(crate) fn new(
        direction: PoseDirection,
        limbs: Vec<LimbPosition>,
        shift: Cartesian2d,
        turn_shoulder: SignedAngle,
        turn_hip: SignedAngle,
        z_absolute: HashMap<BodyPoint, f32>,
        z_order: Vec<BodyPartOrdering>,
    ) -> Self {
        Self {
            direction,
            limbs,
            shift,
            turn_shoulder,
            turn_hip,
            z_absolute,
            z_order,
        }
    }
}

impl Limb {
    pub(crate) fn to_angle(self, kp: &Keypoints) -> Angle3d {
        let start = self.start.keypoint(kp);
        let end = self.end.keypoint(kp);
        Angle3d {
            azimuth: start.azimuth(end),
            polar: start.polar_angle(end),
        }
    }

    pub(crate) fn z(self, kp: &Keypoints) -> f32 {
        let start = self.start.keypoint(kp);
        let end = self.end.keypoint(kp);
        -(start.z + end.z) / 2.0
    }

    pub(crate) fn mirror(&self) -> Self {
        Self {
            start: self.start.mirror(),
            end: self.end.mirror(),
        }
    }
}

impl LimbPosition {
    pub(crate) fn from_limb_and_target(limb: LimbIndex, target: AngleTarget) -> Self {
        Self { limb, target }
    }
    pub(crate) fn new(
        limb: LimbIndex,
        polar: SignedAngle,
        tolerance: SignedAngle,
        weight: f32,
    ) -> Self {
        let target = AngleTarget::new(polar, tolerance, weight);
        Self::from_limb_and_target(limb, target)
    }

    pub(crate) fn weight(&self) -> f32 {
        self.target.weight()
    }
}

impl PoseDirection {
    pub(crate) fn matches_direction(&self, direction: Direction) -> bool {
        match direction {
            Direction::North | Direction::South => matches!(self, Self::Front),
            Direction::East | Direction::West => matches!(self, Self::Right),
            Direction::Unknown => true,
        }
    }
}

impl BodyPartOrdering {
    pub(crate) fn mirror(&self) -> Self {
        Self {
            backward: self.backward.mirror(),
            forward: self.forward.mirror(),
        }
    }

    pub(crate) fn satisfied(&self, positions: &HashMap<BodyPoint, Cartesian3d>) -> bool {
        // Note: This check should probably be more sophisticated, eventually.
        positions[&self.forward].z < positions[&self.backward].z
    }
}

impl std::fmt::Debug for BodyPartOrdering {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{0:?} {1:?} behind {2:?} {3:?}",
            self.backward.side, self.backward.part, self.forward.side, self.forward.part,
        )
    }
}
