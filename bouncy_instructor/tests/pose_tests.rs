//! Tests reading in single keypoint files and checking if a pose is detected.
//!
//! Using one test per position. This way, a CI run clearly shows how many poses
//! were detected wrong when things break.

use bouncy_instructor::Keypoints;

mod common;

/// Check that the expected pose is detected, given the choice of all standard poses.
fn check_pose_in_keypoints(keypoints: &str, expected_pose: &str) {
    let mut tracker = common::setup_tracker();
    let parsed: Vec<(u64, Keypoints)> = ron::from_str(keypoints).expect("parsing test input");
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
    assert_eq!(
        expected_pose,
        approximation.name(),
        "wrong pose detected {:?}",
        approximation
    );
    assert_eq!(timestamp, approximation.timestamp, "timestamp mangled");
    let threshold = 0.15;
    assert!(
        approximation.error < threshold,
        "correct pose but error is too big {}",
        approximation.error,
    );
}

/// Check that error score for a pose is above the threshold.
fn check_pose_not_in_keypoints(keypoints: &str, unexpected_pose: &str) {
    let mut tracker = common::setup_tracker();
    let parsed: Vec<(u64, Keypoints)> = ron::from_str(keypoints).expect("parsing test input");
    let (timestamp, keypoints) = parsed[0];
    tracker.add_keypoints(keypoints, timestamp);
    let pose = tracker
        .all_pose_errors(timestamp)
        .into_iter()
        .find(|approximation| approximation.name() == unexpected_pose)
        .expect("missing approximation for pose");

    for limb in pose.limb_errors() {
        println!(
            "    {:?}: {:?} x {:?}",
            limb.name(),
            limb.error,
            limb.weight
        );
    }
    let threshold = 0.1;
    assert!(
        pose.error > threshold,
        "pose error for {} is too small {}",
        unexpected_pose,
        pose.error,
    );
}

#[test]
fn test_left_forward_1() {
    let keypoints = include_str!("./data/test_poses/left_forward_1.keypoints.ron");
    check_pose_in_keypoints(keypoints, "left-forward");
}

#[test]
fn test_left_forward_2() {
    let keypoints = include_str!("./data/test_poses/left_forward_2.keypoints.ron");
    check_pose_in_keypoints(keypoints, "left-forward");
}

#[test]
fn test_left_up_1() {
    let keypoints = include_str!("./data/test_poses/left_up.keypoints.ron");
    check_pose_in_keypoints(keypoints, "left-up");
}

#[test]
fn test_standing_1() {
    let keypoints = include_str!("./data/test_poses/standing_east.keypoints.ron");
    check_pose_in_keypoints(keypoints, "standing-straight-side");
}

#[test]
fn test_standing_2() {
    let keypoints = include_str!("./data/test_poses/standing_front.keypoints.ron");
    check_pose_in_keypoints(keypoints, "standing-straight-front");
}

#[test]
fn test_standing_3() {
    let keypoints = include_str!("./data/test_poses/standing_west.keypoints.ron");
    check_pose_in_keypoints(keypoints, "standing-straight-side");
}

#[test]
fn test_standing_4() {
    let keypoints = include_str!("./data/test_poses/standing_front.keypoints.ron");
    check_pose_not_in_keypoints(keypoints, "standing-straight-side");
    check_pose_not_in_keypoints(keypoints, "right-forward");
    check_pose_not_in_keypoints(keypoints, "left-forward");
}

#[test]
fn test_right_forward_1() {
    let keypoints = include_str!("./data/test_poses/right_forward_1.keypoints.ron");
    check_pose_in_keypoints(keypoints, "right-forward");
}

#[test]
fn test_right_forward_2() {
    let keypoints = include_str!("./data/test_poses/right_forward_1.keypoints.ron");
    check_pose_not_in_keypoints(keypoints, "standing-straight-side");
    check_pose_not_in_keypoints(keypoints, "standing-straight-front");
}

#[test]
fn test_ready_to_jump_1() {
    let keypoints = include_str!("./data/test_poses/ready_to_jump_sideway.keypoints.ron");
    check_pose_in_keypoints(keypoints, "standing-straight-side");
}

#[test]
fn test_ready_to_jump_2() {
    let keypoints = include_str!("./data/test_poses/ready_to_jump_sideway.keypoints.ron");
    check_pose_not_in_keypoints(keypoints, "standing-straight-front");
    check_pose_not_in_keypoints(keypoints, "right-forward");
    check_pose_not_in_keypoints(keypoints, "left-forward");
}

#[test]
fn test_right_up_1() {
    let keypoints = include_str!("./data/test_poses/right_up_1.keypoints.ron");
    check_pose_in_keypoints(keypoints, "right-up");
}
