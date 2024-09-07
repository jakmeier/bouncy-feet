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
    #[wasm_bindgen(js_name = poseMatches)]
    pub pose_matches: u32,
    #[wasm_bindgen(js_name = poseMisses)]
    pub pose_misses: u32,
}

#[derive(Debug, Clone, Copy)]
#[wasm_bindgen]
pub enum DetectionFailureReason {
    /// The last match was too recent to have another match.
    TooEarly = 1,
    /// The timing is off.
    NotOnBeat = 2,
    /// Detection did not match an expected pose.
    WrongPose = 3,
    /// No data to run detection against.
    NoData = 4,
    /// Currently in a state that does not detect.
    DetectionDisabled = 5,
    /// No *new* data to run detection against.
    NoNewData = 6,
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

    #[wasm_bindgen(js_name = poseHint)]
    pub fn pose_hint(&self) -> PoseHint {
        match &self.last_error {
            Some((hint, _err)) => *hint,
            None => PoseHint::DontKnow,
        }
    }

    #[wasm_bindgen(js_name = poseError)]
    pub fn pose_error(&self) -> Option<PoseApproximation> {
        self.last_error.as_ref().map(|(_hint, err)| err.clone())
    }
}

impl DetectionResult {
    pub(crate) fn new(steps: Vec<DetectedStep>) -> Self {
        Self {
            steps,
            partial: None,
            failure_reason: None,
            last_error: None,
            pose_matches: 0,
            pose_misses: 0,
        }
    }

    pub(crate) fn with_failure_reason(mut self, reason: DetectionFailureReason) -> Self {
        self.failure_reason = Some(reason);
        self
    }
}
