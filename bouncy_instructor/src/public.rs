//! Wrapper module for all types and methods that are exported by the WASM
//! module.
pub(crate) mod dance_file;
pub(crate) mod dance_info;
pub(crate) mod keypoints;
pub(crate) mod pose_file;
pub(crate) mod skeleton;
pub(crate) mod step_file;
pub(crate) mod step_info;
pub(crate) mod tracker;

pub use dance_info::DanceInfo;
pub use keypoints::{Keypoints, Side as KeypointsSide};
pub use step_info::StepInfo;
pub use tracker::Tracker;

use self::dance_file::DanceFile;
use self::pose_file::ParseFileError;
use self::step_file::StepFile;
use super::STATE;
use pose_file::PoseFile;
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

#[wasm_bindgen(js_name = loadStepFile)]
pub async fn load_step_file(url: &str) -> Result<(), JsValue> {
    let text = load_text_file(url).await?;
    load_step_str(&text)?;
    Ok(())
}

#[wasm_bindgen(js_name = loadDanceFile)]
pub async fn load_danc_file(url: &str) -> Result<(), JsValue> {
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
    STATE.with_borrow(|state| state.steps.iter().map(StepInfo::from).collect::<Vec<_>>())
}

#[wasm_bindgen]
pub fn dances() -> Vec<DanceInfo> {
    STATE.with_borrow(|state| state.dances.iter().map(DanceInfo::from).collect::<Vec<_>>())
}

pub fn load_pose_str(text: &str) -> Result<(), ParseFileError> {
    let parsed = PoseFile::from_str(&text)?;
    STATE.with(|state| state.borrow_mut().add_poses(parsed.poses))?;
    Ok(())
}

pub fn load_step_str(text: &str) -> Result<(), ParseFileError> {
    let parsed = StepFile::from_str(&text)?;
    STATE.with(|state| state.borrow_mut().add_steps(&parsed.steps))?;
    Ok(())
}

pub fn load_dance_str(text: &str) -> Result<(), ParseFileError> {
    let parsed = DanceFile::from_str(&text)?;
    STATE.with(|state| state.borrow_mut().add_dances(parsed.dances))?;
    Ok(())
}

async fn load_text_file(url: &str) -> Result<String, JsValue> {
    let request = Request::new_with_str(&url)?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: web_sys::Response = resp_value.dyn_into().unwrap();
    let js_value = JsFuture::from(resp.text()?).await?;
    let text = js_value.as_string().ok_or("Not a string")?;
    Ok(text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::expect;

    const POSE_STR: &str = r#"
      #![enable(implicit_some)]
      (
        version: 0,
        poses: [
          (
            name: "test-pose-left",
            direction: Right,
            limbs: [
              (limb: LeftShin, angle: 0, tolerance: 5, weight: 1.0),
            ]
          ),
          (
            name: "test-pose-right",
            direction: Right,
            mirror_of: "test-pose-left",
          ),
        ]
      )
      "#;

    const STEP_STR: &str = r#"
      #![enable(implicit_some)]
      (
        version: 0,
        steps: [
          (
            name: "Running Man",
            id: "rm-0",
            keyframes: [
              (pose: "test-pose-left", orientation: Right),
              (pose: "test-pose-right", orientation: Right),
            ]
          ),
          (
            name: "Running Man",
            id: "rm-1",
            variation: "reversed",
            keyframes: [
              (pose: "test-pose-right", orientation: Right),
              (pose: "test-pose-left", orientation: Right),
            ]
          ),
        ]
      )
      "#;

    const DANCE_STR: &str = r#"
      (
        version: 0,
        dances: [
          (
            id: "test-dance-0",
            steps: ["Running Man"],
          ),
          (
            id: "test-dance-1",
            steps: ["Running Man", "Running Man", "Running Man"],
          ),
        ]
      )
      "#;

    #[test]
    fn test_valid_pose_reference() {
        load_pose_str(POSE_STR).unwrap();
        load_step_str(STEP_STR).unwrap();
        let num_poses = STATE.with_borrow(|state| state.db.poses().len());
        assert_eq!(num_poses, 2);
    }

    #[test]
    fn test_basic_step_loading() {
        load_pose_str(POSE_STR).unwrap();
        load_step_str(STEP_STR).unwrap();

        let step_ids = steps()
            .into_iter()
            .map(|step| step.id())
            .collect::<Vec<_>>();
        expect![[r#"
            [
                "rm-0",
                "rm-1",
            ]
        "#]]
        .assert_debug_eq(&step_ids);
    }

    #[test]
    fn test_basic_dance_loading() {
        load_pose_str(POSE_STR).unwrap();
        load_step_str(STEP_STR).unwrap();
        load_dance_str(DANCE_STR).unwrap();

        let dance_ids = dances()
            .into_iter()
            .map(|dance| dance.id())
            .collect::<Vec<_>>();
        expect![[r#"
            [
                "test-dance-0",
                "test-dance-1",
            ]
        "#]]
        .assert_debug_eq(&dance_ids);
    }

    #[test]
    fn test_invalid_pose_reference() {
        let input = r#"
        #![enable(implicit_some)]
        (
          version: 0,
          poses: [
            (
              name: "test-pose-left",
              direction: Right,
              limbs: [
                  (limb: LeftShin, angle: 0, tolerance: 5, weight: 1.0),
              ]
            ),
            (
              name: "test-pose-right",
              direction: Right,
              mirror_of: "fake-id",
            ),
          ]
        )
        "#;
        match load_pose_str(input) {
            Err(ParseFileError::UnknownPoseReference(id)) if id == "fake-id" => (),
            Err(other) => panic!("wrong error {other}"),
            Ok(()) => panic!("expected an error when loading invalid reference"),
        }
    }

    #[test]
    fn test_invalid_step_reference() {
        let input = r#"
        (
          version: 0,
          dances: [
            (
              id: "test-dance-0",
              steps: ["Running Man"],
            ),
            (
              id: "test-dance-1",
              steps: ["Running Man", "Running Man", "fake-name", "Running Man"],
            ),
          ]
        )
        "#;
        load_pose_str(POSE_STR).unwrap();
        load_step_str(STEP_STR).unwrap();
        match load_dance_str(input) {
            Err(ParseFileError::UnknownStepName(id)) if id == "fake-name" => (),
            Err(other) => panic!("wrong error {other}"),
            Ok(()) => panic!("expected an error when loading invalid reference"),
        }
    }
}
