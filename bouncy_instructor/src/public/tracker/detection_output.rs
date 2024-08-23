use crate::StepInfo;

use super::DetectedStep;
use wasm_bindgen::prelude::wasm_bindgen;

/// Result of a step or dance detection.
///
/// A detection potentially includes a list of steps. It can be displayed in the
/// frontend as is, or provided to a tracker to update the detection after more
/// data has been added.
#[derive(Debug, Default, Clone)]
#[wasm_bindgen]
pub struct DetectionResult {
    /// fully detected steps
    pub(crate) steps: Vec<DetectedStep>,
    /// partially detected step
    pub(crate) partial: Option<DetectedStep>,
    /// The expected step for unique step tracking
    pub(crate) target_step: Option<StepInfo>,
    /// If the newest detection was negative, this fields contains information
    /// about the reason.
    pub(crate) failure_reason: Option<DetectionFailureReason>,
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub enum DetectionFailureReason {
    TooEarly = 1,
    NotOnBeat = 2,
    WrongPose = 3,
}

#[wasm_bindgen]
impl DetectionResult {
    #[wasm_bindgen(constructor)]
    pub fn new_default() -> Self {
        Self::default()
    }
    pub fn steps(&self) -> Vec<DetectedStep> {
        self.steps.clone()
    }
}

impl DetectionResult {
    pub(crate) fn new(steps: Vec<DetectedStep>) -> Self {
        Self {
            steps,
            partial: None,
            target_step: None,
            failure_reason: None,
        }
    }

    pub(crate) fn init_for_unique_step_tracker(target_step: StepInfo) -> Self {
        Self {
            steps: vec![],
            partial: None,
            target_step: Some(target_step),
            failure_reason: None,
        }
    }

    pub(crate) fn with_failure_reason(mut self, reason: DetectionFailureReason) -> Self {
        self.failure_reason = Some(reason);
        self
    }
}
