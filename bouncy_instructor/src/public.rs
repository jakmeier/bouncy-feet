//! Wrapper module for all types and methods that are exported by the WASM
//! module.
pub(crate) mod keypoints;
pub(crate) mod pose_file;
pub(crate) mod skeleton;
pub(crate) mod tracker;

pub use keypoints::{Keypoints, Side as KeypointsSide};
pub use tracker::Tracker;

use self::pose_file::ParseFileError;
use super::STATE;
use pose_file::PoseFile;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::Request;

#[wasm_bindgen(js_name = loadPoseFile)]
pub async fn load_pose_file(url: &str) -> Result<(), JsValue> {
    let request = Request::new_with_str(&url)?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: web_sys::Response = resp_value.dyn_into().unwrap();
    let js_value = JsFuture::from(resp.text()?).await?;
    let text = js_value.as_string().ok_or("Not a string")?;
    load_pose_str(&text)?;
    Ok(())
}

pub fn load_pose_str(text: &str) -> Result<(), ParseFileError> {
    let parsed = PoseFile::from_str(&text)?;
    STATE.with(|state| state.borrow_mut().db.add(parsed.poses))?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_pose_reference() {
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
              mirror_of: "test-pose-left",
            ),
          ]
        )
        "#;
        load_pose_str(input).unwrap();
        let num_poses = STATE.with_borrow(|state| state.db.poses().len());
        assert_eq!(num_poses, 2);
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
}
