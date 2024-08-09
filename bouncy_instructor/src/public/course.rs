use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug)]
#[wasm_bindgen]
pub struct Course {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) featured_step_id: String,
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

    #[wasm_bindgen(js_name = "featuredStep")]
    pub fn featured_step(&self) -> Option<crate::StepInfo> {
        crate::step_by_id(self.featured_step_id.clone(), false)
    }
}

#[wasm_bindgen]
impl Lesson {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn parts(&self) -> Vec<LessonPart> {
        self.parts.clone()
    }
}

#[wasm_bindgen]
impl LessonPart {
    #[wasm_bindgen(getter)]
    pub fn step(&self) -> String {
        self.step.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn bpms(&self) -> Vec<u16> {
        self.bpms.clone()
    }
}
