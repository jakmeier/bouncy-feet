//! Defines the external format for defining poses, which are still positions of
//! a body.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::ParseFileError;

const CURRENT_VERSION: u16 = 0;

/// Format for pose definition files.
#[derive(Deserialize)]
pub(crate) struct PoseFile {
    pub version: u16,
    pub poses: Vec<Pose>,
}

/// Description of a body position.
///
/// This includes the exact desired position range and a name.
/// This is the format for external files and loaded in at runtime.
/// It is converted to a [`crate::pose::Pose`] for computations.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub(crate) struct Pose {
    pub name: String,
    pub direction: PoseDirection,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub limbs: Vec<LimbPosition>,
    /// Move the entire body in the horizontal direction
    #[serde(default, skip_serializing_if = "is_zero")]
    pub x_shift: f32,
    /// Move the entire body in the vertical direction
    #[serde(default, skip_serializing_if = "is_zero")]
    pub y_shift: f32,
    /// Degrees of turned shoulder from base direction. More than 45° won't
    /// work, use different direction instead.
    #[serde(default, skip_serializing_if = "zero")]
    pub turn_shoulder: i16,
    /// Degrees of turned hip from base direction.
    #[serde(default, skip_serializing_if = "zero")]
    pub turn_hip: i16,
    /// The Z-Axis is the distance to the camera. It can only be measured quite
    /// inaccurately from the camera image, hence poses define only relative
    /// position comparisons instead of numeric coordinates.
    #[serde(default, skip_serializing_if = "PoseZ::is_empty")]
    pub z: PoseZ,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub mirror_of: String,
}

/// Describes a desired angle of a limb defined by start and end point.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub(crate) struct LimbPosition {
    pub limb: Limb,
    pub weight: f32,
    /// Angle in the forward direction (negative for backward).
    ///
    /// Depending on whether the pose faces the camera or to the side, this
    /// angle either means forward or sidewards. From a non-mirrored camera
    /// point of view, it's always to the left in the video.
    pub angle: i16,
    pub tolerance: u8,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) struct BodyPoint {
    pub side: BodySide,
    pub part: BodyPart,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub(crate) struct PoseZ {
    /// +1 is maximally stretched towards the camera, -1 away from the camera,
    /// 0.0 is neutral and default for all unspecified points.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub absolute: HashMap<BodyPoint, f32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub order: Vec<BodyPartOrdering>,
}

/// Defines a simple is-in-front-of relation between two body parts.
///
/// This is more or less the only thing that can be consistently checked in a
/// video. But is it good enough to draw skeletons properly in a 2D projection?
/// Without position per body part info, not really. To know the length of a
/// limb, for example the thigh with a raised knee towards the camera, one must
/// know how much forward the knee is compared to the hip and the foot.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) struct BodyPartOrdering {
    pub forward: BodyPoint,
    pub backward: BodyPoint,
}

/// Either a pre-defined limb or a custom pair of body points.
///
/// Custom points are maximally expressive but also verbose. Any limb that's
/// used frequently should probably be included in the pre-defined list.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub(crate) enum Limb {
    /// knee to ankle
    LeftShin,
    /// hip to knee
    LeftThigh,
    /// hip to ankle
    LeftLeg,
    /// heel to toe
    LeftFoot,
    /// shoulder to elbow
    LeftArm,
    /// elbow to wrist
    LeftForearm,
    /// knee to ankle
    RightShin,
    /// hip to knee
    RightThigh,
    /// hip to ankle
    RightLeg,
    /// shoulder to elbow
    RightArm,
    /// elbow to wrist
    RightForearm,
    /// heel to toe
    RightFoot,
    Custom {
        start: BodyPoint,
        end: BodyPoint,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) enum BodySide {
    Left,
    Right,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum PoseDirection {
    /// Dancer faces the camera.
    Front,
    /// Dancer faces to their right. (Left in non-mirrored video.)
    Right,
}

impl PoseFile {
    pub(crate) fn from_str(text: &str) -> Result<Self, ParseFileError> {
        let parsed: PoseFile = ron::from_str(text)?;
        if parsed.version != CURRENT_VERSION {
            return Err(ParseFileError::VersionMismatch {
                expected: CURRENT_VERSION,
                found: parsed.version,
            });
        }
        Ok(parsed)
    }
}

impl PoseZ {
    pub fn is_empty(&self) -> bool {
        self.absolute.is_empty() && self.order.is_empty()
    }
}

impl BodyPoint {
    pub fn default_pivot() -> Self {
        BodyPoint {
            side: BodySide::Left,
            part: BodyPart::Hip,
        }
    }

    pub fn is_default_pivot(&self) -> bool {
        *self == Self::default_pivot()
    }
}

fn is_zero(f: &f32) -> bool {
    f.abs() < 1e-6
}

fn zero(i: &i16) -> bool {
    *i == 0
}
