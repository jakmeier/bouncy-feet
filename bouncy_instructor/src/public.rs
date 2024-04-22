//! Wrapper module for all types and methods that are exported by the WASM
//! module.
pub(crate) mod dance_info;
pub(crate) mod editor;
pub(crate) mod keypoints;
pub(crate) mod parsing;
pub(crate) mod skeleton;
pub(crate) mod step_info;
pub(crate) mod tracker;

pub use dance_info::DanceInfo;
pub use keypoints::{Keypoints, Side as KeypointsSide};
pub use step_info::StepInfo;
pub use tracker::Tracker;

pub(crate) use parsing::{dance_file, pose_file, step_file};

use super::STATE;
use editor::dance_builder::DanceBuilder;
use parsing::dance_file::DanceFile;
use parsing::pose_file::PoseFile;
use parsing::step_file::StepFile;
use parsing::ParseFileError;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::Request;

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
pub async fn load_step_file(url: &str) -> Result<(), JsValue> {
    let text = load_text_file(url).await?;
    load_step_str(&text)?;
    Ok(())
}

#[wasm_bindgen(js_name = loadDanceFile)]
pub async fn load_dance_file(url: &str) -> Result<(), JsValue> {
    let text = load_text_file(url).await?;
    load_dance_str(&text)?;
    Ok(())
}

#[wasm_bindgen(js_name = loadStepString)]
pub fn load_step_string(data: &str) -> Result<(), JsValue> {
    load_step_str(data)?;
    Ok(())
}

#[wasm_bindgen]
pub fn steps() -> Vec<StepInfo> {
    STATE.with_borrow(|state| {
        state
            .db
            .steps()
            .iter()
            .cloned()
            .map(StepInfo::from)
            .collect::<Vec<_>>()
    })
}

#[wasm_bindgen(js_name = "stepById")]
pub fn step_by_id(id: String, flipped: bool) -> Option<StepInfo> {
    let mut step = STATE.with_borrow(|state| state.db.step(&id).cloned())?;
    if flipped {
        step = step.flipped();
    }
    Some(StepInfo::from(step))
}

#[wasm_bindgen]
pub fn dances() -> Vec<DanceInfo> {
    STATE.with_borrow(|state| {
        state
            .db
            .dances()
            .iter()
            .map(DanceInfo::from)
            .collect::<Vec<_>>()
    })
}

#[wasm_bindgen(js_name = "danceBuilderFromDance")]
pub fn dance_builder_from_dance(dance_id: String) -> Result<DanceBuilder, String> {
    STATE.with_borrow(|state| {
        state
            .db
            .dances()
            .iter()
            .find(|dance| dance.id == dance_id)
            .cloned()
            .map(|dance| DanceBuilder::from(dance))
            .ok_or_else(|| format!("missing dance {dance_id}"))
    })
}

pub fn load_pose_str(text: &str) -> Result<(), ParseFileError> {
    let parsed = PoseFile::from_str(text)?;
    STATE.with(|state| state.borrow_mut().add_poses(parsed.poses))?;
    Ok(())
}

pub fn load_step_str(text: &str) -> Result<(), ParseFileError> {
    let parsed = StepFile::from_str(text)?;
    STATE.with(|state| state.borrow_mut().add_steps(&parsed.steps))?;
    Ok(())
}

pub fn load_dance_str(text: &str) -> Result<(), ParseFileError> {
    let parsed = DanceFile::from_str(text)?;
    STATE.with(|state| state.borrow_mut().add_dances(parsed.dances))?;
    Ok(())
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
