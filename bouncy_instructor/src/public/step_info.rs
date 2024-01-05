use crate::intern::skeleton_3d::Skeleton3d;
use crate::intern::step::Step;
use crate::skeleton::Skeleton;
use crate::STATE;
use wasm_bindgen::prelude::wasm_bindgen;

/// Information about a step for display in the frontend.
#[derive(Debug)]
#[wasm_bindgen]
pub struct StepInfo {
    id: String,
    name: String,
    step_variation: Option<String>,
    skeletons: Vec<Skeleton>,
}

#[wasm_bindgen]
impl StepInfo {
    /// The unique identifier for the step.
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.id.clone()
    }

    /// The descriptive name for the step. The same name is used for variations
    /// of the same step.
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn skeleton(&self, beat: usize) -> Skeleton {
        debug_assert!(self.skeletons.len() > 0);
        self.skeletons[beat % self.skeletons.len()]
    }

    /// Applies a rotation (in degree) and returns the resulting skelton.
    #[wasm_bindgen(js_name = "rotatedSkeleton")]
    pub fn rotated_skeleton(&self, beat: usize, rotation: f32) -> Skeleton {
        debug_assert!(self.skeletons.len() > 0);
        STATE.with_borrow(|state| {
            let step = state.step(&self.id).expect("missing step");
            let pose_index = step.poses[beat % step.poses.len()];
            let pose = &state.db.poses()[pose_index];
            Skeleton3d::from_with_db(pose, &state.db).to_skeleton(rotation)
        })
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

    /// The number of beats the step takes for one repetition.
    #[wasm_bindgen(getter)]
    pub fn beats(&self) -> usize {
        self.skeletons.len()
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
            id: step.id.clone(),
            name: step.name.clone(),
            step_variation: step.variation.clone(),
            skeletons,
        }
    }
}
