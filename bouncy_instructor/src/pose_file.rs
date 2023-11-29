//! Defines the external format for defining poses.
//!
//! Best practice: Don't use any of the type of this file outside of parsing
//! logic. Instead, translate to internal types. This allows refactoring
//! internal without changing the external formats.

use serde::Deserialize;
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
#[derive(Deserialize)]
pub(crate) struct Pose {
    pub name: String,
    pub limbs: Vec<LimbPosition>,
}

/// Describes a desired angle of a limb defined by start and end point.
#[derive(Deserialize)]
pub(crate) struct LimbPosition {
    pub limb: Limb,
    pub weight: f32,
    pub angle: i16,
    pub tolerance: u8,
}

#[derive(Deserialize)]
pub(crate) struct BodyPoint {
    pub side: BodySide,
    pub part: BodyPart,
}

/// Either a pre-defined limb or a custom pair of body points.
///
/// Custom points are maximally expressive but also verbose. Any limb that's
/// used frequently should probably be included in the pre-defined list.
#[derive(Deserialize)]
pub(crate) enum Limb {
    /// knee to ankle
    LeftShin,
    /// hip to knee
    LeftThigh,
    /// hip to ankle
    LeftLeg,
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
    Custom {
        start: BodyPoint,
        end: BodyPoint,
    },
}

#[derive(Deserialize)]
pub(crate) enum BodySide {
    Left,
    Right,
}

#[derive(Deserialize)]
pub(crate) enum BodyPart {
    Shoulder,
    Hip,
    Knee,
    Ankle,
}

#[derive(Error, Debug)]
pub(crate) enum ParseFileError {
    #[error("invalid pose file version (expected {expected:?}, found {found:?})")]
    VersionMismatch { expected: u16, found: u16 },
    #[error("parsing pose file failed, {0}")]
    RonError(#[from] ron::error::SpannedError),
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
