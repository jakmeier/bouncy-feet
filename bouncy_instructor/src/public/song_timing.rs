use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

/// Per-activity deviations from a song's drop metadata.
///
/// Timestamps are milliseconds in the song timeline. Drop IDs are resolved by
/// the frontend against the song catalog, which owns the audio metadata.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[wasm_bindgen]
pub struct SongTimingOverride {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) playback_start_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) count_in_end_drop: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) count_in_end_ms: Option<u64>,
}

impl SongTimingOverride {
    pub(crate) fn validate(&self) -> Result<(), SongTimingOverrideError> {
        if self.count_in_end_drop.is_some() && self.count_in_end_ms.is_some() {
            return Err(SongTimingOverrideError::ConflictingCountInEnd);
        }
        if self
            .count_in_end_drop
            .as_ref()
            .is_some_and(|drop| drop.trim().is_empty())
        {
            return Err(SongTimingOverrideError::EmptyDropId);
        }
        Ok(())
    }
}

#[wasm_bindgen]
impl SongTimingOverride {
    #[wasm_bindgen(getter, js_name = playbackStartMs)]
    pub fn playback_start_ms(&self) -> Option<f64> {
        self.playback_start_ms.map(|value| value as f64)
    }

    #[wasm_bindgen(getter, js_name = countInEndDrop)]
    pub fn count_in_end_drop(&self) -> Option<String> {
        self.count_in_end_drop.clone()
    }

    #[wasm_bindgen(getter, js_name = countInEndMs)]
    pub fn count_in_end_ms(&self) -> Option<f64> {
        self.count_in_end_ms.map(|value| value as f64)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum SongTimingOverrideError {
    ConflictingCountInEnd,
    EmptyDropId,
}

#[cfg(test)]
mod tests {
    use super::{SongTimingOverride, SongTimingOverrideError};

    #[test]
    fn parses_and_validates_a_timing_override() {
        let timing: SongTimingOverride = ron::from_str(
            r#"#![enable(implicit_some)] (playback_start_ms: 2000, count_in_end_drop: "second")"#,
        )
        .unwrap();
        assert_eq!(timing.playback_start_ms, Some(2000));
        assert_eq!(timing.count_in_end_drop.as_deref(), Some("second"));
        assert_eq!(timing.count_in_end_ms, None);
        assert!(timing.validate().is_ok());
    }

    #[test]
    fn rejects_conflicting_count_in_end_fields() {
        let timing: SongTimingOverride = ron::from_str(
            r#"#![enable(implicit_some)] (count_in_end_drop: "first", count_in_end_ms: 1000)"#,
        )
        .unwrap();
        assert!(matches!(
            timing.validate(),
            Err(SongTimingOverrideError::ConflictingCountInEnd)
        ));
    }
}
