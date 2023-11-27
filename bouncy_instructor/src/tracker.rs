use crate::keypoints::Keypoints;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Tracker {
    // invariant: ordered by timestamp
    history: Vec<(u32, Keypoints)>,
}

#[wasm_bindgen]
impl Tracker {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Tracker {
            // order by timestamp satisfied for empty list
            history: vec![],
        }
    }

    pub fn add_keypoints(&mut self, keypoints: Keypoints, timestamp: u32) {
        if let Some(last) = self.history.last() {
            if last.0 >= timestamp {
                panic!("inserted data not strictly monotonically increasing");
            }
        }
        // modification preserves timestamp order if it was true before
        self.history.push((timestamp, keypoints));
    }
}
