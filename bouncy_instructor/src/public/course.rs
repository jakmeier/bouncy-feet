use super::parsing::ParseFileError;
use crate::intern::content_collection::ContentCollection;
use crate::intern::step_pace::StepPace;
use crate::intern::teacher::Teacher;
use crate::intern::tracker_dance_collection::TrackerDanceCollection;
use crate::wrapper::step_wrapper::StepWrapper;
use crate::{parsing, Tracker, VideoDef};
use std::rc::Rc;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug)]
#[wasm_bindgen]
pub struct Course {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) explanation: Option<String>,
    pub(crate) featured_step_id: String,
    pub(crate) video: Option<String>,
    pub(crate) lessons: Vec<Lesson>,
    pub(crate) collection: ContentCollection,
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct Lesson {
    pub(crate) name: String,
    pub(crate) explanation: Option<String>,
    pub(crate) explainer_video: Option<VideoDef>,
    pub(crate) front_video: Option<VideoDef>,
    pub(crate) back_video: Option<VideoDef>,
    pub(crate) song: Option<String>,
    pub(crate) song_timestamp: Option<f64>,
    pub energy: u8,
    pub difficulty: u8,
    pub(crate) parts: Vec<LessonPart>,
}

#[derive(Clone)]
#[wasm_bindgen]
pub struct LessonPart {
    pub(crate) step_name: String,
    pub(crate) step_wrapper: StepWrapper,
    /// How many times the step should be repeated.
    pub(crate) repeat: u32,
    /// At what pace the step should be danced.
    pub(crate) pace: StepPace,
    /// Whether and how the step should be danced and tracked, or just shown.
    pub(crate) tracking: TrackingKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TrackingKind {
    /// Normal tracking, used when nothing is specified.
    Tracked,
    /// Show the step but don't track it.
    Untracked,
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
    pub fn video(&self) -> Option<String> {
        self.video.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn lessons(&self) -> Vec<Lesson> {
        self.lessons.clone()
    }

    #[wasm_bindgen(js_name = "featuredStep")]
    pub fn featured_step(&self) -> Option<StepWrapper> {
        crate::step_by_id(self.featured_step_id.clone(), false)
    }

    pub fn tracker(&self, lesson_index: usize) -> Option<Tracker> {
        self.lessons.get(lesson_index).map(|lesson| {
            let has_video = lesson
                .front_video
                .as_ref()
                .map(|s| !s.is_empty())
                .unwrap_or(false);
            let mut tracker = lesson.tracker(self.collection.tracker_view.clone());
            tracker.use_teacher_video(has_video);
            tracker
        })
    }

    /// WIP: Create a training session for the given course. At the moment, it
    /// is hard coded to give something for testing.
    #[wasm_bindgen(js_name = "trainingTracker")]
    pub fn training_tracker(&self) -> Tracker {
        let db = self.collection.tracker_view.clone();

        // WIP: for now, take the last lesson and create a tracker for those parts
        let steps = self
            .lessons
            .last()
            .expect("must have lesson")
            .parts
            .iter()
            .map(|part| &part.step_wrapper);

        let mut teacher = Teacher::default();
        for step in steps {
            // WIP: hard coding this for testing
            teacher.show_step(step.info(&db), 2, StepPace::quarter_speed());
            teacher.add_step(step.info(&db), 4, StepPace::quarter_speed());
            teacher.show_step(step.info(&db), 2, StepPace::half_speed());
            teacher.add_step(step.info(&db), 4, StepPace::half_speed());
            teacher.show_step(step.info(&db), 2, StepPace::normal());
            teacher.add_step(step.info(&db), 4, StepPace::normal());
        }

        Tracker::new_from_teacher(db, teacher)
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

    #[wasm_bindgen(getter, js_name = "explainerVideo")]
    pub fn explainer_video(&self) -> Option<VideoDef> {
        self.explainer_video.clone()
    }

    #[wasm_bindgen(getter, js_name = "frontVideo")]
    pub fn front_video(&self) -> Option<VideoDef> {
        self.front_video.clone()
    }

    #[wasm_bindgen(getter, js_name = "backVideo")]
    pub fn back_video(&self) -> Option<VideoDef> {
        self.back_video.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn song(&self) -> Option<String> {
        self.song.clone()
    }

    #[wasm_bindgen(getter, js_name = "songTimestamp")]
    pub fn song_timestamp(&self) -> f64 {
        self.song_timestamp.unwrap_or(0.0)
    }

    #[wasm_bindgen(getter)]
    pub fn parts(&self) -> Vec<LessonPart> {
        self.parts.clone()
    }
}

impl Lesson {
    pub(crate) fn tracker(&self, db: impl Into<Rc<TrackerDanceCollection>>) -> crate::Tracker {
        let db = db.into();
        let mut teacher = Teacher::default();

        for part in &self.parts {
            let step = &part.step_wrapper;
            let pace = part.pace;

            match part.tracking {
                TrackingKind::Tracked => teacher.add_step(step.info(&db), part.repeat, pace),
                TrackingKind::Untracked => teacher.show_step(step.info(&db), part.repeat, pace),
            }
        }

        Tracker::new_from_teacher(db, teacher)
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
    pub fn step(&self) -> StepWrapper {
        self.step_wrapper.clone()
    }
}

impl Course {
    pub(crate) fn add_lesson(&mut self, lesson: Lesson) {
        self.lessons.push(lesson);
    }
}

impl LessonPart {
    pub(crate) fn new(
        step_name: String,
        state: &ContentCollection,
        repeat: u32,
        subbeats_per_move: u8,
        tracking: parsing::course_file::TrackingKind,
    ) -> Result<Self, CourseError> {
        let step_wrapper = state
            .step(&step_name)
            .cloned()
            .ok_or_else(|| CourseError::MissingStep(step_name.clone()))?;

        let pace = StepPace::new(subbeats_per_move as u32);
        let tracking = tracking.into();

        Ok(LessonPart {
            step_name,
            step_wrapper,
            repeat,
            pace,
            tracking,
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

impl std::fmt::Debug for LessonPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LessonPart")
            .field("step_name", &self.step_name)
            // intentionally omitted for brevity
            // .field("step_info", &self.step_info)
            .field("repeat", &self.repeat)
            .field("pace", &self.pace)
            .field("tracking", &self.tracking)
            .finish()
    }
}
