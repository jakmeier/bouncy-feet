//! Boilerplate code for converting between the pose definition types and the
//! internal types.
//!
//! Keeping the code here keeps other modules small and tidy. Also, it
//! encapsulates the glue between public / internal. But it needs to be in a
//! ancestor module of pose.rs because it needs access to private fields and
//! types.

use crate::intern::pose::{BodyPart, BodyPoint, BodySide, Limb};
use crate::pose_file;

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
