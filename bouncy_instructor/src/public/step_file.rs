//! Defines the external format for defining steps, which are a combination of
//! poses.
//!
//! Best practice: Don't use any of the type of this file outside of parsing
//! logic. Instead, translate to internal types. This allows refactoring
//! internal without changing the external formats.

use crate::pose_file::ParseFileError;
use serde::{Deserialize, Serialize};

const CURRENT_VERSION: u16 = 0;

/// Format for pose definition files.
#[derive(Deserialize)]
pub(crate) struct StepFile {
    pub version: u16,
    pub steps: Vec<Step>,
}

/// Description of a step.
///
/// A step is a sequence of poses with timing and orientation information.
/// This is the format for external files and loaded in at runtime.
/// It is converted to a [`crate::step::Step`] for step detection.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub(crate) struct Step {
    /// The identifier for the step. The same ID is used for variations of the same step.
    pub name: String,
    /// Description identifier for the translated text which describes how the
    /// variation is different from the original.
    ///
    /// For example: "left-first" can be used for all steps which are the same
    /// as the original but instead of starting with the right foot, it starts
    /// with the left foot first. The app shows a translated text like "Left Leg First".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variation: Option<String>,
    pub keyframes: Vec<StepPosition>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub(crate) struct StepPosition {
    /// Reference to the name of a pose
    pub pose: String,
    /// specify how the pose should be oriented
    #[serde(default, skip_serializing_if = "Orientation::any")]
    pub orientation: Orientation,
}

/// Define in which direction a pose should be oriented.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub(crate) enum Orientation {
    ToCamera,
    Right,
    Away,
    Left,
    /// It doesn't matter in which direction the pose is done.
    Any,
}

impl StepFile {
    pub(crate) fn from_str(text: &str) -> Result<Self, ParseFileError> {
        let parsed: StepFile = ron::from_str(text)?;
        if parsed.version != CURRENT_VERSION {
            return Err(ParseFileError::VersionMismatch {
                expected: CURRENT_VERSION,
                found: parsed.version,
            });
        }
        Ok(parsed)
    }
}

impl Orientation {
    fn any(&self) -> bool {
        matches!(self, Orientation::Any)
    }
}

impl Default for Orientation {
    fn default() -> Self {
        Self::Any
    }
}
