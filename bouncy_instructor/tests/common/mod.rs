#![allow(dead_code)]
//! Common code for Rust-style integration tests.

use bouncy_instructor::{load_dance_str, load_pose_str, load_step_str, Tracker};

pub(crate) fn load_static_files() {
    const POSE_STR: &str = include_str!("../data/pose.ron");
    const IDLE_STEP_STR: &str = include_str!("../data/steps/idle_steps.ron");
    const BASIC_STEP_STR: &str = include_str!("../data/steps/basic.ron");
    const FOOTWORK_STEP_STR: &str = include_str!("../data/steps/footwork.ron");
    const RM_STEP_STR: &str = include_str!("../data/steps/rm_variations.ron");
    const SHAPES_STEP_STR: &str = include_str!("../data/steps/shapes.ron");
    const MISC_STEP_STR: &str = include_str!("../data/steps/misc.ron");
    const DANCE_STR: &str = include_str!("../data/dance.ron");
    load_pose_str(POSE_STR).expect("loading static poses should work");
    for (i, (step_str, source)) in [
        (IDLE_STEP_STR, "idle_steps"),
        (BASIC_STEP_STR, "basic"),
        (FOOTWORK_STEP_STR, "footwork"),
        (RM_STEP_STR, "rm_variations"),
        (SHAPES_STEP_STR, "shapes"),
        (MISC_STEP_STR, "misc"),
    ]
    .iter()
    .enumerate()
    {
        load_step_str(step_str, source.to_string())
            .inspect_err(|_e| println!("failed in file {i} with input: {step_str}"))
            .expect("loading static steps should work");
    }
    load_dance_str(DANCE_STR).expect("loading static dances should work");
}

pub(crate) fn setup_tracker() -> Tracker {
    load_static_files();
    Tracker::new()
}
