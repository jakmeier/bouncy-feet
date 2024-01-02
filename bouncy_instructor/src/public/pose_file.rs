//! Defines the external format for defining poses, which are still positions of
//! a body.
//!
//! Best practice: Don't use any of the type of this file outside of parsing
//! logic. Instead, translate to internal types. This allows refactoring
//! internal without changing the external formats.

use serde::{Deserialize, Serialize};
use thiserror::Error;
use wasm_bindgen::JsValue;

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
    // TODO: define in which z-order two body parts should be
    // pub body: Vec<BodyPartOrdering>
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub(crate) struct BodyPoint {
    pub side: BodySide,
    pub part: BodyPart,
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub(crate) enum BodySide {
    Left,
    Right,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum PoseDirection {
    /// Dancer faces the camera.
    Front,
    /// Dancer faces to their right. (Left in non-mirrored video.)
    Right,
}

#[derive(Error, Debug)]
pub enum ParseFileError {
    #[error("invalid pose file version (expected {expected:?}, found {found:?})")]
    VersionMismatch { expected: u16, found: u16 },
    #[error("parsing pose file failed, {0}")]
    RonError(#[from] ron::error::SpannedError),
    #[error("unknown pose reference `{0}`")]
    UnknownPoseReference(String),
    #[error("unknown step reference `{0}`")]
    UnknownStepName(String),
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

impl From<ParseFileError> for JsValue {
    fn from(value: ParseFileError) -> Self {
        format!("{value}").into()
    }
}
