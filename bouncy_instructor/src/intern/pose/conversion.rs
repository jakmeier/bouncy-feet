//! Boilerplate code for converting between the pose definition types and the
//! internal types.
//!
//! Keeping the code here keeps other modules small and tidy. Also, it
//! encapsulates the glue between public / internal. But it needs to be in a
//! ancestor module of pose.rs because it needs access to private fields and
//! types.
//!
//! There are 2 conversions in here.
//! 1) pose_file::Pose -> intern::pose::Pose
//! 2) Skeleton3d -> pose_file::Pose

use std::f32::consts::TAU;

use crate::intern::pose::{BodyPart, BodyPoint, BodySide, Limb};
use crate::intern::skeleton_3d::Skeleton3d;
use crate::{pose_file, STATE};

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

impl From<BodyPoint> for pose_file::BodyPoint {
    fn from(other: BodyPoint) -> Self {
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
            pose_file::BodyPart::Elbow => Self::Elbow,
            pose_file::BodyPart::Wrist => Self::Wrist,
        }
    }
}

impl From<BodyPart> for pose_file::BodyPart {
    fn from(other: BodyPart) -> Self {
        match other {
            BodyPart::Shoulder => Self::Shoulder,
            BodyPart::Hip => Self::Hip,
            BodyPart::Knee => Self::Knee,
            BodyPart::Ankle => Self::Ankle,
            BodyPart::Elbow => Self::Elbow,
            BodyPart::Wrist => Self::Wrist,
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

impl From<BodySide> for pose_file::BodySide {
    fn from(other: BodySide) -> Self {
        match other {
            BodySide::Left => Self::Left,
            BodySide::Right => Self::Right,
        }
    }
}

impl From<&Skeleton3d> for pose_file::Pose {
    fn from(skeleton: &Skeleton3d) -> Self {
        let limbs = STATE.with_borrow(|state| {
            skeleton
                .angles()
                .iter()
                .zip(state.db.limbs())
                .map(|(angle, (_limb_index, &limb))| {
                    let x = -angle.polar.sin() * angle.azimuth.sin();
                    let z = angle.polar.cos() * -angle.azimuth.cos();
                    let y = angle.polar.cos();

                    let x_angle = (y.atan2(x) + TAU) % TAU;
                    let z_angle = (z.atan2(x) + TAU) % TAU;

                    crate::pose_file::LimbPosition {
                        limb: limb.into(),
                        weight: 1.0,
                        forward: Some(x_angle.to_degrees() as i16),
                        right: Some(z_angle.to_degrees() as i16),
                        tolerance: 0,
                    }
                })
                .collect::<Vec<_>>()
        });
        Self {
            limbs,
            name: "Generated Pose".to_owned(),
        }
    }
}

impl From<Limb> for pose_file::Limb {
    fn from(other: Limb) -> Self {
        Self::Custom {
            start: other.start.into(),
            end: other.end.into(),
        }
    }
}
