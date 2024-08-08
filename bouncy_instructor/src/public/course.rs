use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug)]
#[wasm_bindgen]
pub struct Course {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) lessons: Vec<Lesson>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[wasm_bindgen]
pub struct Lesson {
    pub(crate) name: String,
    pub(crate) icon: String,
    pub(crate) parts: Vec<LessonPart>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[wasm_bindgen]
pub struct LessonPart {
    pub(crate) step: String,
    pub(crate) bpms: Vec<u16>,
}

#[wasm_bindgen]
impl Course {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.id.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn lessons(&self) -> Vec<Lesson> {
        self.lessons.clone()
    }
}
