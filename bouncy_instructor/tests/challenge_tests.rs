use bouncy_instructor::{parse_challenge_str, Keypoints};
use std::fs;
use std::path::{Path, PathBuf};

const CHALLENGE_RON: &str =
    include_str!("../../bouncy_frontend/src/lib/assets/challenges/jumping-jacks.ron");
const ERROR_THRESHOLD: f32 = 0.35;

#[test]
fn jumping_jacks_solutions_match_their_configuration() {
    check_challenge_set(
        CHALLENGE_RON,
        "bouncy_frontend/src/lib/assets/challenges/jumping-jacks.ron",
    );
}

/// Replays every challenge fixture.
fn check_challenge_set(challenge_ron: &str, ron_path_from_crate: &str) {
    let set = parse_challenge_str(challenge_ron, "en")
        .expect("the checked-in challenge definition must parse");
    let challenges = set.challenges();
    assert!(!challenges.is_empty(), "challenge set must not be empty");

    for (index, challenge) in challenges.iter().enumerate() {
        let challenge_id = challenge.id();
        let video = challenge.solution_video();
        assert!(
            !video.path().trim().is_empty(),
            "challenge {challenge_id} must have an accessible solution-video path"
        );

        // Alignment rule: the authoring/export tools and tests use one exact
        // Start marker, and solution_start_timestamp must equal that marker.
        let start_markers = video.start_markers();
        assert_eq!(
            start_markers.len(),
            1,
            "challenge {challenge_id} must have exactly one Start marker"
        );
        assert_eq!(
            challenge.solution_start_timestamp(),
            start_markers[0],
            "challenge {challenge_id}: solution_start_timestamp must equal the Start marker"
        );

        let fixture = fixture_path(&set.id(), &challenge_id);
        let keypoints_ron = fs::read_to_string(&fixture).unwrap_or_else(|error| {
            panic!(
                "missing challenge fixture {} ({error}). From the repository root, generate it with:\n  npm --prefix bouncy_frontend run export:challenge-keypoints -- {} --challenge {}",
                fixture.display(),
                ron_path_from_crate,
                challenge_id,
            )
        });
        let keyframes: Vec<(f64, Keypoints)> = ron::from_str(&keypoints_ron)
            .unwrap_or_else(|error| panic!("invalid fixture {}: {error}", fixture.display()));
        assert!(
            !keyframes.is_empty(),
            "challenge fixture {} must not be empty",
            fixture.display()
        );
        assert!(
            keyframes.windows(2).all(|pair| pair[0].0 < pair[1].0),
            "challenge fixture {} timestamps must be strictly increasing",
            fixture.display()
        );

        let expected: Vec<(String, u8)> = challenge
            .parts()
            .into_iter()
            .flat_map(|part| {
                std::iter::repeat((part.step_name(), part.subbeats_per_move()))
                    .take(part.repeat() as usize)
            })
            .collect();
        let mut tracker = set
            .tracker(index)
            .unwrap_or_else(|| panic!("challenge {challenge_id} must have a tracker"));
        for (timestamp, keypoints) in keyframes {
            tracker.add_keypoints(keypoints, timestamp);
        }

        let detected: Vec<_> = tracker
            .detect_dance()
            .steps()
            .into_iter()
            .filter(|step| !step.name().contains("Idle"))
            .collect();
        let actual: Vec<_> = detected.iter().map(|step| step.id()).collect();
        let expected_ids: Vec<_> = expected.iter().map(|(id, _)| id.clone()).collect();
        println!(
            "challenge {challenge_id} detections: {:?}",
            detected
                .iter()
                .map(|step| (step.id(), step.start, step.end, step.error, step.bpm()))
                .collect::<Vec<_>>()
        );
        assert_eq!(
            actual, expected_ids,
            "challenge {challenge_id}: solution detections do not match configured parts"
        );
        for (step, (_, subbeats_per_move)) in detected.into_iter().zip(expected) {
            assert!(
                step.error < ERROR_THRESHOLD,
                "challenge {challenge_id}: pose error {} for step {} exceeds {}",
                step.error,
                step.id(),
                ERROR_THRESHOLD
            );
            assert_bpm_about_equal(
                challenge.bpm() as usize,
                step.bpm_with_subbeats_per_move(subbeats_per_move),
                &challenge_id,
            );
        }
    }
}

fn fixture_path(set_id: &str, challenge_id: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/test_challenges")
        .join(set_id)
        .join(format!("{challenge_id}.ron"))
}

#[track_caller]
fn assert_bpm_about_equal(expected: usize, actual: f32, challenge_id: &str) {
    let ratio = actual / expected as f32;
    assert!(
        (0.7..=1.3).contains(&ratio),
        "challenge {challenge_id}: detected BPM {actual} is not within 30% of configured BPM {expected}"
    );
}
