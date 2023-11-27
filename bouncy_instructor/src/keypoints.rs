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
    ) -> Self {
        Self {
            shoulder,
            hip,
            knee,
            ankle,
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
