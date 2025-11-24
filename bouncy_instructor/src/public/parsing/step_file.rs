//! Defines the external format for defining steps, which are a combination of
//! poses.

use crate::parsing::ParseFileError;
use crate::pose_file::BodyPoint;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

const CURRENT_VERSION: u16 = 0;

/// Format for pose definition files.
#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct StepFile {
    pub version: u16,
    pub steps: Vec<Step>,
}

/// Description of a step.
///
/// A step is a sequence of poses with timing and orientation information.
/// This is the format for external files and loaded in at runtime.
/// It is converted to a [`crate::intern::step::Step`] for step detection.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub(crate) struct Step {
    /// The unique identifier for the step.
    pub id: String,
    /// The descriptive name for the step. The same name is used for variations
    /// of the same step. This name can also be shown to users if no translation
    /// is available.
    pub name: String,
    /// Description identifier for the translated text which describes how the
    /// variation is different from the original.
    ///
    /// For example: "left-first" can be used for all steps which are the same
    /// as the original but instead of starting with the right foot, it starts
    /// with the left foot first. The app shows a translated text like "Left Leg First".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variation: Option<String>,
    /// Poses per beat.
    pub keyframes: Vec<StepPosition>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub(crate) struct StepPosition {
    /// Reference to the name of a pose
    pub pose: String,
    /// specify how the pose should be oriented
    #[serde(default, skip_serializing_if = "Orientation::any")]
    pub orientation: Orientation,
    /// Which body part shall remain fixed, while the rest of the body moves.
    ///
    /// The pivot defines defines how to transition into this pose. On the first
    /// pose of a step, this is relevant for looping.
    #[serde(
        default = "BodyPoint::default_pivot",
        skip_serializing_if = "BodyPoint::is_default_pivot"
    )]
    pub pivot: BodyPoint,
    /// How high to jump when transitioning into this step, relative to the full
    /// body size.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jump_height: Option<f32>,
}

/// Define in which direction a pose should be oriented.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, Default)]
#[wasm_bindgen]
pub enum Orientation {
    ToCamera,
    Right,
    Away,
    Left,
    /// It doesn't matter in which direction the pose is done.
    #[default]
    Any,
}

impl StepFile {
    pub(crate) fn new() -> Self {
        Self {
            version: CURRENT_VERSION,
            steps: vec![],
        }
    }

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

impl Step {
    pub fn new_empty(id: String, name: String) -> Self {
        Self {
            id,
            name,
            variation: None,
            keyframes: vec![],
        }
    }
}

impl Orientation {
    fn any(&self) -> bool {
        matches!(self, Orientation::Any)
    }
}
