use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Keypoints {
    pub left: Side,
    pub right: Side,
}

#[wasm_bindgen(js_name = KeypointsSide)]
#[derive(Clone, Copy, Debug)]
pub struct Side {
    pub shoulder: Coordinate3d,
    pub hip: Coordinate3d,
    pub knee: Coordinate3d,
    pub ankle: Coordinate3d,
    pub heel: Coordinate3d,
    pub toes: Coordinate3d,
    pub elbow: Coordinate3d,
    pub wrist: Coordinate3d,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Coordinate3d {
    pub x: f32,
    pub y: f32,
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
        shoulder: Coordinate3d,
        hip: Coordinate3d,
        knee: Coordinate3d,
        ankle: Coordinate3d,
        heel: Coordinate3d,
        toes: Coordinate3d,
        elbow: Coordinate3d,
        wrist: Coordinate3d,
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
impl Coordinate3d {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl Coordinate3d {
    /// The polar angle is measured against the y-axis, which goes from the
    /// ground to the sky.
    ///
    /// The signed polar angle is between -180° and +180°, with 0° pointing to
    /// the ground. +90° goes towards east from the dancer perspective, which is
    /// (counter-intuitively) left in a non-mirrored video. But in a mirrored
    /// video, as usually used, it will be to the right.
    pub(crate) fn signed_polar_angle(&self, other: Coordinate3d) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        dx.atan2(dy) * 180.0 / std::f32::consts::PI
    }

    /// The azimuth is the clock-wise angle to the negative z-axis.
    ///
    /// The azimuth is between -180° and 180°. Someone facing the camera has an
    /// azimuth of 0°, which is also known as north.
    ///
    /// Just like in cartography, east is +90° and west is -90° for the dancer.
    /// However, in mirrored videos, the angles are therefore counter-clock-wise
    /// as seen by the viewer.
    ///
    /// Note that in the keypoint coordinate system, the positive z-axis faces
    /// south, not north. But since the camera coordinate system should not be
    /// of interest beyond the translation to angles, this fact rarely matters.
    pub(crate) fn azimuth(&self, other: Coordinate3d) -> f32 {
        let dz = other.z - self.z;
        // this reversal makes the usually ccw atan2 produce cw angles
        let dx = self.x - other.x;
        return (dx.atan2(dz) * 180.0) / std::f32::consts::PI;
    }
}
