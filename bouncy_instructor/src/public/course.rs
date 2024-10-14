use super::parsing::ParseFileError;
use super::{parsing, StepInfo};
use crate::intern::tracker_dance_collection::TrackerDanceCollection;
use crate::Tracker;
use std::rc::Rc;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Course {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) explanation: Option<String>,
    pub(crate) featured_step_id: String,
    pub(crate) lessons: Vec<Lesson>,
    pub(crate) collection: TrackerDanceCollection,
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct Lesson {
    pub(crate) name: String,
    pub(crate) explanation: Option<String>,
    pub(crate) video: Option<String>,
    pub(crate) icon: String,
    pub(crate) parts: Vec<LessonPart>,
}

#[derive(Clone)]
#[wasm_bindgen]
pub struct LessonPart {
    pub(crate) explanation: Option<String>,
    pub(crate) step_name: String,
    pub(crate) step_info: StepInfo,
    pub(crate) bpms: Vec<u16>,
}

#[derive(Debug)]
pub(crate) enum CourseError {
    MissingStep(String),
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
    pub fn explanation(&self) -> Option<String> {
        self.explanation.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn lessons(&self) -> Vec<Lesson> {
        self.lessons.clone()
    }

    #[wasm_bindgen(js_name = "featuredStep")]
    pub fn featured_step(&self) -> Option<crate::StepInfo> {
        crate::step_by_id(self.featured_step_id.clone(), false)
    }

    pub fn tracker(&self, lesson_index: usize) -> Option<Tracker> {
        self.lessons
            .get(lesson_index)
            .map(|lesson| lesson.tracker(self.collection.clone()))
    }
}

#[wasm_bindgen]
impl Lesson {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn explanation(&self) -> Option<String> {
        self.explanation.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn video(&self) -> Option<String> {
        self.video.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn parts(&self) -> Vec<LessonPart> {
        self.parts.clone()
    }

    #[wasm_bindgen(getter, js_name = "iconUrl")]
    pub fn icon_url(&self) -> String {
        self.icon.clone()
    }
}

impl Lesson {
    pub(crate) fn tracker(&self, db: impl Into<Rc<TrackerDanceCollection>>) -> crate::Tracker {
        let first_step = self
            .parts
            .first()
            .expect("no step in lesson to track")
            .step_info
            .clone();
        // TODO: make this number configurable
        let mut tracker = Tracker::new(db, Some(first_step), Some(20));
        // TODO: make half speed property configurable
        tracker.detector.half_speed = true;
        tracker
    }
}

#[wasm_bindgen]
impl LessonPart {
    #[wasm_bindgen(getter)]
    #[wasm_bindgen(js_name = "stepName")]
    pub fn step_name(&self) -> String {
        self.step_name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn explanation(&self) -> Option<String> {
        self.explanation.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn step(&self) -> StepInfo {
        self.step_info.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn bpms(&self) -> Vec<u16> {
        self.bpms.clone()
    }
}

impl Course {
    pub(crate) fn add_lesson(
        &mut self,
        lesson_name: String,
        explanation: Option<String>,
        video: Option<String>,
        lesson_icon: String,
        lesson_parts: Vec<parsing::course_file::Part>,
        lang: &str,
    ) -> Result<(), CourseError> {
        let parts = lesson_parts
            .into_iter()
            .map(|p| {
                LessonPart::new(
                    p.step,
                    p.explanations.and_then(|translated| translated.take(lang)),
                    p.bpms,
                    &self.collection,
                )
            })
            .collect::<Result<_, _>>()?;
        let lesson = Lesson {
            explanation,
            video,
            name: lesson_name,
            icon: lesson_icon,
            parts,
        };
        self.lessons.push(lesson);
        Ok(())
    }
}

impl LessonPart {
    fn new(
        step_name: String,
        explanation: Option<String>,
        bpms: Vec<u16>,
        state: &TrackerDanceCollection,
    ) -> Result<Self, CourseError> {
        let step = state
            .step(&step_name)
            .ok_or_else(|| CourseError::MissingStep(step_name.clone()))?;
        let step_info = StepInfo::from_step(step.clone(), state);
        Ok(LessonPart {
            explanation,
            step_name,
            step_info,
            bpms,
        })
    }
}

impl From<CourseError> for ParseFileError {
    fn from(err: CourseError) -> Self {
        match err {
            CourseError::MissingStep(s) => ParseFileError::UnknownStepName(s),
        }
    }
}

impl std::fmt::Debug for Course {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Course")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("featured_step_id", &self.featured_step_id)
            .field("lessons", &self.lessons)
            .field("collection", &self.collection.short_debug_string())
            .finish()
    }
}

impl std::fmt::Debug for LessonPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LessonPart")
            .field("step_name", &self.step_name)
            // intentionally omitted for brevity
            // .field("step_info", &self.step_info)
            .field("bpms", &self.bpms)
            .finish()
    }
}
