# Challenge solution keypoint fixtures

These fixtures are regression samples for a **specific challenge solution video
and its checked-in challenge RON**. Unlike `test_poses` and `test_steps`, they
must not be reused as generic pose/step samples.

## Layout and provenance

The deterministic path is:

```text
test_challenges/<challenge-set-id>/<challenge-id>.ron
```

The exporter writes `<challenge-id>.metadata.json` beside each fixture. It
records the source video URL, UTC capture date, inclusive selected range, frame
count, MediaPipe package/WASM versions, and pose model. Keep both files in the
same commit. Timestamps in RON are the original video milliseconds so BPM
regressions remain visible.

Range semantics are inclusive `[startMs, endMs]`. By default, export begins at
`solution_start_timestamp` and ends at the final configured beat. If a video has
only one beat, it ends at the end of the video. Authors should use `--start` and
`--end` deliberately when a source has lead-in/outro, then review the exact
fixture against the configured `parts`.

Challenge metadata follows one alignment rule: there is exactly one `Start`
marker and `solution_start_timestamp` equals it. `/dev/challenge-author` emits
this form, the exporter uses it, and the Rust test enforces it.

## Generate or refresh

Prerequisites: Chromium installed for Playwright, frontend dependencies and the
Rust/WASM instructor bindings built, network/CORS access to the solution video,
and a frontend dev server running from the repository root:

```sh
npm --prefix bouncy_frontend run dev
```

In another terminal:

```sh
# Generate only missing fixtures (the default).
npm --prefix bouncy_frontend run export:challenge-keypoints -- \
  bouncy_frontend/src/lib/assets/challenges/jumping-jacks.ron

# Refresh every fixture, or one selected fixture.
npm --prefix bouncy_frontend run export:challenge-keypoints -- \
  bouncy_frontend/src/lib/assets/challenges/jumping-jacks.ron --all
npm --prefix bouncy_frontend run export:challenge-keypoints -- \
  bouncy_frontend/src/lib/assets/challenges/jumping-jacks.ron \
  --challenge jumping-jacks --all
```

The command is author tooling, not part of `cargo test` or CI. It uses the
application's compiled Rust/WASM parser and production `PoseDetection` →
`landmarksToKeypoints` → `Tracker.addKeypoints` → `Tracker.exportKeypoints`
pipeline. It fails rather than writing an empty fixture on video, CORS, or
no-person-detected errors. Review and commit generated files, then run:

```sh
cargo test -p bouncy_instructor --test challenge_tests
cargo test -p bouncy_instructor
```
