use bouncy_instructor::{DetectionResult, Keypoints};
use std::collections::HashSet;

mod common;

#[test]
fn test_unique_dance_id() {
    common::load_static_files();
    let steps = bouncy_instructor::dances();
    let mut unique_ids = HashSet::<String>::new();
    let mut duplicates = vec![];
    for step in &steps {
        if !unique_ids.insert(step.id()) {
            duplicates.push(step.id());
        }
    }

    assert!(
        !unique_ids.is_empty(),
        "No static dances found after loading."
    );
    assert!(
        steps.len() == unique_ids.len(),
        "Duplicate step ID detected. {duplicates:?}"
    )
}

/// Given a list of keyframes, run dance detection and check that the expected
/// step is in it `repetitions` times and has a bpm of around `bpm`.
///
/// This test checks both the general tracker with all steps, as well as a
/// tracker created with new UniqueStepTracker();.
fn check_single_step_in_keypoints(
    keypoints: &str,
    expected_step: &str,
    expected_repetitions: &[usize],
    bpm: usize,
) {
    let parsed: Vec<(u32, Keypoints)> = ron::from_str(keypoints).expect("parsing test input");

    // Start with the easy test: Find the step while only the step is tracked.
    let mut step_tracker = common::setup_step_tracker(expected_step);
    for (timestamp, keypoints) in parsed.clone() {
        step_tracker.add_keypoints(keypoints, timestamp);
    }
    let detection = step_tracker.detect_dance();
    assert_step_detected(detection, expected_step, bpm, expected_repetitions);

    // Now do the same but track any step.
    let mut tracker = common::setup_tracker();
    for (timestamp, keypoints) in parsed {
        tracker.add_keypoints(keypoints, timestamp);
    }
    let detection = tracker.detect_dance();
    assert_step_detected(detection, expected_step, bpm, expected_repetitions);
}

#[track_caller]
fn assert_step_detected(
    detection: DetectionResult,
    expected_step: &str,
    bpm: usize,
    expected_repetitions: &[usize],
) {
    assert!(
        !detection.steps().is_empty(),
        "expected {expected_step} but didn't detect any steps"
    );

    for step in detection.steps() {
        assert_eq!(expected_step, step.name(), "wrong step detected {:?}", step);
    }

    let threshold = 0.15;
    for step in detection.steps() {
        assert!(
            step.error < threshold,
            "correct step but error is too big {}",
            step.error,
        );
        assert_bpm_about_equal(bpm, step.bpm());
    }

    let actual_repetitions = detection.steps().len();
    assert!(
        expected_repetitions.contains(&actual_repetitions),
        "detected {actual_repetitions} step repetitions but expected one of {expected_repetitions:?}"
    );
}

#[track_caller]
fn assert_bpm_about_equal(expected: usize, actual: f32) {
    let ratio = actual / expected as f32;
    if ratio < 0.8 {
        panic!("detected BPM {actual} lower than expected {expected}")
    }
    if ratio > 1.2 {
        panic!("detected BPM {actual} higher than expected {expected}")
    }
}

#[test]
fn test_gangsta_hop() {
    let keypoints = include_str!("./data/test_steps/gangsta-hop-mid.ron");
    check_single_step_in_keypoints(keypoints, "Gangsta Hop", &[3, 4], 95);
}
