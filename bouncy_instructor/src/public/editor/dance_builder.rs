use crate::{intern, DanceInfo};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]

pub struct DanceBuilder {
    pub(crate) id: String,
    step_ids: Vec<String>,
}

#[wasm_bindgen]
impl DanceBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new(id: String) -> Self {
        Self {
            id,
            step_ids: vec![],
        }
    }

    pub fn length(&self) -> usize {
        self.step_ids.len()
    }

    #[wasm_bindgen(js_name = "addStep")]
    pub fn add_step(&mut self, step_id: String) {
        self.step_ids.push(step_id);
    }

    #[wasm_bindgen(js_name = "setId")]
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn clear(&mut self) {
        self.step_ids.clear();
    }

    #[wasm_bindgen(js_name = "danceInfo")]
    pub fn dance_info(&self) -> DanceInfo {
        (&self.build()).into()
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

impl From<&DanceInfo> for DanceBuilder {
    fn from(dance: &DanceInfo) -> Self {
        Self {
            id: dance.id(),
            step_ids: dance.steps.iter().map(|step| step.id()).collect(),
        }
    }
}
