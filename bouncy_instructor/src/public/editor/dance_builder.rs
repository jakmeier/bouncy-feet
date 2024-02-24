use crate::intern;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]

pub struct DanceBuilder {
    id: String,
    step_ids: Vec<String>,
}

#[wasm_bindgen]
impl DanceBuilder {
    pub fn new(id: String) -> Self {
        Self {
            id,
            step_ids: vec![],
        }
    }

    #[wasm_bindgen(js_name = "addStep")]
    pub fn add_step(&mut self, step_id: String) {
        self.step_ids.push(step_id);
    }
}
impl DanceBuilder {
    pub(crate) fn build(&self) -> intern::dance::Dance {
        intern::dance::Dance {
            id: self.id.clone(),
            step_ids: self.step_ids.clone(),
        }
    }
}
