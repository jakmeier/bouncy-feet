//! Types and code for defining poses and comparing them to keypoints.
//!
//! Note that some types have a sibling type in [`crate::pose_file`] for
//! serialization.

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
    angle: (i16, i16),
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
                    LimbPosition {
                        limb: index,
                        angle: def.angle,
                        weight: def.weight,
                    }
                })
                .collect();
            self.poses.push(Pose { limbs });
            self.names.push(pose.name);
        }
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