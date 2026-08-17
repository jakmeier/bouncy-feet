use crate::intern::content_collection::ContentCollection;
use crate::intern::step_pace::StepPace;
use crate::intern::teacher::Teacher;
use crate::public::course::CourseError;
use crate::public::wrapper::step_wrapper::StepWrapper;
use crate::public::SongTimingOverride;
use crate::{Tracker, VideoDef};
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::wasm_bindgen;

/// Challenges score at least this much dancing before they may finish.
///
/// Currently duplicated in bouncy_frontend/src/lib/challengeAttempt.js
pub const MINIMUM_CHALLENGE_DURATION_MS: u32 = 20_000;
const SUBBEAT_DURATION_NUMERATOR_MS: u32 = 30_000;
const REQUIRED_HIT_PERCENT: u32 = 60;

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
    pub(crate) default_song: Option<String>,
    pub(crate) allowed_songs: Vec<String>,
    pub(crate) song_timestamp: Option<f64>,
    pub(crate) song_timing_overrides: HashMap<String, SongTimingOverride>,
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
    #[wasm_bindgen(getter, js_name = defaultSong)]
    pub fn default_song(&self) -> Option<String> {
        self.default_song.clone()
    }
    #[wasm_bindgen(getter, js_name = allowedSongs)]
    pub fn allowed_songs(&self) -> Vec<String> {
        self.allowed_songs.clone()
    }
    #[wasm_bindgen(getter, js_name = songTimestamp)]
    pub fn song_timestamp(&self) -> f64 {
        self.song_timestamp.unwrap_or(0.0)
    }
    #[wasm_bindgen(js_name = songTimingFor)]
    pub fn song_timing_for(&self, song_id: String) -> Option<SongTimingOverride> {
        self.song_timing_overrides.get(&song_id).cloned()
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

    /// Duration of the initial scoring window. It ends exactly at the end of a
    /// configured challenge repetition.
    #[wasm_bindgen(getter, js_name = challengeDurationMs)]
    pub fn challenge_duration_ms(&self) -> u32 {
        let (_, total_subbeats, _) = self.attempt_shape();
        // A subbeat lasts 30_000 / BPM milliseconds.
        total_subbeats * SUBBEAT_DURATION_NUMERATOR_MS / self.bpm as u32
    }

    /// Hits needed to complete the challenge.
    #[wasm_bindgen(getter, js_name = requiredHits)]
    pub fn required_hits(&self) -> u32 {
        let (_, _, expected_poses) = self.attempt_shape();
        (expected_poses * REQUIRED_HIT_PERCENT).div_ceil(100)
    }

    /// Total pose opportunities in the initial scoring window.
    #[wasm_bindgen(getter, js_name = maxPoses)]
    pub fn max_poses(&self) -> u32 {
        let (_, _, expected_poses) = self.attempt_shape();
        expected_poses
    }

    /// Fully correct step repetitions needed to complete the challenge.
    #[wasm_bindgen(getter, js_name = requiredSteps)]
    pub fn required_steps(&self) -> u32 {
        (self.max_steps() * REQUIRED_HIT_PERCENT).div_ceil(100)
    }

    /// Total full-step opportunities in the initial scoring window.
    #[wasm_bindgen(getter, js_name = maxSteps)]
    pub fn max_steps(&self) -> u32 {
        let (repetitions, _, _) = self.attempt_shape();
        let pattern_steps: u32 = self.parts.iter().map(|part| part.repeat).sum();
        repetitions * pattern_steps
    }
}
impl Challenge {
    /// Returns the number of full pattern repetitions, the corresponding
    /// subbeats, and score opportunities in the initial scoring window.
    fn attempt_shape(&self) -> (u32, u32, u32) {
        let db = &self.collection.tracker_view;
        let (pattern_subbeats, pattern_poses) =
            self.parts
                .iter()
                .fold((0_u32, 0_u32), |(subbeats, poses), part| {
                    let poses_per_step = part.step_wrapper.info(db).num_poses() as u32;
                    (
                        subbeats + part.repeat * poses_per_step * part.pace.subbeats_per_pose(),
                        poses + part.repeat * poses_per_step,
                    )
                });
        assert!(pattern_subbeats > 0, "challenge must contain poses");
        let minimum_subbeats = (MINIMUM_CHALLENGE_DURATION_MS * self.bpm as u32)
            .div_ceil(SUBBEAT_DURATION_NUMERATOR_MS);
        let repetitions = minimum_subbeats.div_ceil(pattern_subbeats);
        (
            repetitions,
            repetitions * pattern_subbeats,
            repetitions * pattern_poses,
        )
    }

    fn tracker(&self) -> Tracker {
        let db = Rc::clone(&self.collection.tracker_view);
        let (repetitions, _, _) = self.attempt_shape();
        let offline_step_limit = self.parts.iter().map(|part| part.repeat as usize).sum();
        let mut teacher = Teacher::default();
        for _ in 0..repetitions {
            for p in &self.parts {
                teacher.add_step(p.step_wrapper.info(&db), p.repeat, p.pace);
            }
        }
        // Continue after the initial score window so dancers can recover from
        // misses. The live activity stops this loop on a successful score or
        // when the song/video ends. Offline solution-video tests still verify
        // only the one authored pattern.
        teacher.set_offline_step_limit(offline_step_limit);
        teacher.loop_forever();
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
