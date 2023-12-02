//! This module contains types for `Keypoints`, which are the main input for the
//! instructor.

use crate::intern::geom::SignedAngle;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Keypoints {
    pub left: Side,
    pub right: Side,
}

#[wasm_bindgen(js_name = KeypointsSide)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
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
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
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
    pub fn new(left: Side, right: Side) -> Self {
        Self { left, right }
    }
}

#[wasm_bindgen(js_class = KeypointsSide)]
impl Side {
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

impl Cartesian3d {
    /// The polar angle is measured against the y-axis, which goes from the
    /// ground to the sky.
    ///
    /// The polar angle is between 0° and +180°, with 0° pointing to
    /// the ground, 180° to the sky.
    ///
    /// Returned values are in radian, hence [0, PI]
    pub(crate) fn polar_angle(&self, other: Cartesian3d) -> SignedAngle {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let dz = other.z - self.z;

        let r = (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt();
        if !r.is_normal() {
            // Handle vectors of lengths very close to zero, NaN, or infinity.
            // Returning 0° is as good as any other angle.
            return SignedAngle(0.0);
        }
        // note: potentially this could be computed more efficiently
        // note 2: what about Math.acos() instead of wasm ?
        SignedAngle((dy / r).acos())
    }

    /// The azimuth is the clock-wise angle to the negative z-axis.
    ///
    /// The azimuth is between -180° and 180°. Someone facing the camera has an
    /// azimuth of 0°, which is also known as north.
    ///
    /// Returned values are in radian, hence [-PI to PI].
    ///
    /// Just like in cartography, east is +90° and west is -90° for the dancer.
    /// However, in mirrored videos, the angles are therefore counter-clock-wise
    /// as seen by the viewer.
    ///
    /// Note that in the keypoint coordinate system, the positive z-axis faces
    /// south, not north. But since the camera coordinate system should not be
    /// of interest beyond the translation to angles, this fact rarely matters.
    pub(crate) fn azimuth(&self, other: Cartesian3d) -> SignedAngle {
        let dz = other.z - self.z;
        // this reversal makes the angles clock-wise
        let dx = self.x - other.x;
        let r = dx.hypot(dz);
        if !r.is_normal() {
            // Handle vectors of lengths very close to zero, NaN, or infinity.
            // Returning 0° is as good as any other angle.
            return SignedAngle(0.0);
        }
        // note: potentially this could be computed more efficiently, esp. the sign
        // note 2: what about Math.acos() instead of wasm ?
        SignedAngle(dx.signum() * (dz / r).acos())
    }
}
