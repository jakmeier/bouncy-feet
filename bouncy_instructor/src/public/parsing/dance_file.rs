//! Defines the external format for defining dances, which are a sequence of
//! steps.

use super::VersionCheck;
use crate::parsing::ParseFileError;
use serde::{Deserialize, Serialize};

pub(crate) const CURRENT_VERSION: u16 = 1;

/// Format for dance definition files.
#[derive(Serialize, Deserialize)]
pub(crate) struct DanceFile {
    pub version: u16,
    pub dances: Vec<Dance>,
}

/// Description of a dance.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Dance {
    pub id: String,
    pub steps: Vec<DanceStep>,
}

/// Description of step inside a dance.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct DanceStep {
    pub id: String,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub flip_orientation: bool,
}

impl DanceFile {
    pub(crate) fn from_str(text: &str) -> Result<Self, ParseFileError> {
        let check: VersionCheck = ron::from_str(text)?;
        if check.version == 0 {
            let parsed: v0::DanceFile = ron::from_str(text)?;
            return Ok(parsed.into());
        }

        if check.version != CURRENT_VERSION {
            return Err(ParseFileError::VersionMismatch {
                expected: CURRENT_VERSION,
                found: check.version,
            });
        }
        let parsed: DanceFile = ron::from_str(text)?;
        Ok(parsed)
    }
}

mod v0 {
    use serde::{Deserialize, Serialize};

    use super::{DanceStep, CURRENT_VERSION};

    /// Format for dance definition files.
    #[derive(Serialize, Deserialize)]
    pub(super) struct DanceFile {
        pub version: u16,
        pub dances: Vec<Dance>,
    }

    /// Description of a dance.
    #[derive(Serialize, Deserialize, Debug)]
    pub(super) struct Dance {
        pub id: String,
        pub steps: Vec<String>,
    }

    impl From<DanceFile> for super::DanceFile {
        fn from(v0: DanceFile) -> Self {
            super::DanceFile {
                version: CURRENT_VERSION,
                dances: v0
                    .dances
                    .into_iter()
                    .map(|dance| super::Dance {
                        id: dance.id,
                        steps: dance
                            .steps
                            .into_iter()
                            .map(|id| DanceStep {
                                id,
                                flip_orientation: false,
                            })
                            .collect(),
                    })
                    .collect(),
            }
        }
    }
}
