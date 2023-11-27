mod keypoints;
mod pose;
mod tracker;

pub use keypoints::{Keypoints, Side as KeypointsSide};
pub use tracker::Tracker;

use pose::{Pose, PoseFile};
use std::cell::RefCell;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::Request;

/// Singleton internal state, shared between `Tracker` instances that run in the
/// same JS worker thread.
struct State {
    poses: Vec<Pose>,
}
thread_local! {
    static STATE: RefCell<State> = State { poses: vec![] }.into();
}

#[wasm_bindgen(js_name = loadPoseFile)]
pub async fn load_pose_file(url: &str) -> Result<(), JsValue> {
    let request = Request::new_with_str(&url)?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: web_sys::Response = resp_value.dyn_into().unwrap();
    let js_value = JsFuture::from(resp.text()?).await?;
    let text = js_value.as_string().ok_or("Not a string")?;
    let parsed = PoseFile::from_str(&text)?;
    STATE.with(|state| state.borrow_mut().poses.extend(parsed.poses));

    Ok(())
}
