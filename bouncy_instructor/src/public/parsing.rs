//! Defines the external formats for defining poses, steps, and dances. This is
//! one of the main interfaces of the bouncy_instructor WASM module.
//!
//! Best practice: Don't use any of the type of this file outside of parsing
//! logic. Instead, translate to internal types. This allows refactoring
//! internal without changing the external formats.

pub(crate) mod course_file;
pub(crate) mod dance_file;
pub(crate) mod pose_file;
pub(crate) mod step_file;

use thiserror::Error;
use wasm_bindgen::JsValue;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct VersionCheck {
    pub version: u16,
}

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
    #[error("missing translation for `{id}` with lang `{lang}`")]
    MissingTranslation { id: String, lang: String },
}

impl From<ParseFileError> for JsValue {
    fn from(value: ParseFileError) -> Self {
        format!("{value}").into()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        dances, load_dance_str, load_pose_str, load_step_str, parse_course_str, steps, STATE,
    };

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

    const COURSE_STR: &str = r#"
    #![enable(implicit_some)]
    (
      version: 0,
      id: "running-man-basics",
      names: {"de": "Running Man Anfängerkurs", "en": "Running Man beginner's course"},
      featured_step: "rm-0",
      lessons: [
        (
          names: {"de": "Micro Bounce", "en": "Micro Bounce"},
          difficulty: 2,
          energy: 3,
          parts: [
            (step: "run-in-place", repeat: 1, subbeats_per_move: 1),
            (step: "another-step", repeat: 2, subbeats_per_move: 4),
          ],
        ),
      ],
      poses: [
        (
          id: "in-place-right-up",
          direction: Front,
          limbs: [
            (limb: LeftThigh, weight: 1.0, angle: -1, tolerance: 0),
            (limb: LeftShin, weight: 1.0, angle: 3, tolerance: 0),
            (limb: LeftFoot, weight: 1.0, angle: -7, tolerance: 0),
          ],
          z: (
            absolute: {(side: Right, part: Knee): -1.0, (side: Left, part: Heel): 1.0},
          ),
        ),
        (
          id: "in-place-left-up",
          direction: Front,
          limbs: [
            (limb: RightThigh, weight: 1.0, angle: 0, tolerance: 0),
            (limb: RightShin, weight: 1.0, angle: -1, tolerance: 0),
            (limb: RightFoot, weight: 1.0, angle: 9, tolerance: 0),
          ],
          z: (
            absolute: {(side: Left, part: Knee): -1.0, (side: Right, part: Heel): 1.0},
          ),
        ),
      ],
      steps: [
        (
          name: "Run in place",
          id: "run-in-place",
          variation: "left-first",
          keyframes: [
            (pose: "in-place-left-up", orientation: ToCamera),
            (pose: "in-place-right-up", orientation: ToCamera),
          ],
        ),
        (
          name: "Another step for testing",
          id: "another-step",
          keyframes: [
            (pose: "in-place-left-up", orientation: ToCamera),
            (pose: "in-place-left-up", orientation: ToCamera),
            (pose: "in-place-right-up", orientation: ToCamera),
            (pose: "in-place-right-up", orientation: ToCamera),
          ],
        ),
      ],
    )
    "#;

    const COURSE_STR_1: &str = r#"
    #![enable(implicit_some)]
    (
      version: 0,
      id: "test-meta",
      names: {"de": "Running Man Anfängerkurs", "en": "Running Man beginner's course"},
      featured_step: "rm-0",
      video: "https://app.bouncy-feet.ch/media/videos/c3/overview.mp4",
      lessons: [
        (
          names: {"de": "Micro Bounce", "en": "Micro Bounce"},
          difficulty: 2,
          energy: 3,
          song: "105bpm_tropical_house",
          song_timestamp: 1500,
          explainer_video: "https://app.bouncy-feet.ch/media/videos/c3/combo_fast.mp4",
          front_video: "https://app.bouncy-feet.ch/media/videos/c3/combo_fast_front_view.mp4",
          back_video: "https://app.bouncy-feet.ch/media/videos/c3/combo_fast_back_view.mp4",
          parts: [
            (step: "run-in-place", repeat: 1, subbeats_per_move: 1, tracking: Untracked),
          ],
        ),
      ],
      poses: [
        (
          id: "in-place-right-up",
          direction: Front,
          limbs: [
            (limb: LeftThigh, weight: 1.0, angle: -1, tolerance: 0),
            (limb: LeftShin, weight: 1.0, angle: 3, tolerance: 0),
            (limb: LeftFoot, weight: 1.0, angle: -7, tolerance: 0),
          ],
          z: (
            absolute: {(side: Right, part: Knee): -1.0, (side: Left, part: Heel): 1.0},
          ),
        ),
        (id: "in-place-left-up", mirror_of: "in-place-right-up", direction: Front),
      ],
      steps: [
        (
          name: "Run in place",
          id: "run-in-place",
          variation: "left-first",
          keyframes: [
            (pose: "in-place-left-up", orientation: ToCamera),
            (pose: "in-place-right-up", orientation: ToCamera),
          ],
        ),
      ],
    )
    "#;

    #[test]
    fn test_valid_pose_reference() {
        load_pose_str(POSE_STR).unwrap();
        load_step_str(STEP_STR, "test".to_owned()).unwrap();
        let num_poses = STATE.with_borrow(|state| state.global_db.tracker_view.poses().len());
        assert_eq!(num_poses, 2);
    }

    #[test]
    fn test_basic_step_loading() {
        load_pose_str(POSE_STR).unwrap();
        load_step_str(STEP_STR, "test".to_owned()).unwrap();

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
        load_step_str(STEP_STR, "test".to_owned()).unwrap();
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
        load_step_str(STEP_STR, "test".to_owned()).unwrap();
        match load_dance_str(input) {
            Err(ParseFileError::UnknownStepName(id)) if id == "fake-id" => (),
            Err(other) => panic!("wrong error {other}"),
            Ok(()) => panic!("expected an error when loading invalid reference"),
        }
    }

    #[test]
    fn test_basic_course_loading() {
        let en_course = parse_course_str(COURSE_STR, "en").unwrap();
        expect![[r#"
            Course {
                id: "running-man-basics",
                name: "Running Man beginner's course",
                explanation: None,
                featured_step_id: "rm-0",
                video: None,
                lessons: [
                    Lesson {
                        name: "Micro Bounce",
                        explanation: None,
                        explainer_video: None,
                        front_video: None,
                        back_video: None,
                        song: None,
                        song_timestamp: None,
                        energy: 3,
                        difficulty: 2,
                        parts: [
                            LessonPart {
                                step_name: "run-in-place",
                                repeat: 1,
                                pace: StepPace {
                                    subbeats_per_pose: 1,
                                },
                                tracking: Tracked,
                            },
                            LessonPart {
                                step_name: "another-step",
                                repeat: 2,
                                pace: StepPace {
                                    subbeats_per_pose: 4,
                                },
                                tracking: Tracked,
                            },
                        ],
                    },
                ],
                collection: ContentCollection {
                    poses(len): 2,
                    steps(len/source): [
                        "course: 2",
                    ],
                    dances(len): 0,
                    tracker_view: "DanceCollection { limbs: (10): [\"LeftThigh\", \"LeftShin\", \"LeftFoot\", \"LeftArm\", \"LeftForearm\", \"RightThigh\", \"RightShin\", \"RightFoot\", \"RightArm\", \"RightForearm\"], poses(2): [\"in-place-right-up\", \"in-place-left-up\"], steps(2): [\"Run in place\", \"Another step for testing\"], dances(0): []}",
                },
            }
        "#]]
        .assert_debug_eq(&en_course);
    }

    #[test]
    fn test_basic_course_loading_with_more_meta_data() {
        let en_course = parse_course_str(COURSE_STR_1, "en").unwrap();
        expect![[r#"
            Course {
                id: "test-meta",
                name: "Running Man beginner's course",
                explanation: None,
                featured_step_id: "rm-0",
                video: Some(
                    "https://app.bouncy-feet.ch/media/videos/c3/overview.mp4",
                ),
                lessons: [
                    Lesson {
                        name: "Micro Bounce",
                        explanation: None,
                        explainer_video: Some(
                            "https://app.bouncy-feet.ch/media/videos/c3/combo_fast.mp4",
                        ),
                        front_video: Some(
                            "https://app.bouncy-feet.ch/media/videos/c3/combo_fast_front_view.mp4",
                        ),
                        back_video: Some(
                            "https://app.bouncy-feet.ch/media/videos/c3/combo_fast_back_view.mp4",
                        ),
                        song: Some(
                            "105bpm_tropical_house",
                        ),
                        song_timestamp: Some(
                            1500.0,
                        ),
                        energy: 3,
                        difficulty: 2,
                        parts: [
                            LessonPart {
                                step_name: "run-in-place",
                                repeat: 1,
                                pace: StepPace {
                                    subbeats_per_pose: 1,
                                },
                                tracking: Untracked,
                            },
                        ],
                    },
                ],
                collection: ContentCollection {
                    poses(len): 2,
                    steps(len/source): [
                        "course: 1",
                    ],
                    dances(len): 0,
                    tracker_view: "DanceCollection { limbs: (10): [\"LeftThigh\", \"LeftShin\", \"LeftFoot\", \"LeftArm\", \"LeftForearm\", \"RightThigh\", \"RightShin\", \"RightFoot\", \"RightArm\", \"RightForearm\"], poses(2): [\"in-place-right-up\", \"in-place-left-up\"], steps(1): [\"Run in place\"], dances(0): []}",
                },
            }
        "#]]
        .assert_debug_eq(&en_course);
    }

    #[test]
    fn test_missing_step_course_loading() {
        let input = r#"
          #![enable(implicit_some)]
          (
            version: 0,
            id: "running-man-basics",
            names: {
              "de": "Running Man Anfängerkurs",
              "en": "Running Man beginner's course",
            },
            featured_step: "rm-0",
            lessons: [
              (
                names: {
                  "de": "Micro Bounce",
                  "en": "Micro Bounce",
                },
                difficulty: 1,
                energy: 2,
                parts: [
                  (step: "run-in-place", repeat: 1, subbeats_per_move: 1),
                  (step: "micro-bounce", repeat: 2, subbeats_per_move: 4),
                ]
              ),
            ],
            poses: [],
            steps: [],
          )
        "#;

        let result = parse_course_str(input, "en");
        assert!(matches!(result, Err(ParseFileError::UnknownStepName(_))));
    }
}
