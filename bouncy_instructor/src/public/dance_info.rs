use crate::intern::dance::Dance;
use crate::public::skeleton::Skeleton;
use crate::{StepInfo, STATE};
use wasm_bindgen::prelude::wasm_bindgen;

/// Information about a dance for display in the frontend.
#[derive(Debug)]
#[wasm_bindgen]
pub struct DanceInfo {
    id: String,
    steps: Vec<StepInfo>,
    /// invariant: total_beats is the sum of all steps beat lengths
    total_beats: usize,
}

#[wasm_bindgen]
impl DanceInfo {
    /// Create a new dance info object, without registering it.
    ///
    /// Cursed: Calling this constructor leaves the values inside the input list wasm-nulled out.
    /// 
    /// Let's say we pass in `[new StepInfo(), new SteInfo()]`. After calling
    /// this, it will look like
    /// `[Object { __wbg_ptr: 0 }, Object { __wbg_ptr: 0 }]`.
    /// The Rust values have been moved out of the JS array into a Rust Vec.
    /// 
    /// At this point, checking the values inside the array for null in JS will
    /// correctly show that these are not null. But any further calls to Rust
    /// methods will result in "Error: null pointer passed to rust".
    // TODO: Can I find a cleaner solution?
    #[wasm_bindgen(constructor)]
    pub fn new(id: String, steps: Vec<StepInfo>) -> Self {
        Self {
            id,
            total_beats: steps.iter().map(|step| step.beats()).sum(),
            steps,
        }
    }

    /// The unique identifier for the dance.
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn steps(&self) -> Vec<StepInfo> {
        self.steps.clone()
    }

    pub fn skeleton(&self, beat: usize) -> Skeleton {
        debug_assert!(!self.steps.is_empty() && self.total_beats > 0);
        let mut offset = beat % self.total_beats;

        for step in &self.steps {
            if step.beats() < offset {
                offset -= step.beats();
            } else {
                return step.skeleton(offset);
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
                .steps
                .iter()
                .map(|step_name| {
                    state
                        .first_step_by_name(step_name)
                        .expect("step must exist")
                        .into()
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
