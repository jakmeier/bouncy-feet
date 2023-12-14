use crate::intern::step::Step;
use wasm_bindgen::prelude::wasm_bindgen;

/// Information about a step for display in the frontend.
#[derive(Debug)]
#[wasm_bindgen]
pub struct StepInfo {
    name: String,
    // TODO: other fields
}

#[wasm_bindgen]
impl StepInfo {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl From<&Step> for StepInfo {
    fn from(value: &Step) -> Self {
        Self {
            name: value.name.clone(),
        }
    }
}
