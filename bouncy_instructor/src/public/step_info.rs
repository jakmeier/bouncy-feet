use crate::intern::step::Step;
use crate::skeleton::Skeleton;
use crate::STATE;
use wasm_bindgen::prelude::wasm_bindgen;

/// Information about a step for display in the frontend.
#[derive(Debug)]
#[wasm_bindgen]
pub struct StepInfo {
    name: String,
    step_variation: Option<String>,
    skeletons: Vec<Skeleton>,
}

#[wasm_bindgen]
impl StepInfo {
    /// The identifier for the step. The same ID is used for variations of the same step.
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn skeleton(&self, beat: usize) -> Skeleton {
        debug_assert!(self.skeletons.len() > 0);
        self.skeletons[beat % self.skeletons.len()]
    }

    /// Description identifier for the translated text which describes how the
    /// variation is different from the original.
    ///
    /// For example: "left-first" can be used for all steps which are the same
    /// as the original but instead of starting with the right foot, it starts
    /// with the left foot first. The app shows a translated text like "Left Leg First".
    #[wasm_bindgen(getter)]
    pub fn variation(&self) -> Option<String> {
        self.step_variation.clone()
    }
}

impl From<&Step> for StepInfo {
    fn from(step: &Step) -> Self {
        let skeletons = STATE.with_borrow(|state| {
            step.poses
                .iter()
                .map(|pose_index| {
                    let pose = &state.db.poses()[*pose_index];
                    Skeleton::from_pose(pose, &state.db)
                })
                .collect()
        });
        Self {
            name: step.name.clone(),
            step_variation: step.variation.clone(),
            skeletons,
        }
    }
}
