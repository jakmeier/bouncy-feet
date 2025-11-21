# Bouncy Lab

CLI utility to run video analysis locally, outside the web environment of the main app.

## Setup

Work is in progress here, don't expect this to "just run".

A rough sketch of what is needed to make this work:

- clone [google-ai-edge/mediapipe](https://github.com/google-ai-edge/mediapipe) at ../../
- patch file `mediapipe/tasks/c/vision/pose_landmarker/BUILD`
    - the build target "libpose_landmarker.so" must depend on ":pose_landmarker_lib" and ":pose_landmarker_c_lib"
- you need all dev tooling to build mediapipe (bazel, c++, computer vision dev libraries, a compatible python version, maybe more)
- in the mediapipe folder, build two .so files
    - `bazel build -c opt --linkopt -s --strip always //mediapipe/tasks/c/vision/pose_landmarker:pose_landmarker_c_lib`
    - `bazel build -c opt --linkopt -s --strip always //mediapipe/tasks/c/vision/pose_landmarker:libpose_landmarker.so`
- The path of the built .so files must be included for runtime linking
    - ```
      export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:`pwd`/../../mediapipe/bazel-bin/mediapipe/tasks/c/vision/pose_landmarker
      ```

## Usage

```rust
// List available commands
cargo run -- help
```