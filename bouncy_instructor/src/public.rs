//! Wrapper module for all types and methods that are exported by the WASM
//! module.
mod course;
pub(crate) mod dance_info;
pub(crate) mod editor;
pub(crate) mod keypoints;
pub(crate) mod parsing;
pub(crate) mod renderable;
pub(crate) mod skeleton;
pub(crate) mod step_info;
pub(crate) mod tracker;
pub(crate) mod ui_event;
pub(crate) mod wrapper;

use crate::intern::step::StepSource;
pub use crate::public::course::Course;
pub use dance_info::DanceInfo;
pub use keypoints::{Keypoints, Side as KeypointsSide};
pub use skeleton::Cartesian2d;
pub use step_info::StepInfo;
pub use tracker::{DetectionFailureReason, DetectionResult, PoseHint, Tracker};
pub use ui_event::{AudioEffect, TextEffect};

pub(crate) use parsing::{dance_file, pose_file, step_file};
use wrapper::pose_wrapper::PoseWrapper;

use super::STATE;
use crate::intern::lfsr;
use editor::dance_builder::DanceBuilder;
use parsing::course_file::CourseFile;
use parsing::dance_file::DanceFile;
use parsing::pose_file::PoseFile;
use parsing::step_file::StepFile;
use parsing::ParseFileError;
use std::ops::Deref;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::Request;
use wrapper::dance_wrapper::DanceWrapper;
use wrapper::step_wrapper::StepWrapper;

#[wasm_bindgen(js_name = init)]
pub fn init(random_seed: u32, lang: String) -> Result<(), JsValue> {
    if random_seed == 0 {
        return Err("random seed must not be 0".into());
    }
    lfsr::init(random_seed);
    STATE.with(|state| state.borrow_mut().reset_language(lang));
    Ok(())
}

#[wasm_bindgen(js_name = loadPoseFile)]
pub async fn load_pose_file(url: &str) -> Result<(), JsValue> {
    let text = load_text_file(url).await?;
    load_pose_str(&text)?;
    Ok(())
}

#[wasm_bindgen(js_name = loadPoseString)]
pub fn load_pose_string(data: &str) -> Result<(), JsValue> {
    load_pose_str(data)?;
    Ok(())
}

#[wasm_bindgen(js_name = loadDanceString)]
pub fn load_dance_string(data: &str) -> Result<(), JsValue> {
    load_dance_str(data)?;
    Ok(())
}

#[wasm_bindgen(js_name = loadStepFile)]
pub async fn load_step_file(url: &str, source: String) -> Result<(), JsValue> {
    let text = load_text_file(url).await?;
    load_step_str(&text, source)?;
    Ok(())
}

#[wasm_bindgen(js_name = loadDanceFile)]
pub async fn load_dance_file(url: &str) -> Result<(), JsValue> {
    let text = load_text_file(url).await?;
    load_dance_str(&text)?;
    Ok(())
}

#[wasm_bindgen(js_name = loadStepString)]
pub fn load_step_string(data: &str, source: String) -> Result<(), JsValue> {
    load_step_str(data, source)?;
    Ok(())
}

#[wasm_bindgen(js_name = parseCourseString)]
pub fn parse_course_string(data: &str, lang: &str) -> Result<Course, JsValue> {
    let course = parse_course_str(data, lang)?;
    Ok(course)
}

#[wasm_bindgen]
pub fn poses() -> Vec<PoseWrapper> {
    STATE.with_borrow(|state| state.global_db.poses().to_vec())
}

#[wasm_bindgen]
pub fn steps() -> Vec<StepWrapper> {
    STATE.with_borrow(|state| state.global_db.steps().cloned().collect())
}

#[wasm_bindgen(js_name = "stepsBySource")]
pub fn steps_by_source(source: &str) -> Vec<StepWrapper> {
    STATE.with_borrow(|state| {
        state
            .global_db
            .steps()
            .filter(|step| step.source().deref() == source)
            .cloned()
            .collect::<Vec<_>>()
    })
}

#[wasm_bindgen(js_name = "stepById")]
pub fn step_by_id(id: String, flipped: bool) -> Option<StepWrapper> {
    STATE.with_borrow(|state| {
        let mut step = state.global_db.step(&id).cloned()?;
        if flipped {
            step = step.flipped(&state.global_db.tracker_view);
        }
        Some(step)
    })
}

#[wasm_bindgen(js_name = "stepsByName")]
pub fn steps_by_name(step_name: String) -> Vec<StepWrapper> {
    STATE.with_borrow(|state| state.global_db.steps_by_name(&step_name).cloned().collect())
}

#[wasm_bindgen]
pub fn dances() -> Vec<DanceWrapper> {
    STATE.with_borrow(|state| state.global_db.dances().cloned().collect::<Vec<_>>())
}

#[wasm_bindgen(js_name = "danceBuilderFromDance")]
pub fn dance_builder_from_dance(dance_id: String) -> Result<DanceBuilder, String> {
    STATE.with_borrow(|state| {
        state
            .global_db
            .tracker_view
            .dances()
            .iter()
            .find(|dance| dance.id == dance_id)
            .cloned()
            .map(DanceBuilder::from)
            .ok_or_else(|| format!("missing dance {dance_id}"))
    })
}

pub fn load_pose_str(text: &str) -> Result<(), ParseFileError> {
    console_error_panic_hook::set_once();
    let parsed = PoseFile::from_str(text)?;
    STATE.with(|state| state.borrow_mut().add_poses(parsed.poses))?;
    Ok(())
}

pub fn load_step_str(text: &str, source: String) -> Result<(), ParseFileError> {
    let parsed = StepFile::from_str(text)?;
    STATE.with(|state| state.borrow_mut().add_steps(parsed.steps, source))?;
    Ok(())
}

pub fn load_dance_str(text: &str) -> Result<(), ParseFileError> {
    let parsed = DanceFile::from_str(text)?;
    STATE.with(|state| state.borrow_mut().add_dances(parsed.dances))?;
    Ok(())
}

pub fn parse_course_str(text: &str, lang: &str) -> Result<Course, ParseFileError> {
    let parsed = CourseFile::from_str(text)?;
    parsed.into_course(lang)
}

async fn load_text_file(url: &str) -> Result<String, JsValue> {
    let request = Request::new_with_str(url)?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: web_sys::Response = resp_value.dyn_into().unwrap();
    let js_value = JsFuture::from(resp.text()?).await?;
    let text = js_value.as_string().ok_or("Not a string")?;
    Ok(text)
}

#[wasm_bindgen(js_name = "addLocalPoses")]
pub fn add_local_poses(poses: Vec<PoseWrapper>) {
    STATE.with(|state| state.borrow_mut().global_db.complement_poses(poses));
}

#[wasm_bindgen(js_name = "loadLocalSteps")]
pub fn load_local_steps(steps: Vec<StepWrapper>) {
    STATE.with(|state| {
        state
            .borrow_mut()
            .global_db
            .replace_steps(StepSource::new("lab".to_owned()), steps)
    });
}
