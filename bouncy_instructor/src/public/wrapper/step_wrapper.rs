use std::rc::Rc;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::intern::step::StepSource;
use crate::step_file;

#[derive(Debug)]
#[wasm_bindgen]
pub struct StepWrapper {
    /// The source of truth. Modification must only go here first and then
    /// propagate to the other fields.
    step_definition: Rc<step_file::Step>,
    source: StepSource,
}

impl StepWrapper {
    pub(crate) fn new(definition: step_file::Step, source: StepSource) -> Self {
        Self {
            step_definition: Rc::new(definition),
            source,
        }
    }

    pub(crate) fn definition(&self) -> &step_file::Step {
        &self.step_definition
    }

    pub(crate) fn source(&self) -> &StepSource {
        &self.source
    }
}
