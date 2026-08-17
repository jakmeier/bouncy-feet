use bouncy_instructor::{parse_challenge_str, Keypoints, MINIMUM_CHALLENGE_DURATION_MS};
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

#[test]
fn challenge_tracker_repeats_to_a_full_initial_score_window() {
    let set = parse_challenge_str(CHALLENGE_RON, "en").unwrap();
    let challenge = set.challenges().remove(0);
    let tracker = set.tracker(0).unwrap();

    assert_eq!(
        challenge.default_song().as_deref(),
        Some("105bpm_tropical_house")
    );
    assert_eq!(challenge.allowed_songs(), ["105bpm_tropical_house"]);
    assert_eq!(challenge.song_timestamp(), 0.0);

    // The score window is at least the configured minimum and ends on a
    // subbeat boundary. These assertions deliberately follow the shared
    // constant so a duration-policy change does not require fixture updates.
    let duration_ms = challenge.challenge_duration_ms();
    assert!(duration_ms >= MINIMUM_CHALLENGE_DURATION_MS);
    assert_eq!(
        duration_ms,
        tracker.tracked_subbeats() * 30_000 / challenge.bpm() as u32
    );
    assert_eq!(
        challenge.required_hits(),
        (tracker.expected_poses() * 60).div_ceil(100)
    );

    let configured_steps: u32 = challenge.parts().iter().map(|part| part.repeat()).sum();
    assert!(challenge.max_steps() >= configured_steps);
    assert_eq!(challenge.max_steps() % configured_steps, 0);
    assert_eq!(
        challenge.required_steps(),
        (challenge.max_steps() * 60).div_ceil(100)
    );

    // The teacher loops after that score window instead of completing.
    assert_eq!(
        tracker
            .cursor_at_subbeat(tracker.tracked_subbeats() as i32, false)
            .pose_index,
        0
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
                std::iter::repeat_n(
                    (part.step_name(), part.subbeats_per_move()),
                    part.repeat() as usize,
                )
            })
            .collect();
        let mut tracker = set
            .tracker(index)
            .unwrap_or_else(|| panic!("challenge {challenge_id} must have a tracker"));
        let detection_start = challenge.solution_start_timestamp();
        for (timestamp, keypoints) in keyframes {
            if timestamp >= detection_start {
                tracker.add_keypoints(keypoints, timestamp);
            }
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
            "challenge **{challenge_id}**: solution detections do not match configured parts"
        );
        for step in &detected {
            assert!(
                step.error < ERROR_THRESHOLD,
                "challenge **{challenge_id}**: pose error {} for step **{}** exceeds {}",
                step.error,
                step.id(),
                ERROR_THRESHOLD
            );
        }

        // Validate rhythm using only the known start and BPM. The detector is
        // deliberately not given the video's beat markers.
        assert_detected_starts_follow_tempo(
            detection_start,
            challenge.bpm() as usize,
            detected.iter().map(|step| step.start),
            &challenge_id,
        );
    }
}

fn fixture_path(set_id: &str, challenge_id: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/test_challenges")
        .join(set_id)
        .join(format!("{challenge_id}.ron"))
}

/// Ensure detected steps begin near monotonically advancing beats inferred
/// solely from the activity start and BPM.
fn assert_detected_starts_follow_tempo(
    activity_start: f64,
    configured_bpm: usize,
    starts: impl IntoIterator<Item = f64>,
    challenge_id: &str,
) {
    let beat_duration = 60_000.0 / configured_bpm as f64;
    let max_offset = beat_duration * 0.45;
    let mut previous_beat_index = None;

    for start in starts {
        let beat_index = ((start - activity_start) / beat_duration).round().max(0.0) as usize;
        let nearest_beat = activity_start + beat_index as f64 * beat_duration;
        let offset = (start - nearest_beat).abs();
        assert!(
            offset <= max_offset,
            "challenge {challenge_id}: detected step start {start} is {offset:.1}ms off the inferred beat {nearest_beat}"
        );
        assert!(
            previous_beat_index.is_none_or(|previous| beat_index > previous),
            "challenge {challenge_id}: detected step starts must advance through the inferred beat grid"
        );
        previous_beat_index = Some(beat_index);
    }
}

#[cfg(test)]
mod beat_grid_tests {
    use super::assert_detected_starts_follow_tempo;

    #[test]
    fn rejects_a_low_error_pose_match_that_starts_off_beat() {
        let result = std::panic::catch_unwind(|| {
            assert_detected_starts_follow_tempo(0.0, 100, [300.0], "off-beat-regression");
        });
        assert!(result.is_err());
    }

    #[test]
    fn accepts_step_starts_near_successive_beats() {
        assert_detected_starts_follow_tempo(0.0, 100, [25.0, 1_150.0], "on-beat-regression");
    }
}
