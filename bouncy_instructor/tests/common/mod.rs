//! Common code for Rust-style integration tests.

use bouncy_instructor::{load_pose_str, Tracker};

pub(crate) fn setup_tracker() -> Tracker {
    const POSE_STR: &str = include_str!("../data/pose.ron");
    load_pose_str(POSE_STR).expect("loading poses");
    Tracker::new()
}
