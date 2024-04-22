//! Defines the external formats for defining poses, steps, and dances. This is
//! one of the main interfaces of the bouncy_instructor WASM module.
//!
//! Best practice: Don't use any of the type of this file outside of parsing
//! logic. Instead, translate to internal types. This allows refactoring
//! internal without changing the external formats.

pub(crate) mod dance_file;
pub(crate) mod pose_file;
pub(crate) mod step_file;

use thiserror::Error;
use wasm_bindgen::JsValue;

#[derive(Error, Debug)]
pub enum ParseFileError {
    #[error("invalid file version (expected {expected:?}, found {found:?})")]
    VersionMismatch { expected: u16, found: u16 },
    #[error("parsing file failed, {0}")]
    RonError(#[from] ron::error::SpannedError),
    #[error("unknown pose reference `{0}`")]
    UnknownPoseReference(String),
    #[error("unknown step reference `{0}`")]
    UnknownStepName(String),
}

impl From<ParseFileError> for JsValue {
    fn from(value: ParseFileError) -> Self {
        format!("{value}").into()
    }
}

#[cfg(test)]
mod tests {
    use crate::{dances, load_dance_str, load_pose_str, load_step_str, steps, STATE};

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
            steps: ["rm-0"],
          ),
          (
            id: "test-dance-1",
            steps: ["rm-0", "rm-0", "rm-0"],
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
              steps: ["rm-0"],
            ),
            (
              id: "test-dance-1",
              steps: ["rm-0", "rm-0", "fake-id", "rm-0"],
            ),
          ]
        )
        "#;
        load_pose_str(POSE_STR).unwrap();
        load_step_str(STEP_STR).unwrap();
        match load_dance_str(input) {
            Err(ParseFileError::UnknownStepName(id)) if id == "fake-id" => (),
            Err(other) => panic!("wrong error {other}"),
            Ok(()) => panic!("expected an error when loading invalid reference"),
        }
    }
}
