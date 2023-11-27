use serde::Deserialize;
use thiserror::Error;
use wasm_bindgen::JsValue;

const CURRENT_VERSION: u16 = 0;

/// Format for pose definition files.
#[derive(Deserialize)]
pub struct PoseFile {
    pub version: u16,
    pub poses: Vec<Pose>,
}

/// Description of a body position.
///
/// This includes the exact desired position, a name, and most importantly, the
/// formulas for computing an error.
///
/// Actual poses are defined in external files and loaded in at runtime.
///
/// Errors are always between 0 and 1, where 0 is a perfect match.
#[derive(Deserialize)]
pub struct Pose {
    name: String,
    limbs: Vec<LimbPosition>,
}

/// Describes a desired angle of a limb defined by start and end point.
#[derive(Deserialize)]
struct LimbPosition {
    start: BodyPoint,
    end: BodyPoint,
    angle: (i16, i16),
    weight: f32,
}

#[derive(Deserialize)]
struct BodyPoint {
    side: BodySide,
    part: BodyPart,
}

#[derive(Deserialize)]
enum BodySide {
    Left,
    Right,
}

#[derive(Deserialize)]
enum BodyPart {
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
