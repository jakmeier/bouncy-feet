use crate::intern::step::StepSource;
use crate::intern::tracker_dance_collection::TrackerDanceCollection;
use crate::skeleton::{Cartesian2d, Skeleton};
use crate::{intern, step_file, StepInfo};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct StepWrapper {
    /// The source of truth. Modification must only go here first and then
    /// propagate to the other fields.
    step_definition: Rc<step_file::Step>,
    source: StepSource,
    /// Flip the underlying step, usually defined in a parent dance.
    flipped: bool,

    /// A view on the step for UI display.
    cached_info: Rc<RefCell<Option<StepInfo>>>,
    cached_intern: Rc<RefCell<Option<intern::step::Step>>>,
}

impl StepWrapper {
    pub(crate) fn new(definition: step_file::Step, source: StepSource) -> Self {
        Self {
            step_definition: Rc::new(definition),
            source,
            flipped: false,
            cached_info: Rc::default(),
            cached_intern: Rc::default(),
        }
    }

    pub(crate) fn warm_up(self, db: &TrackerDanceCollection) -> Self {
        let step = db.step(&self.definition().id).cloned().unwrap();
        *self.cached_intern.borrow_mut() = Some(step.clone());
        *self.cached_info.borrow_mut() = Some(StepInfo::from_step(step, db));
        self
    }

    // this would be needed if modifications are allowed
    // fn invalidate_cache(&mut self) {
    //     self.cached_info = Rc::default();
    //     self.cached_intern = Rc::default();
    // }

    pub(crate) fn definition(&self) -> &step_file::Step {
        &self.step_definition
    }

    pub(crate) fn source(&self) -> &StepSource {
        &self.source
    }

    pub(crate) fn step(&self, db: &TrackerDanceCollection) -> intern::step::Step {
        if self.cached_intern.borrow().is_none() {
            let mut step = db.step(&self.definition().id).cloned().unwrap();
            if self.flipped {
                step = step.flipped();
            }
            *self.cached_intern.borrow_mut() = Some(step);
        }
        self.cached_intern.borrow().as_ref().unwrap().clone()
    }

    pub(crate) fn info(&self, db: &TrackerDanceCollection) -> StepInfo {
        if self.cached_info.borrow().is_none() {
            let step = self.step(db).clone();
            *self.cached_info.borrow_mut() = Some(StepInfo::from_step(step, db));
        }
        self.cached_info.borrow().as_ref().unwrap().clone()
    }

    /// Assume the step info has been initialized and read it. This allows
    /// access without passing around the collection.
    ///
    ///  TODO: get rid of me, there's gotta be a less ugly solution
    fn info_unchecked(&self) -> StepInfo {
        self.cached_info.borrow().as_ref().unwrap().clone()
    }

    pub(crate) fn flipped(&self, db: &TrackerDanceCollection) -> StepWrapper {
        let definition = self.definition().clone();
        let source = self.source.clone();
        let mut wrapper = StepWrapper::new(definition, source);
        wrapper.flipped = !self.flipped;
        wrapper.warm_up(db)
    }
}

#[wasm_bindgen]
impl StepWrapper {
    /// The unique identifier for the step.
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.step_definition.id.clone()
    }

    /// The descriptive name for the step. The same name is used for variations
    /// of the same step.
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.definition().name.clone()
    }

    pub fn skeleton(&self, beat: usize) -> Skeleton {
        self.info_unchecked().skeleton(beat)
    }

    /// How much the body position deviates from the origin.
    #[wasm_bindgen(js_name = "bodyShift")]
    pub fn body_shift(&self, beat: usize) -> Cartesian2d {
        self.info_unchecked().body_shift(beat)
    }

    /// Applies a rotation (in degree) and returns the resulting skelton.
    #[wasm_bindgen(js_name = "rotatedSkeleton")]
    pub fn rotated_skeleton(&self, beat: usize, rotation: f32) -> Skeleton {
        self.info_unchecked().rotated_skeleton(beat, rotation)
    }

    #[wasm_bindgen(js_name = "jumpHeight")]
    pub fn jump_height(&self, beat: usize) -> Option<f32> {
        self.info_unchecked().jump_height(beat)
    }

    /// Description identifier for the translated text which describes how the
    /// variation is different from the original.
    ///
    /// For example: "left-first" can be used for all steps which are the same
    /// as the original but instead of starting with the right foot, it starts
    /// with the left foot first. The app shows a translated text like "Left Leg First".
    #[wasm_bindgen(getter)]
    pub fn variation(&self) -> Option<String> {
        self.info_unchecked().variation()
    }

    /// The number of beats the step takes for one repetition.
    #[wasm_bindgen(getter)]
    pub fn beats(&self) -> usize {
        self.info_unchecked().beats()
    }
}
