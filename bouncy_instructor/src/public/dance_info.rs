use crate::intern::dance::Dance;
use wasm_bindgen::prelude::wasm_bindgen;

/// Information about a dance for display in the frontend.
#[derive(Debug)]
#[wasm_bindgen]
pub struct DanceInfo {
    id: String,
    // TODO: add more fields as appropriate
}

#[wasm_bindgen]
impl DanceInfo {
    /// The unique identifier for the dance.
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.id.clone()
    }
}

impl From<&Dance> for DanceInfo {
    fn from(dance: &Dance) -> Self {
        Self {
            id: dance.id.clone(),
        }
    }
}
