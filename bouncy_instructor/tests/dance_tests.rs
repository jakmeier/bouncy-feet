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
    // Start with the easy test: Find the step while only the step is tracked.
    check_step_in_keypoints_unique_tracker(keypoints, expected_step, expected_repetitions, bpm);

    // Now do the same but track any step.
    check_step_in_keypoints_general_tracker(keypoints, expected_step, expected_repetitions, bpm);
}

/// For when `check_single_step_in_keypoints` does not pass.
///
/// Getting steps to be detected properly is already challenging in the unique
/// step tracker. Before committing to correctly differentiate all steps, let's
/// start by making sure all steps pass this check.
fn check_step_in_keypoints_unique_tracker(
    keypoints: &str,
    expected_step: &str,
    expected_repetitions: &[usize],
    bpm: usize,
) {
    let parsed: Vec<(u64, Keypoints)> = ron::from_str(keypoints).expect("parsing test input");
    let mut step_tracker = common::setup_step_tracker(expected_step);
    step_tracker.set_bpm(2.0 * bpm as f32);
    for (timestamp, keypoints) in parsed {
        step_tracker.add_keypoints(keypoints, timestamp);
    }
    let detection = step_tracker.detect_dance();
    assert_step_detected(detection, expected_step, bpm, expected_repetitions);
}

fn check_step_in_keypoints_general_tracker(
    keypoints: &str,
    expected_step: &str,
    expected_repetitions: &[usize],
    bpm: usize,
) {
    let parsed: Vec<(u64, Keypoints)> = ron::from_str(keypoints).expect("parsing test input");
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

    let steps: Vec<_> = detection
        .steps()
        .iter()
        .filter(|s| !s.name().contains("Idle"))
        .cloned()
        .collect();
    assert!(
        !steps.is_empty(),
        "expected {expected_step} but only detected idle steps"
    );

    println!("(step, error, bpm):");
    for step in &steps {
        print!("({}, {}, {}), ", step.name(), step.error, step.bpm());
        assert_eq!(expected_step, step.name(), "wrong step detected {:?}", step);
    }
    println!();

    let threshold = 0.35;
    for step in &steps {
        assert!(
            step.error < threshold,
            "correct step but error is too big {}",
            step.error,
        );
        assert_bpm_about_equal(bpm, step.bpm());
    }

    let actual_repetitions = steps.len();
    assert!(
        expected_repetitions.contains(&actual_repetitions),
        "detected {actual_repetitions} step repetitions but expected one of {expected_repetitions:?}"
    );
}

#[track_caller]
fn assert_bpm_about_equal(expected: usize, actual: f32) {
    let ratio = actual / expected as f32;
    if ratio < 0.7 {
        panic!("detected BPM {actual} lower than expected {expected}")
    }
    if ratio > 1.3 {
        panic!("detected BPM {actual} higher than expected {expected}")
    }
}

// Put step detections tests below. They should be two-liners specifying the
// input and the expected output, calling checker function above. Make one test
// per input file to make it immediately obvious which detection failed in case
// of a failing test.

#[test]
fn test_running_man_0() {
    let keypoints = include_str!("./data/test_steps/running-man-100bpm-12x.ron");
    // FIXME: Also check in general tracker (detects double RM last time I tried)
    check_step_in_keypoints_unique_tracker(keypoints, "Running Man", &[12], 100);
}

#[test]
fn test_gangsta_hop() {
    let keypoints = include_str!("./data/test_steps/gangsta-hop-mid.ron");
    check_single_step_in_keypoints(keypoints, "Gangsta Hop", &[3, 4], 95);
}

#[test]
fn test_kicking_reverse_rm_0() {
    let keypoints = include_str!("./data/test_steps/kicking-reverse-rm-100bpm-15x.ron");
    // FIXME: Also check in general tracker (detects double RM last time I tried)
    check_step_in_keypoints_unique_tracker(keypoints, "Kicking Reverse RM", &[15], 120);
}

#[test]
fn test_reverse_rm_0() {
    let keypoints = include_str!("./data/test_steps/reverse-running-man-100bpm-11x.ron");
    // FIXME: Also check in general tracker (detects double RM last time I tried)
    check_step_in_keypoints_unique_tracker(keypoints, "Reverse RM", &[10, 11], 120);
}

#[test]
fn test_happy_feet_0() {
    let keypoints = include_str!("./data/test_steps/happy-feet-2x.ron");
    // FIXME: Also check in general tracker (detects V-Step last time I tried)
    check_step_in_keypoints_unique_tracker(keypoints, "Happy Feet", &[1, 2], 100);
}
