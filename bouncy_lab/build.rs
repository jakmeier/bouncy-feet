use std::path::PathBuf;

fn main() {
    let pose_landmarker_path =
        PathBuf::from("../../mediapipe/mediapipe/tasks/c/vision/pose_landmarker");
    let header = pose_landmarker_path.join("pose_landmarker.h");

    let so_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../mediapipe/bazel-bin/mediapipe/tasks/c/vision/pose_landmarker");

    // Tell Cargo to add this directory to linker search path
    println!(
        "cargo:rustc-link-search=native={}",
        so_path.to_str().unwrap()
    );

    // Tell Cargo which libraries to link
    // Note that these must be built separately
    println!("cargo:rustc-link-lib=dylib=pose_landmarker");

    // Tell Cargo to re-run if the header changes
    println!("cargo:rerun-if-changed={}", header.to_str().unwrap());

    // Generate Rust bindings
    let bindings = bindgen::Builder::default()
        .header(header.to_str().unwrap())
        .clang_arg("-I../../mediapipe")
        .clang_arg("-I/usr/include/c++/15")
        .clang_arg("-I/usr/include")
        .clang_arg("-xc++")
        .clang_arg("-std=c++17")
        // limit what is generated
        .allowlist_function("pose_landmarker.*")
        .allowlist_function("Mp.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("./src/generated");
    bindings
        .write_to_file(out_path.join("pose_bindings.rs"))
        .expect("Couldn't write bindings!");
}
