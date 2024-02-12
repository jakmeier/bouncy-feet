//! Types and code for representing poses internally.
//!
//! Note that some types have a sibling type in [`pose_file`] for
//! serialization. Code for conversion from that format into the format in this
//! file is included here.

mod approximation;
mod conversion;

use super::geom::SignedAngle;
use super::pose_db::LimbIndex;
use super::pose_score::AngleTarget;
use crate::intern::geom::Angle3d;
use crate::public::keypoints::Cartesian3d;
use crate::Keypoints;

#[derive(Debug)]
pub(crate) struct Pose {
    pub(crate) direction: PoseDirection,
    pub(crate) limbs: Vec<LimbPosition>,
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
    start: BodyPoint,
    end: BodyPoint,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct BodyPoint {
    side: BodySide,
    part: BodyPart,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum BodySide {
    Left,
    Right,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum BodyPart {
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

    fn mirror(&self) -> Self {
        Self {
            side: match self.side {
                BodySide::Left => BodySide::Right,
                BodySide::Right => BodySide::Left,
            },
            part: self.part,
        }
    }
}

impl Pose {
    pub(crate) fn new(direction: PoseDirection, limbs: Vec<LimbPosition>) -> Self {
        Self { direction, limbs }
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
