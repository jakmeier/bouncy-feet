//! This module contains types for `Keypoints`, which are the main input for the
//! instructor.

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default)]
pub struct Keypoints {
    pub left: Side,
    pub right: Side,
    #[serde(default = "true_fn")]
    #[wasm_bindgen(js_name = fullyVisible)]
    pub fully_visible: bool,
}

#[wasm_bindgen(js_name = KeypointsSide)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default)]
pub struct Side {
    pub shoulder: Cartesian3d,
    pub hip: Cartesian3d,
    pub knee: Cartesian3d,
    pub ankle: Cartesian3d,
    pub heel: Cartesian3d,
    pub toes: Cartesian3d,
    pub elbow: Cartesian3d,
    pub wrist: Cartesian3d,
}

/// Coordinate for Keypoints
///
/// The coordinate system is growing down (y-axis), right (x-axis), and away
/// from the camera (z-axis).
///
/// See the Keypoints section in bouncy_instructor/coordinates.md for visuals
/// and rationale.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default)]
pub struct Cartesian3d {
    /// left-right direction
    pub x: f32,
    /// up-down direction
    pub y: f32,
    /// distance to camera
    pub z: f32,
}

/// `Keypoints` define a specific position of a body.
///
/// This is the live input of the instructor which feeds from a camera or video.
/// They are normalized 3D coordinates of important body parts.
///
/// To evaluate `Keypoints`, we calculate angles and compare those to the `Pose`
/// list which is loaded dynamically.
#[wasm_bindgen]
impl Keypoints {
    #[wasm_bindgen(constructor)]
    pub fn new(left: Side, right: Side, fully_visible: bool) -> Self {
        Self {
            left,
            right,
            fully_visible,
        }
    }
}

#[wasm_bindgen(js_class = KeypointsSide)]
impl Side {
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(constructor)]
    pub fn new(
        shoulder: Cartesian3d,
        hip: Cartesian3d,
        knee: Cartesian3d,
        ankle: Cartesian3d,
        heel: Cartesian3d,
        toes: Cartesian3d,
        elbow: Cartesian3d,
        wrist: Cartesian3d,
    ) -> Self {
        Self {
            shoulder,
            hip,
            knee,
            ankle,
            heel,
            toes,
            elbow,
            wrist,
        }
    }
}

#[wasm_bindgen]
impl Cartesian3d {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

fn true_fn() -> bool {
    true
}
