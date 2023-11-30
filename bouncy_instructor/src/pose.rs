//! Types and code for defining poses and comparing them to keypoints.
//!
//! Note that some types have a sibling type in [`crate::pose_file`] for
//! serialization.

use crate::keypoints::Coordinate3d;
use crate::skeleton::{Angle3d, SignedAngle, Skeleton3d};
use crate::Keypoints;
use std::f32::consts::PI;

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
}

struct Pose {
    limbs: Vec<LimbPosition>,
}

struct LimbPosition {
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
    start: BodyPoint,
    end: BodyPoint,
}

#[derive(PartialEq, Eq, Hash)]
struct BodyPoint {
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

impl Default for LimbPositionDatabase {
    fn default() -> Self {
        Self {
            poses: vec![],
            names: vec![],
            limbs: Limb::base_limbs(),
        }
    }
}

impl LimbPositionDatabase {
    pub(crate) fn add(&mut self, poses: Vec<crate::pose_file::Pose>) {
        for pose in poses {
            let limbs = pose
                .limbs
                .into_iter()
                .map(|def| {
                    let limb = Limb::from(def.limb);
                    let index;
                    if let Some(i) = self.limbs.iter().position(|l| *l == limb) {
                        index = i;
                    } else {
                        index = self.limbs.len();
                        self.limbs.push(limb);
                    }
                    // definition is in °, internal usage is in rad
                    let alpha = SignedAngle::degree(def.angle as f32);
                    let tolerance = SignedAngle::degree(def.tolerance as f32);

                    // convert from pose definition coordinates to spherical coordinates
                    //
                    // iterative implementation:
                    // now: only forward/backward angles are allowed
                    // future: explicitly define whether it is a side angle or a forward/backward angle
                    let azimuth = SignedAngle(if alpha.is_sign_positive() { 0.0 } else { PI });
                    let polar = SignedAngle(alpha.abs());

                    LimbPosition {
                        limb: index,
                        azimuth_range: (azimuth - tolerance, azimuth + tolerance),
                        polar_range: (polar - tolerance, polar + tolerance),
                        weight: def.weight,
                    }
                })
                .collect();
            self.poses.push(Pose { limbs });
            self.names.push(pose.name);
        }
    }

    pub(crate) fn angles_from_keypoints(&self, kp: &Keypoints) -> Vec<Angle3d> {
        self.limbs
            .iter()
            .map(|l| {
                let start = l.start.keypoint(kp);
                let end = l.end.keypoint(kp);
                Angle3d {
                    azimuth: start.azimuth(end),
                    polar: start.polar_angle(end),
                }
            })
            .collect()
    }

    pub(crate) fn fit(&self, skeleton: &Skeleton3d) -> (f32, usize) {
        assert!(!self.poses.is_empty());

        let mut best_error = f32::INFINITY;
        let mut best_i = 0;
        for (i, pose) in self.poses.iter().enumerate() {
            let err = pose.error(skeleton.angles());
            if err < best_error {
                best_error = err;
                best_i = i;
            }
        }
        return (best_error, best_i);
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.poses.is_empty()
    }

    pub(crate) fn pose_name(&self, i: usize) -> &str {
        &self.names[i]
    }
}

impl BodyPoint {
    fn keypoint(&self, kp: &Keypoints) -> Coordinate3d {
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
    /// Error is between 0.0  and 1.0
    fn error(&self, angles: &[Angle3d]) -> f32 {
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
    fn base_limbs() -> Vec<Self> {
        vec![
            crate::pose_file::Limb::LeftThigh.into(),
            crate::pose_file::Limb::LeftShin.into(),
            crate::pose_file::Limb::LeftArm.into(),
            crate::pose_file::Limb::LeftForearm.into(),
            crate::pose_file::Limb::RightThigh.into(),
            crate::pose_file::Limb::RightShin.into(),
            crate::pose_file::Limb::RightArm.into(),
            crate::pose_file::Limb::RightForearm.into(),
        ]
    }
}

impl From<crate::pose_file::Limb> for Limb {
    fn from(other: crate::pose_file::Limb) -> Self {
        match other {
            crate::pose_file::Limb::LeftShin => Self {
                start: BodyPoint {
                    part: BodyPart::Knee,
                    side: BodySide::Left,
                },
                end: BodyPoint {
                    part: BodyPart::Ankle,
                    side: BodySide::Left,
                },
            },
            crate::pose_file::Limb::LeftThigh => Self {
                start: BodyPoint {
                    part: BodyPart::Hip,
                    side: BodySide::Left,
                },
                end: BodyPoint {
                    part: BodyPart::Knee,
                    side: BodySide::Left,
                },
            },
            crate::pose_file::Limb::LeftLeg => Self {
                start: BodyPoint {
                    part: BodyPart::Hip,
                    side: BodySide::Left,
                },
                end: BodyPoint {
                    part: BodyPart::Ankle,
                    side: BodySide::Left,
                },
            },
            crate::pose_file::Limb::LeftArm => Self {
                start: BodyPoint {
                    part: BodyPart::Shoulder,
                    side: BodySide::Left,
                },
                end: BodyPoint {
                    part: BodyPart::Elbow,
                    side: BodySide::Left,
                },
            },
            crate::pose_file::Limb::LeftForearm => Self {
                start: BodyPoint {
                    part: BodyPart::Elbow,
                    side: BodySide::Left,
                },
                end: BodyPoint {
                    part: BodyPart::Wrist,
                    side: BodySide::Left,
                },
            },
            crate::pose_file::Limb::RightShin => Self {
                start: BodyPoint {
                    part: BodyPart::Knee,
                    side: BodySide::Right,
                },
                end: BodyPoint {
                    part: BodyPart::Ankle,
                    side: BodySide::Right,
                },
            },
            crate::pose_file::Limb::RightThigh => Self {
                start: BodyPoint {
                    part: BodyPart::Hip,
                    side: BodySide::Right,
                },
                end: BodyPoint {
                    part: BodyPart::Knee,
                    side: BodySide::Right,
                },
            },
            crate::pose_file::Limb::RightLeg => Self {
                start: BodyPoint {
                    part: BodyPart::Hip,
                    side: BodySide::Right,
                },
                end: BodyPoint {
                    part: BodyPart::Ankle,
                    side: BodySide::Right,
                },
            },
            crate::pose_file::Limb::RightArm => Self {
                start: BodyPoint {
                    part: BodyPart::Shoulder,
                    side: BodySide::Right,
                },
                end: BodyPoint {
                    part: BodyPart::Elbow,
                    side: BodySide::Right,
                },
            },
            crate::pose_file::Limb::RightForearm => Self {
                start: BodyPoint {
                    part: BodyPart::Elbow,
                    side: BodySide::Right,
                },
                end: BodyPoint {
                    part: BodyPart::Wrist,
                    side: BodySide::Right,
                },
            },

            crate::pose_file::Limb::Custom { start, end } => Limb {
                start: start.into(),
                end: end.into(),
            },
        }
    }
}

impl From<crate::pose_file::BodyPoint> for BodyPoint {
    fn from(other: crate::pose_file::BodyPoint) -> Self {
        Self {
            part: other.part.into(),
            side: other.side.into(),
        }
    }
}

impl From<crate::pose_file::BodyPart> for BodyPart {
    fn from(other: crate::pose_file::BodyPart) -> Self {
        match other {
            crate::pose_file::BodyPart::Shoulder => Self::Shoulder,
            crate::pose_file::BodyPart::Hip => Self::Hip,
            crate::pose_file::BodyPart::Knee => Self::Knee,
            crate::pose_file::BodyPart::Ankle => Self::Ankle,
        }
    }
}

impl From<crate::pose_file::BodySide> for BodySide {
    fn from(other: crate::pose_file::BodySide) -> Self {
        match other {
            crate::pose_file::BodySide::Left => Self::Left,
            crate::pose_file::BodySide::Right => Self::Right,
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
