//! Types and code for representing poses internally.
//!
//! Note that some types have a sibling type in [`pose_file`] for
//! serialization. Code for conversion from that format into the format in this
//! file is included here.

mod conversion;

use super::geom::SignedAngle;
use super::pose_db::LimbIndex;
use super::pose_score::AngleTarget;
use crate::intern::geom::Angle3d;
use crate::public::keypoints::Cartesian3d;
use crate::Keypoints;
use std::f32::consts::{FRAC_PI_2, PI};

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
    pub(crate) fn to_angle(&self, kp: &Keypoints) -> Angle3d {
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

    /// Applies two rotations, the forward rotation first and then the right rotation.
    ///
    /// Note: This is currently not used because the comparisons are made in 2D
    /// projections. But I might reconsider 3D comparison in the future. And
    /// then we also need 3D input.
    #[allow(dead_code)]
    pub(crate) fn from_orthogonal_angles(
        limb: LimbIndex,
        forward: Option<i16>,
        right: Option<i16>,
        tolerance: u8,
        weight: f32,
    ) -> Self {
        // convert from pose definition coordinates to spherical coordinates
        // definition is in °, internal usage is in rad
        // definition uses two 2D angle which need to be combined into a 3D angle
        let angle = match (forward, right) {
            (None, None) => Angle3d::ZERO,
            (None, Some(right)) => {
                let azimuth = SignedAngle::radian(right.signum() as f32 * FRAC_PI_2);
                let polar = SignedAngle::degree(right.abs() as f32);
                Angle3d::new(azimuth, polar)
            }
            (Some(forward), None) => {
                let azimuth = SignedAngle::radian(if forward.is_negative() { PI } else { 0.0 });
                let polar = SignedAngle::degree(forward.abs() as f32);
                Angle3d::new(azimuth, polar)
            }
            (Some(forward), Some(right)) => {
                // Here we have 2 angles to be combined.
                let right = SignedAngle::degree(right as f32);
                let forward = SignedAngle::degree(forward as f32);
                Angle3d::from_rotations(forward, right)
            }
        };

        Self::new(
            limb,
            // angle.azimuth,
            angle.polar,
            SignedAngle::degree(tolerance as f32),
            weight,
        )
    }

    pub(crate) fn weight(&self) -> f32 {
        self.target.weight()
    }
}
