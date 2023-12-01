//! Types and code for representing poses internally.
//!
//! Note that some types have a sibling type in [`pose_file`] for
//! serialization. Code for conversion from that format into the format in this
//! file is included here.

use crate::intern::geom::{Angle3d, SignedAngle};
use crate::public::keypoints::Cartesian3d;
use crate::public::pose_file;
use crate::Keypoints;

pub(crate) struct Pose {
    limbs: Vec<LimbPosition>,
}

pub(crate) struct LimbPosition {
    /// index to stored limbs
    limb: usize,
    /// range of polar angles considered zero error
    polar_range: (SignedAngle, SignedAngle),
    /// range of azimuth angles considered zero error
    azimuth_range: (SignedAngle, SignedAngle),
    /// weight used for computing the position error
    weight: f32,
}

#[derive(PartialEq, Eq, Hash)]
pub(crate) struct Limb {
    pub start: BodyPoint,
    pub end: BodyPoint,
}

#[derive(PartialEq, Eq, Hash)]
pub(crate) struct BodyPoint {
    side: BodySide,
    part: BodyPart,
}

#[derive(PartialEq, Eq, Hash)]
enum BodySide {
    Left,
    Right,
}

#[derive(PartialEq, Eq, Hash)]
enum BodyPart {
    Shoulder,
    Hip,
    Knee,
    Ankle,
    Elbow,
    Wrist,
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
        }
    }
}

impl Pose {
    pub(crate) fn new(limbs: Vec<LimbPosition>) -> Self {
        Self { limbs }
    }

    /// Error is between 0.0  and 1.0
    pub(crate) fn error(&self, angles: &[Angle3d]) -> f32 {
        let mut err = 0.0;
        let mut w = 0.0;
        for limb in &self.limbs {
            w += limb.weight;
            let angle = angles[limb.limb];
            err += range_error(angle.azimuth, limb.azimuth_range);
            err += range_error(angle.polar, limb.polar_range);
        }
        // (sus) normalize such that 45° away is 1.0
        let normalized = err / w / std::f32::consts::FRAC_PI_4.powi(2);
        // anything above 45° is a flat 100% error
        return normalized.min(1.0);
    }
}

impl Limb {
    pub(crate) const LEFT_THIGH: usize = 0;
    pub(crate) const LEFT_SHIN: usize = 1;
    pub(crate) const LEFT_ARM: usize = 2;
    pub(crate) const LEFT_FOREARM: usize = 3;
    pub(crate) const RIGHT_THIGH: usize = 4;
    pub(crate) const RIGHT_SHIN: usize = 5;
    pub(crate) const RIGHT_ARM: usize = 6;
    pub(crate) const RIGHT_FOREARM: usize = 7;

    /// List of limbs that are always racked.
    /// They can be relied upon for rendering.
    pub(crate) fn base_limbs() -> Vec<Self> {
        vec![
            pose_file::Limb::LeftThigh.into(),
            pose_file::Limb::LeftShin.into(),
            pose_file::Limb::LeftArm.into(),
            pose_file::Limb::LeftForearm.into(),
            pose_file::Limb::RightThigh.into(),
            pose_file::Limb::RightShin.into(),
            pose_file::Limb::RightArm.into(),
            pose_file::Limb::RightForearm.into(),
        ]
    }
}

impl LimbPosition {
    pub(crate) fn new(
        limb: usize,
        azimuth: SignedAngle,
        polar: SignedAngle,
        tolerance: SignedAngle,
        weight: f32,
    ) -> Self {
        Self {
            limb,
            azimuth_range: (azimuth - tolerance, azimuth + tolerance),
            polar_range: (polar - tolerance, polar + tolerance),
            weight,
        }
    }
}

impl From<pose_file::Limb> for Limb {
    fn from(other: pose_file::Limb) -> Self {
        match other {
            pose_file::Limb::LeftShin => Self {
                start: BodyPoint {
                    part: BodyPart::Knee,
                    side: BodySide::Left,
                },
                end: BodyPoint {
                    part: BodyPart::Ankle,
                    side: BodySide::Left,
                },
            },
            pose_file::Limb::LeftThigh => Self {
                start: BodyPoint {
                    part: BodyPart::Hip,
                    side: BodySide::Left,
                },
                end: BodyPoint {
                    part: BodyPart::Knee,
                    side: BodySide::Left,
                },
            },
            pose_file::Limb::LeftLeg => Self {
                start: BodyPoint {
                    part: BodyPart::Hip,
                    side: BodySide::Left,
                },
                end: BodyPoint {
                    part: BodyPart::Ankle,
                    side: BodySide::Left,
                },
            },
            pose_file::Limb::LeftArm => Self {
                start: BodyPoint {
                    part: BodyPart::Shoulder,
                    side: BodySide::Left,
                },
                end: BodyPoint {
                    part: BodyPart::Elbow,
                    side: BodySide::Left,
                },
            },
            pose_file::Limb::LeftForearm => Self {
                start: BodyPoint {
                    part: BodyPart::Elbow,
                    side: BodySide::Left,
                },
                end: BodyPoint {
                    part: BodyPart::Wrist,
                    side: BodySide::Left,
                },
            },
            pose_file::Limb::RightShin => Self {
                start: BodyPoint {
                    part: BodyPart::Knee,
                    side: BodySide::Right,
                },
                end: BodyPoint {
                    part: BodyPart::Ankle,
                    side: BodySide::Right,
                },
            },
            pose_file::Limb::RightThigh => Self {
                start: BodyPoint {
                    part: BodyPart::Hip,
                    side: BodySide::Right,
                },
                end: BodyPoint {
                    part: BodyPart::Knee,
                    side: BodySide::Right,
                },
            },
            pose_file::Limb::RightLeg => Self {
                start: BodyPoint {
                    part: BodyPart::Hip,
                    side: BodySide::Right,
                },
                end: BodyPoint {
                    part: BodyPart::Ankle,
                    side: BodySide::Right,
                },
            },
            pose_file::Limb::RightArm => Self {
                start: BodyPoint {
                    part: BodyPart::Shoulder,
                    side: BodySide::Right,
                },
                end: BodyPoint {
                    part: BodyPart::Elbow,
                    side: BodySide::Right,
                },
            },
            pose_file::Limb::RightForearm => Self {
                start: BodyPoint {
                    part: BodyPart::Elbow,
                    side: BodySide::Right,
                },
                end: BodyPoint {
                    part: BodyPart::Wrist,
                    side: BodySide::Right,
                },
            },

            pose_file::Limb::Custom { start, end } => Limb {
                start: start.into(),
                end: end.into(),
            },
        }
    }
}

impl From<pose_file::BodyPoint> for BodyPoint {
    fn from(other: pose_file::BodyPoint) -> Self {
        Self {
            part: other.part.into(),
            side: other.side.into(),
        }
    }
}

impl From<pose_file::BodyPart> for BodyPart {
    fn from(other: pose_file::BodyPart) -> Self {
        match other {
            pose_file::BodyPart::Shoulder => Self::Shoulder,
            pose_file::BodyPart::Hip => Self::Hip,
            pose_file::BodyPart::Knee => Self::Knee,
            pose_file::BodyPart::Ankle => Self::Ankle,
        }
    }
}

impl From<pose_file::BodySide> for BodySide {
    fn from(other: pose_file::BodySide) -> Self {
        match other {
            pose_file::BodySide::Left => Self::Left,
            pose_file::BodySide::Right => Self::Right,
        }
    }
}

fn range_error(value: SignedAngle, range: (SignedAngle, SignedAngle)) -> f32 {
    // TODO: add tests and fix (e.g. error around positive to negative ranges, which is broken now)
    let min = *range.0;
    let max = *range.1;
    if *value < min {
        (min - *value).powi(2)
    } else if *value > max {
        (*value - max).powi(2)
    } else {
        0.0
    }
}
