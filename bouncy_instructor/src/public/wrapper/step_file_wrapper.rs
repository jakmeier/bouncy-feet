use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use crate::editor::ExportError;
use crate::intern::step::StepSource;
use crate::intern::tracker_dance_collection::TrackerDanceCollection;
use crate::step_file::StepFile;
use crate::wrapper::step_wrapper::StepWrapper;
use crate::STATE;

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct StepFileWrapper {
    step_file: Rc<RefCell<StepFile>>,
    // cache must always be kept in sync
    steps_cache: Rc<RefCell<Vec<StepWrapper>>>,
}

#[wasm_bindgen]
impl StepFileWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new_empty() -> Self {
        Self::new_cold_lab_step_file(StepFile::new())
    }

    #[wasm_bindgen(js_name = "fromRon")]
    /// FIXME: This adds steps as lab steps and then calls a warm up. This is to
    /// avoid the problem where a step wrapper can only be created for steps
    /// that are already registered in global state. A proper refactoring should
    /// solve this.
    pub fn from_ron(text: &str) -> Result<StepFileWrapper, JsValue> {
        let file = StepFile::from_str(text)?;
        let file_wrapper = Self::new_cold_lab_step_file(file);

        STATE.with_borrow_mut(|state| {
            state
                .global_db
                .replace_steps(StepSource::new("lab".to_owned()), file_wrapper.steps());
            file_wrapper.warm_up(&state.global_db.tracker_view);
        });
        Ok(file_wrapper)
    }

    pub fn steps(&self) -> Vec<StepWrapper> {
        self.steps_cache.as_ref().borrow().clone()
    }

    #[wasm_bindgen(js_name = "addStep")]
    pub fn add_step(&mut self, new_step: &StepWrapper) -> Result<(), String> {
        let file = self.step_file.as_ref().borrow();
        if file
            .steps
            .iter()
            .any(|step| new_step.definition().id == step.id)
        {
            return Err("Step ID already exists".to_owned());
        }
        drop(file);
        self.push_step_internal(new_step.clone());
        Ok(())
    }

    #[wasm_bindgen(js_name = "overwriteStep")]
    pub fn overwrite_step(&mut self, new_step: &StepWrapper) -> Result<(), String> {
        let file = self.step_file.as_ref().borrow();

        if let Some(index) = file
            .steps
            .iter()
            .position(|step| step.id == new_step.definition().id)
        {
            drop(file);
            self.set_step(index, new_step.clone());
            Ok(())
        } else {
            Err("Step ID does not exists".to_owned())
        }
    }

    #[wasm_bindgen(js_name = "removeStep")]
    pub fn remove_step(&mut self, id: String) -> Result<(), String> {
        let file = self.step_file.as_ref().borrow();
        if let Some(index) = file.steps.iter().position(|step| step.id == id) {
            drop(file);
            self.remove_step_internal(index);
            Ok(())
        } else {
            Err("Step ID does not exists".to_owned())
        }
    }

    #[wasm_bindgen(js_name = "buildRon")]
    pub fn build_ron(&self) -> Result<String, ExportError> {
        let file_data = self.step_file.as_ref().borrow();
        let string = ron::ser::to_string(&*file_data)?;
        Ok(string)
    }
}

impl StepFileWrapper {
    /// Creates a step file for the lab, with steps that need to be warmed up
    /// before using them.
    fn new_cold_lab_step_file(file: StepFile) -> Self {
        let source = StepSource::new("lab".to_owned());
        let steps = file
            .steps
            .iter()
            .cloned()
            .map(|def| StepWrapper::new_cold(def, source.clone()))
            .collect();
        Self {
            step_file: Rc::new(RefCell::new(file)),
            steps_cache: Rc::new(RefCell::new(steps)),
        }
    }

    fn warm_up(&self, db: &TrackerDanceCollection) {
        for step in self.steps_cache.borrow_mut().iter_mut() {
            step.warm_up(db);
        }
    }

    fn push_step_internal(&self, step: StepWrapper) {
        self.step_file
            .borrow_mut()
            .steps
            .push(step.definition().clone());
        self.steps_cache.borrow_mut().push(step);
    }

    fn remove_step_internal(&self, index: usize) {
        self.steps_cache.borrow_mut().remove(index);
        self.step_file.borrow_mut().steps.remove(index);
    }

    fn set_step(&self, index: usize, step: StepWrapper) {
        self.step_file.borrow_mut().steps[index] = step.definition().clone();
        self.steps_cache.borrow_mut()[index] = step;
    }
}
