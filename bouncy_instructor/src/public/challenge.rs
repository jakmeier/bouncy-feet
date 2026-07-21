use crate::intern::content_collection::ContentCollection;
use crate::intern::step_pace::StepPace;
use crate::intern::teacher::Teacher;
use crate::public::course::CourseError;
use crate::public::wrapper::step_wrapper::StepWrapper;
use crate::{Tracker, VideoDef};
use std::rc::Rc;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug)]
#[wasm_bindgen]
pub struct ChallengeSet {
    pub(crate) id: String,
    pub(crate) definition_version: u16,
    pub(crate) name: String,
    pub(crate) explanation: Option<String>,
    pub(crate) challenges: Vec<Challenge>,
}
#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct Challenge {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) explanation: Option<String>,
    pub(crate) prerequisites: Vec<String>,
    pub(crate) bpm: u16,
    pub(crate) solution_video: VideoDef,
    pub(crate) solution_start_timestamp: f64,
    pub(crate) parts: Vec<ChallengePart>,
    /// The definitions needed to run this challenge, excluding unrelated
    /// challenge-set poses and steps.
    pub(crate) collection: ContentCollection,
}
#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct ChallengePart {
    pub(crate) step_name: String,
    pub(crate) step_wrapper: StepWrapper,
    pub(crate) repeat: u32,
    pub(crate) pace: StepPace,
}

#[wasm_bindgen]
impl ChallengeSet {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.id.clone()
    }
    #[wasm_bindgen(getter, js_name = definitionVersion)]
    pub fn definition_version(&self) -> u16 {
        self.definition_version
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
    pub fn challenges(&self) -> Vec<Challenge> {
        self.challenges.clone()
    }
    pub fn tracker(&self, index: usize) -> Option<Tracker> {
        self.challenges.get(index).map(Challenge::tracker)
    }
}
#[wasm_bindgen]
impl Challenge {
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
    pub fn prerequisites(&self) -> Vec<String> {
        self.prerequisites.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn bpm(&self) -> u16 {
        self.bpm
    }
    #[wasm_bindgen(getter, js_name = solutionVideo)]
    pub fn solution_video(&self) -> VideoDef {
        self.solution_video.clone()
    }
    #[wasm_bindgen(getter, js_name = solutionStartTimestamp)]
    pub fn solution_start_timestamp(&self) -> f64 {
        self.solution_start_timestamp
    }
    #[wasm_bindgen(getter)]
    pub fn parts(&self) -> Vec<ChallengePart> {
        self.parts.clone()
    }
}
impl Challenge {
    fn tracker(&self) -> Tracker {
        let db = Rc::clone(&self.collection.tracker_view);
        let mut teacher = Teacher::default();
        for p in &self.parts {
            teacher.add_step(p.step_wrapper.info(&db), p.repeat, p.pace);
        }
        let mut tracker = Tracker::new_from_teacher(db, teacher);
        tracker.set_bpm(self.bpm as f32);
        tracker.enforce_beat(true);
        tracker
    }
}
#[wasm_bindgen]
impl ChallengePart {
    #[wasm_bindgen(getter, js_name = stepName)]
    pub fn step_name(&self) -> String {
        self.step_name.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn repeat(&self) -> u32 {
        self.repeat
    }
    #[wasm_bindgen(getter, js_name = subbeatsPerMove)]
    pub fn subbeats_per_move(&self) -> u8 {
        self.pace.subbeats_per_pose() as u8
    }
}
impl ChallengePart {
    pub(crate) fn new(
        step_name: String,
        collection: &ContentCollection,
        repeat: u32,
        subbeats_per_move: u8,
    ) -> Result<Self, CourseError> {
        let step_wrapper = collection
            .step(&step_name)
            .cloned()
            .ok_or_else(|| CourseError::MissingStep(step_name.clone()))?;
        Ok(Self {
            step_name,
            step_wrapper,
            repeat,
            pace: StepPace::new(subbeats_per_move as u32),
        })
    }
}
