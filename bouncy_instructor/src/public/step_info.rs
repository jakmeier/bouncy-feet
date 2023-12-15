use crate::intern::step::Step;
use crate::skeleton::Skeleton;
use crate::STATE;
use wasm_bindgen::prelude::wasm_bindgen;

/// Information about a step for display in the frontend.
#[derive(Debug)]
#[wasm_bindgen]
pub struct StepInfo {
    name: String,
    skeletons: Vec<Skeleton>,
}

#[wasm_bindgen]
impl StepInfo {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn skeleton(&self, beat: usize) -> Skeleton {
        debug_assert!(self.skeletons.len() > 0);
        self.skeletons[beat % self.skeletons.len()]
    }
}

impl From<&Step> for StepInfo {
    fn from(step: &Step) -> Self {
        let skeletons = STATE.with_borrow(|state| {
            step.poses
                .iter()
                .enumerate()
                .map(|(i, pose_index)| {
                    let pose = &state.db.poses()[*pose_index];
                    Skeleton::from_pose(pose, step.directions[i], &state.db)
                })
                .collect()
        });
        Self {
            name: step.name.clone(),
            skeletons,
        }
    }
}
