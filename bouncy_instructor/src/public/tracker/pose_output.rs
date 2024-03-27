use super::Timestamp;
use crate::intern::pose::BodyPoint;
use crate::intern::pose_score::ErrorDetails;
use crate::STATE;
use wasm_bindgen::prelude::wasm_bindgen;

/// The result of fitting keypoints to poses.
#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct PoseApproximation {
    /// ID defined in pose file
    pub(crate) name: String,
    /// Total error between 0.0 and 1.0.
    pub error: f32,
    /// Timestamp for which Keypoints were added
    pub timestamp: Timestamp,
    pub(crate) error_details: ErrorDetails,
}

/// Self-describing error score for a specific limb
#[wasm_bindgen]
pub struct LimbError {
    name: String,
    pub error: f32,
    pub weight: f32,
}

#[wasm_bindgen]
pub struct ZError {
    body_point: BodyPoint,
    pub error: f32,
    pub quadrant_error: bool,
}

#[wasm_bindgen]
pub struct ZWrongOrderError {
    expected_forward: BodyPoint,
    expected_backward: BodyPoint,
}

#[wasm_bindgen]
impl PoseApproximation {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// List all limbs, order by how well they fit, best fit first.
    #[wasm_bindgen(js_name = limbErrors)]
    pub fn limb_errors(&self) -> Vec<LimbError> {
        let increasing = true;
        let weighted = false;
        self.sorted_limb_errors_impl(increasing, weighted).collect()
    }

    #[wasm_bindgen(js_name = zErrors)]
    pub fn z_errors(&self) -> Vec<ZError> {
        self.error_details
            .z_absolute_errors
            .iter()
            .zip(&self.error_details.quadrant_errors)
            .zip(&self.error_details.body_points)
            .map(|((&error, &quadrant_error), &body_point)| ZError {
                body_point,
                error,
                quadrant_error,
            })
            .collect()
    }

    #[wasm_bindgen(js_name = zOrderErrors)]
    pub fn z_order_errors(&self) -> Vec<ZWrongOrderError> {
        self.error_details
            .z_order_errors
            .iter()
            .map(|expected| ZWrongOrderError {
                expected_backward: expected.backward,
                expected_forward: expected.forward,
            })
            .collect()
    }

    /// List the `n` limbs with the highest error contribution to the pose error.
    #[wasm_bindgen(js_name = worstLimbs)]
    pub fn worst_limbs(&self, n: usize) -> Vec<LimbError> {
        let increasing = false;
        let weighted = true;
        self.sorted_limb_errors_impl(increasing, weighted)
            .take(n)
            .collect()
    }

    fn sorted_limb_errors_impl(
        &self,
        increasing: bool,
        weighted: bool,
    ) -> impl Iterator<Item = LimbError> + '_ {
        self.error_details
            .sorted_by_error(increasing, weighted)
            .into_iter()
            .map(|i| LimbError {
                name: STATE.with_borrow(|state| {
                    state.db.limb_name(self.error_details.limbs[i]).to_owned()
                }),
                error: self.error_details.errors[i],
                weight: self.error_details.weights[i],
            })
    }
}

#[wasm_bindgen]
impl LimbError {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

#[wasm_bindgen]
impl ZError {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        format!("{:?}", self.body_point)
    }
}

#[wasm_bindgen]
impl ZWrongOrderError {
    #[wasm_bindgen(getter)]
    pub fn expected(&self) -> String {
        format!(
            "{:?} behind {:?}",
            self.expected_backward, self.expected_forward
        )
    }
}
