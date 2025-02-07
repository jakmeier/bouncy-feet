use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::intern::content_collection::ContentCollection;
use crate::skeleton::Cartesian2d;
use crate::{dance_file, intern, DanceInfo, STATE};

use super::step_wrapper::StepWrapper;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct DanceWrapper {
    /// The source of truth. Modification must only go here first and then
    /// propagate to the other fields.
    dance_definition: Rc<dance_file::Dance>,
    /// Associated collection
    ///
    /// TODO: get rid of me
    db: Rc<ContentCollection>,

    /// A view on the step for UI display.
    cached_info: Rc<RefCell<Option<DanceInfo>>>,
}

impl DanceWrapper {
    pub(crate) fn new(definition: dance_file::Dance, db: impl Into<Rc<ContentCollection>>) -> Self {
        Self {
            dance_definition: Rc::new(definition),
            cached_info: Rc::default(),
            db: db.into(),
        }
    }

    pub(crate) fn new_on_global_collection(definition: dance_file::Dance) -> Self {
        STATE.with_borrow(|state| {
            // FIXME: This clone is inefficient and fragile to modifications.
            let cloned_collection = ContentCollection::clone(&state.global_db);
            Self::new(definition, cloned_collection)
        })
    }

    // this would be needed if modifications are allowed
    // fn invalidate_cache(&mut self) {
    //     self.cached_info = None;
    // }

    pub(crate) fn definition(&self) -> &dance_file::Dance {
        &self.dance_definition
    }

    pub(crate) fn info(&self) -> DanceInfo {
        if self.cached_info.as_ref().borrow().is_none() {
            let dance: dance_file::Dance = self.dance_definition.as_ref().clone();
            let dance = DanceInfo::from(&intern::dance::Dance::from(dance));
            *self.cached_info.borrow_mut() = Some(dance);
        }
        self.cached_info.as_ref().borrow().as_ref().unwrap().clone()
    }
}

#[wasm_bindgen]
impl DanceWrapper {
    /// The unique identifier for the dance.
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.definition().id.clone()
    }

    pub fn length(&self) -> usize {
        self.definition().steps.len()
    }

    pub fn steps(&self) -> Vec<StepWrapper> {
        self.dance_definition
            .steps
            .iter()
            .map(|step| self.db.step(&step.id).cloned().unwrap())
            .collect()
    }

    // TODO: probably should return skeleton wrapper
    pub fn skeleton(&self, beat: usize) -> Option<crate::skeleton::Skeleton> {
        self.info().skeleton(beat)
    }

    /// The number of subbeats the dance takes for one repetition.
    #[wasm_bindgen(getter)]
    pub fn subbeats(&self) -> usize {
        self.info().total_subbeats
    }

    /// How much the body position deviates from the origin.
    #[wasm_bindgen(js_name = "bodyShift")]
    pub fn body_shift(&self, beat: usize) -> Cartesian2d {
        self.info().body_shift(beat)
    }
}
