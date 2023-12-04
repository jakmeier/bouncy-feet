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
///
/// The coordinate system is growing down (y-axis), right (x-axis), and away
/// from the camera (z-axis).
///
/// See the Keypoints section in bouncy_instructor/coordinates.md for visuals
/// and rationale.
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
        // only the sign of dy matters here, and we want it to grow down when we
        // use acos to compute the polar angle
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
        SignedAngle::radian((dy / r).acos())
    }

    /// The azimuth is the clock-wise angle to the negative z-axis.
    ///
    /// The azimuth is between -180° and 180°. Someone facing the camera has an
    /// azimuth of 0°, which is also known as north.
    ///
    /// Returned values are in radian, (-PI to PI].
    ///
    /// Just like in cartography, east is +90° (PI/2) and west is -90° (-PI/2)
    /// for the dancer. However, in videos, the angles are therefore
    /// counter-clock-wise as seen by the camera.
    ///
    /// Note that in the keypoint coordinate system, the x-axis grows to the
    /// right. In a (non-mirrored) video this means we see the left arm of the
    /// dance (west) in the positive x-direction. Which is the opposite of how
    /// angles grow in our spherical coordinates. Also confusing, the positive
    /// z-axis faces south, not north.
    pub(crate) fn azimuth(&self, other: Cartesian3d) -> SignedAngle {
        // usually you should expect other - self, but we need to flip both signs
        let dz = self.z - other.z;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::assert_angle_eq;

    #[test]
    fn test_cartesian_to_angle() {
        // input, azimuth, polar

        // in keypoint coordinates, the negative x direction is the right hand
        // of the dancer, which is the positive angle direction
        check_cartesian_to_angle(Cartesian3d::new(1.0, 0.0, 0.0), -90.0, 90.0);
        check_cartesian_to_angle(Cartesian3d::new(-1.0, 0.0, 0.0), 90.0, 90.0);

        // down is 0° (and camera y grows down)
        check_cartesian_to_angle(Cartesian3d::new(0.0, 1.0, 0.0), 0.0, 0.0);
        // up is 180°
        check_cartesian_to_angle(Cartesian3d::new(0.0, -1.0, 0.0), 0.0, 180.0);

        // away from the camera means south => azimuth = 180°
        check_cartesian_to_angle(Cartesian3d::new(0.0, 0.0, 1.0), 180.0, 90.0);
        // to the camera means north => azimuth = 0
        check_cartesian_to_angle(Cartesian3d::new(0.0, 0.0, -1.0), 0.0, 90.0);
    }

    #[track_caller]
    fn check_cartesian_to_angle(
        cartesian: Cartesian3d,
        expected_azimuth: f32,
        expected_polar: f32,
    ) {
        let origin = Cartesian3d::new(0.0, 0.0, 0.0);
        assert_angle_eq(
            SignedAngle::degree(expected_azimuth),
            origin.azimuth(cartesian),
        );
        assert_angle_eq(
            SignedAngle::degree(expected_polar),
            origin.polar_angle(cartesian),
        );
    }
}
