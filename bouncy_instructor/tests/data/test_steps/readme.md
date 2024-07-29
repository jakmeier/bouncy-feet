# Test steps

This folder contains test input data for step detection tests.


## What are step detection tests?

These are regression tests to ensure future addition or changes won't invalidate
existing steps. It's done on a `Tracker` object basis, which takes times
keypoints as inputs and produces a detected dance as result.

Every step ready for users to track themselves should contain at least one such a test.

To make the test setup fully end-to-end, it would be best to use videos as test
inputs. But to keep the test data size small, we instead use the keypoints
detected by mediapipe in a video.

Consequently, we would have to regenerate all test inputs if we swap out the
video to skeleton detection to a different library or algorithm. On the
positive, however, the tests only check the code in this repository, rather
than testing the mediapipe dependency as well.

In any case, for this testing setup, we need a video first. For now, we also
have to pre-process it once in a manual workflow. Video don't need to be
uploaded but it would be good if they are still retained somewhere, in case we
want to regenerate test input again in the future.


## Create a new test input

1. Record yourself doing the step, or find a video with a public domain license.
2. Cut the video to only include the step and make sure it is a clean
   performance.
3. Determine expected repetitions: Manually count how many full repetitions of
   the step are in the video.
4. Determine expected BPM: If you don't know the speed of the track, you can use
   this [online tool](https://www.all8.com/tools/bpm.htm) to estimate the BPM.
5. Open the dev page in the app, load the video file.
6. Wait for the video to be played to the end. No fast forward or skipping.
7. Click on "Download Keypoints of Video" and your browser should download a
   file bane "keypoints.ron".
8. Rename the file and put it in this folder.

Now you can create a test in `dance_tests.rs` that looks something like this.

```rs
#[test]
fn test_my_move() {
    let keypoints = include_str!("./data/test_steps/my-move.ron");
    check_single_step_in_keypoints(
        keypoints,
        "My Move",
        &[EXPECTED_REPETITIONS, EXPECTED_REPETITIONS-1],
        EXPECTED_BPM,
    );
}
```
`EXPECTED_REPETITIONS` and `EXPECTED_BPM` should have been determined in steps 3 and 4.