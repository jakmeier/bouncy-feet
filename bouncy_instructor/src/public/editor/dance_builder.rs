use crate::dance_file::DanceStep;
use crate::intern::content_collection::ContentCollection;
use crate::wrapper::dance_wrapper::DanceWrapper;
use crate::{dance_file, intern, STATE};
use std::rc::Rc;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct DanceBuilder {
    pub(crate) id: String,
    step_ids: Vec<String>,
    flip_orientation: Vec<bool>,
    // hack to keep a copy around for easier creation of step wrappers, which
    // unfortunately contain a collection rc
    pub(crate) global_collection: Rc<ContentCollection>,
}

#[wasm_bindgen]
impl DanceBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new(id: String) -> Self {
        let global_collection = STATE.with_borrow(|state| {
            // FIXME: This clone is inefficient and fragile to modifications.
            Rc::new(state.global_db.clone())
        });
        Self {
            id,
            step_ids: vec![],
            flip_orientation: vec![],
            global_collection,
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
    pub fn as_dance(&self) -> DanceWrapper {
        // Dance builder currently are only supported on the global collection.
        DanceWrapper::new_on_global_collection(self.build())
    }
}
impl DanceBuilder {
    pub(crate) fn build(&self) -> dance_file::Dance {
        let steps = self
            .step_ids
            .iter()
            .zip(&self.flip_orientation)
            .map(|(id, &flip_orientation)| DanceStep {
                id: id.clone(),
                flip_orientation,
            })
            .collect();
        dance_file::Dance {
            id: self.id.clone(),
            steps,
        }
    }
}

impl From<intern::dance::Dance> for DanceBuilder {
    fn from(dance: intern::dance::Dance) -> Self {
        let global_collection = STATE.with_borrow(|state| {
            // FIXME: This clone is inefficient and fragile to modifications.
            Rc::new(state.global_db.clone())
        });
        Self {
            id: dance.id,
            step_ids: dance.step_ids,
            flip_orientation: dance.flip_orientation,
            global_collection,
        }
    }
}

impl From<dance_file::Dance> for DanceBuilder {
    fn from(definition: dance_file::Dance) -> Self {
        let (step_ids, flip_orientation) = definition
            .steps
            .into_iter()
            .map(|s| (s.id, s.flip_orientation))
            .unzip();
        let global_collection = STATE.with_borrow(|state| {
            // FIXME: This clone is inefficient and fragile to modifications.
            Rc::new(state.global_db.clone())
        });
        Self {
            id: definition.id,
            step_ids,
            flip_orientation,
            global_collection,
        }
    }
}

impl From<DanceWrapper> for DanceBuilder {
    fn from(wrapper: DanceWrapper) -> Self {
        wrapper.definition().clone().into()
    }
}
