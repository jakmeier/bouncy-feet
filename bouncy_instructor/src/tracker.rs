use crate::keypoints::Keypoints;
use crate::println;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Tracker {
    /// full keypoints as originally recorded
    /// invariant: ordered by timestamp
    history: Vec<(u32, Keypoints)>,
    /// tracked limbs
    limb_angles: Vec<Vec<f32>>,
}

#[wasm_bindgen]
impl Tracker {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Tracker {
            // order by timestamp satisfied for empty list
            history: vec![],
            limb_angles: vec![],
        }
    }

    #[wasm_bindgen(js_name = addKeypoints)]
    pub fn add_keypoints(&mut self, keypoints: Keypoints, timestamp: u32) {
        if let Some(last) = self.history.last() {
            if last.0 >= timestamp {
                panic!("inserted data not strictly monotonically increasing");
            }
        }
        // modification preserves timestamp order if it was true before
        self.history.push((timestamp, keypoints));
        let limbs = super::STATE.with(|state| state.borrow().db.angles_from_keypoints(&keypoints));
        self.limb_angles.push(limbs);
    }
}
