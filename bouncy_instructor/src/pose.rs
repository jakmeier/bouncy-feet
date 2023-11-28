//! Types and code for defining poses and comparing them to keypoints.
//!
//! Note that some types have a sibling type in [`crate::pose_file`] for
//! serialization.

use crate::keypoints::Coordinate3d;
use crate::skeleton::Skeleton;
use crate::Keypoints;

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
#[derive(Default)]
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
    /// range of angles considered zero error
    angle: (f32, f32),
    /// weight used for computing the position error
    weight: f32,
}

#[derive(PartialEq, Eq, Hash)]
struct Limb {
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
                    let alpha = def.angle as f32;
                    let tolerance = def.tolerance as f32;
                    LimbPosition {
                        limb: index,
                        angle: (alpha - tolerance, alpha + tolerance),
                        weight: def.weight,
                    }
                })
                .collect();
            self.poses.push(Pose { limbs });
            self.names.push(pose.name);
        }
    }

    pub(crate) fn angles_from_keypoints(&self, kp: &Keypoints) -> Vec<f32> {
        self.limbs
            .iter()
            .map(|l| {
                let start = l.start.keypoint(kp);
                let end = l.end.keypoint(kp);
                start.signed_polar_angle(end)
            })
            .collect()
    }

    pub(crate) fn fit(&self, skeleton: &Skeleton) -> (f32, usize) {
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
        }
    }
}

impl Pose {
    /// Error is between 0.0  and 1.0
    fn error(&self, angles: &[f32]) -> f32 {
        let mut err = 0.0;
        let mut w = 0.0;
        for limb in &self.limbs {
            w += limb.weight;
            let angle = angles[limb.limb];
            if angle < limb.angle.0 {
                err += (angle - limb.angle.0).powi(2);
            } else if angle > limb.angle.1 {
                err += (angle - limb.angle.1).powi(2);
            }
        }
        // normalize such that 45° away is 1.0
        let normalized = err / w / (45u32.pow(2) as f32);
        // anything above 45° is a flat 100% error
        return normalized.min(1.0);
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
