use super::pose_wrapper::PoseWrapper;
use crate::editor::step_position_builder::StepPositionBuilder;
use crate::intern::content_collection::ContentCollection;
use crate::intern::step::StepSource;
use crate::intern::tracker_dance_collection::TrackerDanceCollection;
use crate::skeleton::{Cartesian2d, Skeleton};
use crate::{intern, step_file, StepInfo, STATE};
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

    // cached fields are updated lazily
    /// A view on the step for UI display.
    cached_info: Rc<RefCell<Option<StepInfo>>>,
    cached_intern: Rc<RefCell<Option<intern::step::Step>>>,
    cached_poses: Rc<RefCell<Option<Vec<PoseWrapper>>>>,
}

impl StepWrapper {
    // Create a step wrapper that needs to be warmed up before using it.
    pub(crate) fn new_cold(definition: step_file::Step, source: StepSource) -> Self {
        Self {
            step_definition: Rc::new(definition),
            source,
            flipped: false,
            cached_info: Rc::default(),
            cached_intern: Rc::default(),
            cached_poses: Rc::default(),
        }
    }

    pub(crate) fn warmed_up(mut self, db: &TrackerDanceCollection) -> Self {
        self.warm_up(db);
        self
    }

    pub(crate) fn warm_up(&mut self, db: &TrackerDanceCollection) {
        let step = db
            .step(&self.definition().id)
            .cloned()
            .unwrap_or_else(|| panic!("missing step {:?}", self.definition().id));
        *self.cached_intern.borrow_mut() = Some(step.clone());
        *self.cached_info.borrow_mut() = Some(StepInfo::from_step(step, db));
    }

    fn invalidate_cache(&mut self) {
        self.cached_info = Rc::default();
        self.cached_intern = Rc::default();
        self.cached_poses = Rc::default();
    }

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
        let mut wrapper = StepWrapper::new_cold(definition, source);
        wrapper.flipped = !self.flipped;
        wrapper.warmed_up(db)
    }

    fn compute_pose_wrappers(&self, db: &ContentCollection) -> Vec<PoseWrapper> {
        self.step_definition
            .keyframes
            .iter()
            .map(|step_pose| {
                db.pose_by_id(&step_pose.pose)
                    .unwrap_or_else(|| panic!("pose {step_pose:?} not found"))
            })
            .cloned()
            .collect()
    }

    /// Recomputes caches using the global collection. Do not use for courses
    /// that require a custom collection!
    fn recompute_caches(&mut self) {
        // FIXME: Using the global collection here makes thing a bit fragile, it
        // will break if used from courses.
        self.invalidate_cache();
        STATE.with_borrow(|state| self.warm_up(&state.global_db.tracker_view))
    }
}

#[wasm_bindgen]
impl StepWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new_empty(id: String, name: String, source: String) -> Self {
        let new_step_def = step_file::Step::new_empty(id, name);
        let source = StepSource::new(source);
        let mut step = Self {
            step_definition: Rc::new(new_step_def),
            source: source.clone(),
            flipped: false,
            cached_info: Rc::default(),
            cached_intern: Rc::default(),
            cached_poses: Rc::default(),
        };
        STATE.with_borrow_mut(|state| {
            state
                .global_db
                .add_steps([step.definition().clone()], source)
                .unwrap_or_else(|err| panic!("adding step {} failed, error {err:?}", step.id()));
            step.warm_up(&state.global_db.tracker_view);
        });
        step
    }

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

    /// Look up poses from the global collection, do not use for courses that
    /// require a custom collection.
    pub fn poses(&self) -> Vec<PoseWrapper> {
        if self.cached_poses.borrow().is_none() {
            let poses = STATE.with_borrow(|state| self.compute_pose_wrappers(&state.global_db));
            *self.cached_poses.borrow_mut() = Some(poses);
        }
        self.cached_poses.borrow().as_ref().unwrap().clone()
    }

    /// Positions with poses from the global collection, do not use for courses
    /// that require a custom collection.
    pub fn positions(&self) -> Vec<StepPositionBuilder> {
        STATE
            .with_borrow(|state| {
                self.step_definition
                    .keyframes
                    .iter()
                    .map(|pose| StepPositionBuilder::try_from_step_position(pose, &state.global_db))
                    .collect::<Option<Vec<_>>>()
            })
            .unwrap()
    }

    /// Add poses from the global collection, do not use for courses that
    /// require a custom collection.
    #[wasm_bindgen(js_name = "addPosition")]
    pub fn add_position(&mut self, position: &StepPositionBuilder) {
        Rc::make_mut(&mut self.step_definition)
            .keyframes
            .push(position.into());
        self.recompute_caches();
    }

    #[wasm_bindgen(js_name = "removePosition")]
    pub fn remove_position(&mut self, index: usize) -> Result<(), String> {
        if index >= self.step_definition.keyframes.len() {
            return Err(format!("step position index {index} out of bound"));
        }
        Rc::make_mut(&mut self.step_definition)
            .keyframes
            .remove(index);
        self.recompute_caches();
        Ok(())
    }

    #[wasm_bindgen(js_name = "insertPosition")]
    pub fn insert_position(
        &mut self,
        index: usize,
        position: &StepPositionBuilder,
    ) -> Result<(), String> {
        if index >= self.step_definition.keyframes.len() {
            return Err(format!("step position index {index} out of bound"));
        }
        Rc::make_mut(&mut self.step_definition)
            .keyframes
            .insert(index, position.into());
        Ok(())
    }
}
