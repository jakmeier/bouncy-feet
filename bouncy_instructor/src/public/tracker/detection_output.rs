use super::{DetectedStep, PoseApproximation};
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
    /// If the newest detection was negative, this fields contains information
    /// about the reason.
    #[wasm_bindgen(js_name = "failureReason")]
    pub failure_reason: Option<DetectionFailureReason>,
    /// This contains computed details about the mismatched pose, if the latest
    /// detection failed because of it.
    pub(crate) last_error: Option<(PoseHint, PoseApproximation)>,
}

#[derive(Debug, Clone, Copy)]
#[wasm_bindgen]
pub enum DetectionFailureReason {
    TooEarly = 1,
    NotOnBeat = 2,
    WrongPose = 3,
    NoData = 4,
}

/// Best guess for what the dancer needs to change to fit the pose.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub enum PoseHint {
    DontKnow,
    LeftRight,
    ZOrder,
    WrongDirection,
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
            failure_reason: None,
            last_error: None,
        }
    }

    pub(crate) fn with_failure_reason(mut self, reason: DetectionFailureReason) -> Self {
        self.failure_reason = Some(reason);
        self
    }
}
