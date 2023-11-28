use crate::Keypoints;

/// A normalized representation of a specific body position snapshot.
///
/// Keypoints can be converted to a skeleton and then compared to poses.
/// Rendering a skeleton should also be possible, which leads to a standardized
/// character render. Of course, it won't match in an overlay to a video because
/// limb lengths are off. But for rendering independent character of different
/// physic this is perfect.
///
/// Note that the a skeleton needs a list of limb definitions to be meaningful.
pub(crate) struct Skeleton {
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

impl Skeleton {
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
}
