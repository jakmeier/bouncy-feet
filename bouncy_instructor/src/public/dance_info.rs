use crate::intern::dance::Dance;
use crate::public::skeleton::Skeleton;
use crate::{StepInfo, STATE};
use wasm_bindgen::prelude::wasm_bindgen;

/// Information about a dance for display in the frontend.
#[derive(Debug)]
#[wasm_bindgen]
pub struct DanceInfo {
    pub(crate) id: String,
    pub(crate) steps: Vec<StepInfo>,
    /// invariant: total_beats is the sum of all steps beat lengths
    pub(crate) total_beats: usize,
}

#[wasm_bindgen]
impl DanceInfo {
    /// The unique identifier for the dance.
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn length(&self) -> usize {
        self.steps.len()
    }

    pub fn steps(&self) -> Vec<StepInfo> {
        self.steps.clone()
    }

    pub fn skeleton(&self, beat: usize) -> Option<Skeleton> {
        if self.steps.is_empty() {
            return None;
        }
        debug_assert!(self.total_beats > 0);
        let mut offset = beat % self.total_beats;

        for step in &self.steps {
            if step.beats() < offset {
                offset -= step.beats();
            } else {
                return Some(step.skeleton(offset));
            }
        }
        unreachable!("must find a skeleton");
    }

    /// The number of beats the dance takes for one repetition.
    #[wasm_bindgen(getter)]
    pub fn beats(&self) -> usize {
        self.total_beats
    }
}

impl From<&Dance> for DanceInfo {
    fn from(dance: &Dance) -> Self {
        let steps: Vec<StepInfo> = STATE.with_borrow(|state| {
            dance
                .step_ids
                .iter()
                .zip(&dance.flip_orientation)
                .map(|(step_name, &flipped)| {
                    let mut step = state.step(step_name).expect("step must exist").clone();
                    if flipped {
                        step = step.flipped();
                    }
                    step.into()
                })
                .collect()
        });
        let total_beats = steps.iter().map(|step| step.beats()).sum();
        Self {
            id: dance.id.clone(),
            steps,
            total_beats,
        }
    }
}
