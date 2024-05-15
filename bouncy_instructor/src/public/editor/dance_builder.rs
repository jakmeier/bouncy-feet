use crate::{intern, DanceInfo};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]

pub struct DanceBuilder {
    pub(crate) id: String,
    step_ids: Vec<String>,
    flip_orientation: Vec<bool>,
}

#[wasm_bindgen]
impl DanceBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new(id: String) -> Self {
        Self {
            id,
            step_ids: vec![],
            flip_orientation: vec![],
        }
    }

    pub fn length(&self) -> usize {
        self.step_ids.len()
    }

    #[wasm_bindgen(js_name = "setId")]
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    #[wasm_bindgen(js_name = "addStep")]
    pub fn add_step(&mut self, step_id: String) {
        self.step_ids.push(step_id);
        self.flip_orientation.push(false);
    }

    #[wasm_bindgen(js_name = "removeStep")]
    pub fn remove_step(&mut self, pos: usize) -> Result<String, String> {
        if pos >= self.step_ids.len() {
            return Err(format!("step index {pos} out of bound"));
        }
        let step = self.step_ids.remove(pos);
        self.flip_orientation.remove(pos);
        Ok(step)
    }

    #[wasm_bindgen(js_name = "insertStep")]
    pub fn insert_step(&mut self, pos: usize, step_id: String) -> Result<(), String> {
        if pos > self.step_ids.len() {
            return Err(format!("step index {pos} out of bound"));
        }
        self.step_ids.insert(pos, step_id);
        self.flip_orientation.insert(pos, false);
        Ok(())
    }

    #[wasm_bindgen(js_name = "setOrientation")]
    pub fn set_orientation(&mut self, pos: usize, flipped: bool) -> Result<(), String> {
        if pos >= self.flip_orientation.len() {
            return Err(format!("step index {pos} out of bound"));
        }
        self.flip_orientation[pos] = flipped;
        Ok(())
    }

    #[wasm_bindgen(js_name = "isFlipped")]
    pub fn is_flipped(&mut self, pos: usize) -> Result<bool, String> {
        if pos >= self.flip_orientation.len() {
            return Err(format!("step index {pos} out of bound"));
        }
        Ok(self.flip_orientation[pos])
    }

    pub fn clear(&mut self) {
        self.step_ids.clear();
        self.flip_orientation.clear();
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
            flip_orientation: self.flip_orientation.clone(),
        }
    }
}

impl From<intern::dance::Dance> for DanceBuilder {
    fn from(dance: intern::dance::Dance) -> Self {
        Self {
            id: dance.id,
            step_ids: dance.step_ids,
            flip_orientation: dance.flip_orientation,
        }
    }
}
