//! UI events emitted by Wasm code, to be picked up by JS code. These can be
//! visual or audio effects.
use std::collections::VecDeque;

use wasm_bindgen::prelude::wasm_bindgen;

type Timestamp = f64;

#[derive(Default, Debug)]
pub(crate) struct UiEvents {
    audio: VecDeque<AudioEffect>,
}

#[derive(Debug, PartialEq)]
#[wasm_bindgen]
pub struct AudioEffect {
    /// When the sound should be played, could be in the future.
    pub timestamp: Timestamp,
    /// Which sound to play.
    pub(crate) sound_id: String,
}

impl UiEvents {
    pub(crate) fn next_audio(&mut self) -> Option<AudioEffect> {
        self.audio.pop_front()
    }

    pub(crate) fn add_audio(&mut self, t: Timestamp, sound_id: String) {
        let effect = AudioEffect {
            sound_id,
            timestamp: t,
        };
        let pos = self.audio.partition_point(|e| e.timestamp < t);
        self.audio.insert(pos, effect);
    }
}

#[wasm_bindgen]
impl AudioEffect {
    #[wasm_bindgen(getter, js_name=soundId)]
    pub fn sound_id(&self) -> String {
        self.sound_id.clone()
    }
}
