//! TODO: this is copy-pasta from bouncy_instructor, I should probably use a shared lib for handling exported data etc
//!
//! This module contains types for `Keypoints`, which are the main input for the
//! instructor.

use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default)]
pub struct Keypoints {
    pub left: Side,
    pub right: Side,
}

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
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default)]
pub struct Cartesian3d {
    /// left-right direction
    pub x: f32,
    /// up-down direction
    pub y: f32,
    /// distance to camera
    pub z: f32,
}
