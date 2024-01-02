//! Defines the external format for defining dances, which are a sequence of
//! steps.
//!
//! Best practice: Don't use any of the type of this file outside of parsing
//! logic. Instead, translate to internal types. This allows refactoring
//! internal without changing the external formats.

use crate::pose_file::ParseFileError;
use serde::{Deserialize, Serialize};

const CURRENT_VERSION: u16 = 0;

/// Format for dance definition files.
#[derive(Deserialize)]
pub(crate) struct DanceFile {
    pub version: u16,
    pub dances: Vec<Dance>,
}

/// Description of a dance.
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Dance {
    pub id: String,
    pub steps: Vec<String>,
}

impl DanceFile {
    pub(crate) fn from_str(text: &str) -> Result<Self, ParseFileError> {
        let parsed: DanceFile = ron::from_str(text)?;
        if parsed.version != CURRENT_VERSION {
            return Err(ParseFileError::VersionMismatch {
                expected: CURRENT_VERSION,
                found: parsed.version,
            });
        }
        Ok(parsed)
    }
}
