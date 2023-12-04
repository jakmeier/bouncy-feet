//! Tests reading in single keypoint files and checking if a pose is detected.
//!
//! Using one test per position. This way, a CI run clearly shows how many poses
//! were detected wrong when things break.

use bouncy_instructor::Keypoints;

mod common;

#[track_caller]
fn check_pose_in_keypoints(keypoints: &str, expected_pose: &str) {
    let mut tracker = common::setup_tracker();
    let parsed: Vec<(u32, Keypoints)> = ron::from_str(keypoints).expect("parsing test input");
    let (timestamp, keypoints) = parsed[0];
    tracker.add_keypoints(keypoints, timestamp);
    let approximation = tracker
        .best_fit_pose(timestamp, timestamp)
        .expect("no match found");

    for pose in tracker.all_pose_errors(timestamp) {
        println!("{}: {}", pose.name(), pose.error);
        for limb in pose.limb_errors() {
            println!(
                "    {:?}: {:?} x {:?}",
                limb.name(),
                limb.error,
                limb.weight
            );
        }
        println!();
    }
    assert_eq!(expected_pose, approximation.name(), "wrong pose detected");
    assert_eq!(timestamp, approximation.timestamp, "timestamp mangled");
    // TODO: this threshold should be much smaller, but for now I'm happy if tests just pass
    let threshold = 0.35;
    assert!(
        approximation.error < threshold,
        "correct pose but error is too big {}",
        approximation.error,
    );
}

#[test]
fn test_left_forward_1() {
    let keypoints = include_str!("./data/left_forward_1.keypoints.ron");
    check_pose_in_keypoints(keypoints, "left-forward");
}

#[test]
fn test_left_forward_2() {
    let keypoints = include_str!("./data/left_forward_2.keypoints.ron");
    check_pose_in_keypoints(keypoints, "left-forward");
}

#[test]
fn test_left_up_1() {
    let keypoints = include_str!("./data/left_up.keypoints.ron");
    check_pose_in_keypoints(keypoints, "left-up");
}

#[test]
fn test_standing_1() {
    let keypoints = include_str!("./data/standing_east.keypoints.ron");
    check_pose_in_keypoints(keypoints, "standing-straight");
}

#[test]
fn test_standing_2() {
    let keypoints = include_str!("./data/standing_front.keypoints.ron");
    check_pose_in_keypoints(keypoints, "standing-straight");
}

#[test]
fn test_standing_3() {
    let keypoints = include_str!("./data/standing_west.keypoints.ron");
    check_pose_in_keypoints(keypoints, "standing-straight");
}
