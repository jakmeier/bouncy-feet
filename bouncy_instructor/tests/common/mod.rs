#![allow(dead_code)]
//! Common code for Rust-style integration tests.

use bouncy_instructor::{load_pose_str, load_step_str, Tracker};

pub(crate) fn load_static_files() {
    const POSE_STR: &str = include_str!("../data/pose.ron");
    const STEP_STR: &str = include_str!("../data/step.ron");
    load_pose_str(POSE_STR).expect("loading poses");
    load_step_str(STEP_STR).expect("loading steps");
}

pub(crate) fn setup_tracker() -> Tracker {
    load_static_files();
    Tracker::new()
}
