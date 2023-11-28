use crate::pose::Limb;
use crate::Keypoints;
use wasm_bindgen::prelude::wasm_bindgen;

/// A self-sufficient description of a body position snapshot.
///
/// This format is for exporting to other modules. JS code can easily read it
/// and potentially render it.
///
/// Note that the skeleton is stripped of position information, it only has
/// angles of all body parts. This means it cannot be used to overlay a video.
/// Use the original keypoints for such matters.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Skeleton {
    pub left: Side,
    pub right: Side,
}

#[wasm_bindgen(js_name = SkeletonSide)]
#[derive(Clone, Copy, Debug)]
pub struct Side {
    pub thigh: f32,
    pub shin: f32,
    // TODO: add all other renderable limbs (requires adding keypoints, too)
}

/// A normalized representation of a body position snapshot, including all tracked information.
///
/// This format is optimal for comparisons against many different poses.
///
/// Keypoints can be converted to a SkeletonInfo object and then compared to poses.
///
/// Note that the a SkeletonInfo object needs a list of limb definitions to be meaningful.
pub(crate) struct SkeletonInfo {
    /// A list of angles of the skeleton.
    ///
    /// Which angles are included depends on the poses we want to detect.
    limb_angles: Vec<f32>,
    /// Which direction the skeleton faces, as stored.
    ///
    /// This must never be West, because standardization flips it to East.
    direction: Direction,
    /// In which direction the original keypoints were facing
    original_direction: Direction,
}

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    /// Looking at the camera (0° azimuth)
    North,
    /// Looking to the left as seen in a non-mirrored video, which is to the
    /// right-hand-side of the dancer. (90° azimuth)
    East,
    /// Looking away from the camera (+/-180° azimuth)
    South,
    /// Looking to the right as seen in a non-mirrored video, which is to the
    /// left-hand-side of the dancer. (-90° azimuth)
    West,
    /// For when we don't know the direction
    Unknown,
}

impl SkeletonInfo {
    pub(crate) fn from_keypoints(kp: &Keypoints) -> Self {
        let mut limb_angles =
            super::STATE.with(|state| state.borrow().db.angles_from_keypoints(kp));
        // For now, use only shoulder to figure out direction.
        let shoulder_angle = kp.left.shoulder.azimuth(kp.right.shoulder);
        let original_direction = match shoulder_angle {
            alpha if alpha <= 45.0 && alpha >= -45.0 => Direction::West,
            alpha if alpha < 135.0 && alpha > 45.0 => Direction::North,
            alpha if alpha <= -135.0 || alpha >= 135.0 => Direction::East,
            alpha if alpha < -45.0 && alpha > -135.0 => Direction::South,
            _ => Direction::Unknown,
        };
        let mut direction = original_direction;

        // flip x-axis if the dancer faces west, to make angles comparable to east-facing poses
        if original_direction == Direction::West {
            direction = Direction::East;
            // fow now, all limb angles are x-direction against y-axis, so flipping is easy
            for angle in &mut limb_angles {
                *angle *= -1.0;
            }
        }
        Self {
            limb_angles,
            original_direction,
            direction,
        }
    }

    pub(crate) fn angles(&self) -> &[f32] {
        &self.limb_angles
    }

    pub(crate) fn to_skeleton(&self) -> Skeleton {
        let mut left = Side {
            thigh: self.limb_angles[Limb::LEFT_THIGH],
            shin: self.limb_angles[Limb::LEFT_SHIN],
        };
        let mut right = Side {
            thigh: self.limb_angles[Limb::RIGHT_THIGH],
            shin: self.limb_angles[Limb::RIGHT_SHIN],
        };

        if self.original_direction != self.direction {
            left.flip_x();
            right.flip_x();
        }
        Skeleton { left, right }
    }
}

impl Side {
    fn flip_x(&mut self) {
        self.thigh *= -1.0;
        self.shin *= -1.0;
    }
}
