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

pub(crate) struct Pose {
    pub(crate) limbs: Vec<LimbPosition>,
}

#[derive(Clone)]
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
    pub(crate) fn new(limbs: Vec<LimbPosition>) -> Self {
        Self { limbs }
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
        azimuth: SignedAngle,
        polar: SignedAngle,
        tolerance: SignedAngle,
        weight: f32,
    ) -> Self {
        let angle = Angle3d::new(azimuth, polar);
        let target = AngleTarget::new(angle, tolerance, weight);

        Self::from_limb_and_target(limb, target)
    }

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
        let mut angle = Angle3d::ZERO;

        if let Some(forward) = forward {
            let azimuth = SignedAngle::radian(if forward.is_positive() { 0.0 } else { PI });
            let polar = SignedAngle::degree(forward as f32).abs();
            angle.azimuth = angle.azimuth + azimuth;
            angle.polar = angle.polar + polar;
        }

        if let Some(right) = right {
            let azimuth = SignedAngle::radian(right.signum() as f32 * FRAC_PI_2);
            let polar = SignedAngle::degree(right as f32).abs();
            angle.azimuth = angle.azimuth + azimuth;
            angle.polar = angle.polar + polar;
        }

        Self::new(
            limb,
            angle.azimuth,
            angle.polar,
            SignedAngle::degree(tolerance as f32),
            weight,
        )
    }

    pub(crate) fn weight(&self) -> f32 {
        self.target.weight()
    }
}
