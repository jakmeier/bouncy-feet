use super::geom::{Angle3d, SignedAngle};
use super::pose::{BodyPoint, Limb};
use super::pose_db::LimbIndex;
use crate::skeleton::{Segment, Side, Skeleton};
use crate::{Keypoints, STATE};
use std::collections::HashMap;
use std::f32::consts::FRAC_PI_2;

/// A normalized representation of a body position snapshot, including all
/// tracked information.
///
/// This format is optimal for comparisons against many different poses.
///
/// Keypoints can be converted to a Skeleton3d object and then compared to
/// poses.
///
/// Note that the a Skeleton3d object needs a list of limb definitions to be
/// meaningful.
#[derive(Debug)]
pub(crate) struct Skeleton3d {
    /// Which direction the dance faces, before applying the azimuth correction.
    direction: Direction,
    /// A list of angles of the skeleton for comparison to poses.
    ///
    /// This is a 2D projection as seen from the camera.
    /// Which limbs are included depends on the poses we want to detect.
    limb_angles: Vec<SignedAngle>,
    /// A list of angles of the skeleton.
    ///
    /// Same as `limb_angles` but includes 3D information.
    /// However, 3D is less accurate, generally speaking.
    limb_angles_3d: Vec<Angle3d>,
    /// The angle between the standardized direction as stored (East) and what
    /// was recorded.
    azimuth_correction: SignedAngle,
    /// Z position estimates of body parts
    z: HashMap<BodyPoint, f32>,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub(crate) enum Direction {
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

impl Skeleton3d {
    pub(crate) fn new(
        direction: Direction,
        limb_angles_3d: Vec<Angle3d>,
        azimuth_correction: SignedAngle,
        z: HashMap<BodyPoint, f32>,
    ) -> Self {
        let limb_angles = limb_angles_3d.iter().map(Angle3d::project_2d).collect();
        Self {
            direction,
            limb_angles,
            limb_angles_3d,
            azimuth_correction,
            z,
        }
    }

    pub(crate) fn from_keypoints(kp: &Keypoints) -> Self {
        let limb_angles_3d = STATE.with(|state| {
            state
                .borrow()
                .db
                .limbs()
                .map(|(_index, limb)| limb.to_angle(kp))
                .collect::<Vec<_>>()
        });
        let shoulder_angle = kp.left.shoulder.azimuth(kp.right.shoulder);
        let z: HashMap<_, _> = kp
            .body_points()
            .map(|(body_point, coordinate)| (body_point, coordinate.z))
            .collect();
        Self::from_angles(limb_angles_3d, shoulder_angle, z)
    }

    pub(crate) fn from_angles(
        mut limb_angles_3d: Vec<Angle3d>,
        shoulder_angle: SignedAngle,
        z: HashMap<BodyPoint, f32>,
    ) -> Self {
        // Shoulder defines where the person is looking
        let direction = Direction::from_shoulder(shoulder_angle);

        // Rotate skelton to face either north or east.
        let mut azimuth_correction = SignedAngle::degree(0.0);
        match direction {
            Direction::West | Direction::South => {
                azimuth_correction = SignedAngle::degree(180.0);
                for angle in &mut limb_angles_3d {
                    angle.azimuth = angle.azimuth - azimuth_correction;
                }
            }
            _ => (),
        }

        Self::new(direction, limb_angles_3d, azimuth_correction, z)
    }

    pub(crate) fn angles(&self) -> &[SignedAngle] {
        &self.limb_angles
    }

    pub(crate) fn positions(&self) -> &HashMap<BodyPoint, f32> {
        &self.z
    }

    pub(crate) fn direction(&self) -> Direction {
        self.direction
    }

    pub(crate) fn to_skeleton(&self, rotation: f32) -> Skeleton {
        let direction = self.direction().rotate(SignedAngle::degree(rotation));
        let sideway = match direction {
            Direction::North | Direction::South | Direction::Unknown => false,
            Direction::East | Direction::West => true,
        };
        let correction = self.azimuth_correction - SignedAngle::degree(rotation);
        let backwards = correction.as_radians().abs() > FRAC_PI_2;
        let segment = |i: LimbIndex| -> Segment {
            Angle3d::new(
                self.limb_angles_3d[i.as_usize()].azimuth + correction,
                self.limb_angles_3d[i.as_usize()].polar,
            )
            .into()
        };

        let left = Side {
            thigh: segment(Limb::LEFT_THIGH),
            shin: segment(Limb::LEFT_SHIN),
            arm: segment(Limb::LEFT_ARM),
            forearm: segment(Limb::LEFT_FOREARM),
            foot: segment(Limb::LEFT_FOOT),
        };
        let right = Side {
            thigh: segment(Limb::RIGHT_THIGH),
            shin: segment(Limb::RIGHT_SHIN),
            arm: segment(Limb::RIGHT_ARM),
            forearm: segment(Limb::RIGHT_FOREARM),
            foot: segment(Limb::RIGHT_FOOT),
        };

        Skeleton {
            left,
            right,
            sideway,
            backwards,
        }
    }
}

#[allow(clippy::manual_range_contains)]
impl Direction {
    pub(crate) fn from_shoulder(shoulder_angle: SignedAngle) -> Self {
        match shoulder_angle.to_degrees() {
            alpha if alpha <= 45.0 && alpha >= -45.0 => Direction::West,
            alpha if alpha < 135.0 && alpha > 45.0 => Direction::North,
            alpha if alpha <= -135.0 || alpha >= 135.0 => Direction::East,
            alpha if alpha < -45.0 && alpha > -135.0 => Direction::South,
            _ => Direction::Unknown,
        }
    }

    fn rotate(&self, rotation: SignedAngle) -> Direction {
        let quarters = rotation.as_positive_radians() / FRAC_PI_2;
        let mut out = *self;
        for _ in 0..quarters.round() as usize {
            out = out.rotate_one();
        }
        out
    }

    fn rotate_one(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::Unknown => Direction::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::test_utils::*;

    use crate::intern::pose;
    use crate::intern::skeleton_3d::Skeleton3d;
    use crate::keypoints::Cartesian3d;

    #[test]
    fn test_keypoints_to_3d_skeleton() {
        let kp = straight_standing_keypoints();
        let skeleton = Skeleton3d::from_keypoints(&kp);
        assert_float_angle_eq(0.0, skeleton.azimuth_correction);
        assert_angle_3d_eq(
            Angle3d::ZERO,
            skeleton.limb_angles_3d[pose::Limb::LEFT_THIGH.as_usize()],
        );
        assert_angle_3d_eq(
            Angle3d::ZERO,
            skeleton.limb_angles_3d[pose::Limb::LEFT_SHIN.as_usize()],
        );
        assert_angle_3d_eq(
            Angle3d::degree(-90.0, 45.0),
            skeleton.limb_angles_3d[pose::Limb::LEFT_ARM.as_usize()],
        );
        assert_angle_3d_eq(
            Angle3d::degree(-90.0, 45.0),
            skeleton.limb_angles_3d[pose::Limb::LEFT_FOREARM.as_usize()],
        );
        assert_angle_3d_eq(
            Angle3d::ZERO,
            skeleton.limb_angles_3d[pose::Limb::RIGHT_THIGH.as_usize()],
        );
        assert_angle_3d_eq(
            Angle3d::ZERO,
            skeleton.limb_angles_3d[pose::Limb::RIGHT_SHIN.as_usize()],
        );
        assert_angle_3d_eq(
            Angle3d::degree(90.0, 45.0),
            skeleton.limb_angles_3d[pose::Limb::RIGHT_ARM.as_usize()],
        );
        assert_angle_3d_eq(
            Angle3d::degree(90.0, 45.0),
            skeleton.limb_angles_3d[pose::Limb::RIGHT_FOREARM.as_usize()],
        );
    }

    ///     O
    ///  /\   /\
    /// /  |_|  \
    ///    | |
    ///   _| |_
    fn straight_standing_keypoints() -> Keypoints {
        Keypoints {
            left: crate::keypoints::Side {
                shoulder: Cartesian3d::new(6.0, 1.0, 1.0),
                hip: Cartesian3d::new(5.0, 3.0, 1.0),
                knee: Cartesian3d::new(5.0, 4.0, 1.0),
                ankle: Cartesian3d::new(5.0, 5.0, 1.0),
                heel: Cartesian3d::new(5.0, 5.0, 1.0),
                toes: Cartesian3d::new(6.0, 5.0, 1.0),
                elbow: Cartesian3d::new(7.0, 2.0, 1.0),
                wrist: Cartesian3d::new(8.0, 3.0, 1.0),
            },
            right: crate::keypoints::Side {
                shoulder: Cartesian3d::new(2.0, 1.0, 1.0),
                hip: Cartesian3d::new(3.0, 3.0, 1.0),
                knee: Cartesian3d::new(3.0, 4.0, 1.0),
                ankle: Cartesian3d::new(3.0, 5.0, 1.0),
                heel: Cartesian3d::new(3.0, 5.0, 1.0),
                toes: Cartesian3d::new(2.0, 5.0, 1.0),
                elbow: Cartesian3d::new(1.0, 2.0, 1.0),
                wrist: Cartesian3d::new(0.0, 3.0, 1.0),
            },
        }
    }
}
