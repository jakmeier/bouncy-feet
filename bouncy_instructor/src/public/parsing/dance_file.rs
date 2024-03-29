//! Defines the external format for defining dances, which are a sequence of
//! steps.

use crate::parsing::ParseFileError;
use serde::{Deserialize, Serialize};

pub(crate) const CURRENT_VERSION: u16 = 0;

/// Format for dance definition files.
#[derive(Serialize, Deserialize)]
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
