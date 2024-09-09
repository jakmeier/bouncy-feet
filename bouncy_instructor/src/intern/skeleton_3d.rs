use super::dance_collection::{DanceCollection, LimbIndex};
use super::geom::{Angle3d, SignedAngle};
use super::pose::{BodyPoint, Limb};
use crate::keypoints::Cartesian3d;
use crate::skeleton::{Segment, Side, Skeleton};
use crate::Keypoints;
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
    /// Degrees of turned shoulder from base direction. More than 45° won't
    /// work, use different direction instead.
    pub(crate) turn_shoulder: SignedAngle,
    /// Degrees of turned hip from base direction.
    pub(crate) turn_hip: SignedAngle,
    /// The angle between the standardized direction as stored (East) and what
    /// was recorded.
    azimuth_correction: SignedAngle,
    /// Z position estimates of body parts
    coordinates: HashMap<BodyPoint, Cartesian3d>,
    /// Z position estimates of limbs
    limbs_z: Vec<f32>,
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
        limbs_z: Vec<f32>,
        turn_shoulder: SignedAngle,
        turn_hip: SignedAngle,
        azimuth_correction: SignedAngle,
        coordinates: HashMap<BodyPoint, Cartesian3d>,
    ) -> Self {
        let limb_angles = limb_angles_3d.iter().map(Angle3d::project_2d).collect();
        Self {
            direction,
            limb_angles,
            limb_angles_3d,
            limbs_z,
            turn_shoulder,
            turn_hip,
            azimuth_correction,
            coordinates,
        }
    }

    pub(crate) fn from_keypoints(kp: &Keypoints, db: &DanceCollection) -> Self {
        let limb_angles_3d = db
            .limbs()
            .map(|(_index, limb)| limb.to_angle(kp))
            .collect::<Vec<_>>();
        let shoulder_angle = kp.left.shoulder.azimuth(kp.right.shoulder);
        let hip_angle = kp.left.hip.azimuth(kp.right.hip);
        let pos: HashMap<_, _> = kp.body_points().collect();
        let limbs_z = db
            .limbs()
            .map(|(_index, limb)| limb.z(kp))
            .collect::<Vec<_>>();
        Self::from_angles(limb_angles_3d, shoulder_angle, hip_angle, pos, limbs_z)
    }

    pub(crate) fn from_angles(
        mut limb_angles_3d: Vec<Angle3d>,
        shoulder_angle: SignedAngle,
        hip_angle: SignedAngle,
        pos: HashMap<BodyPoint, Cartesian3d>,
        limbs_z: Vec<f32>,
    ) -> Self {
        // Shoulder defines where the person is looking
        let direction = Direction::from_shoulder(shoulder_angle);
        // This means, by definition, turn shoulder is zero
        let turn_shoulder = SignedAngle::ZERO;
        let turn_hip = hip_angle - shoulder_angle;

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

        Self::new(
            direction,
            limb_angles_3d,
            limbs_z,
            turn_shoulder,
            turn_hip,
            azimuth_correction,
            pos,
        )
    }

    pub(crate) fn angles(&self) -> &[SignedAngle] {
        &self.limb_angles
    }

    /// Undo the normalization done when the skeleton was added.
    ///
    /// For comparing to camera-facing poses, this is better. Otherwise,
    /// slightly west looking skeletons will have messed up all angles by 180°.
    pub(crate) fn original_angles(&self) -> Vec<SignedAngle> {
        self.limb_angles_3d
            .iter()
            .cloned()
            .map(|mut angle| {
                angle.azimuth = angle.azimuth + self.azimuth_correction;
                angle
            })
            .map(|angle| angle.project_2d())
            .collect()
    }

    pub(crate) fn positions(&self) -> &HashMap<BodyPoint, Cartesian3d> {
        &self.coordinates
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
            let mut s: Segment = Angle3d::new(
                self.limb_angles_3d[i.as_usize()].azimuth + correction,
                self.limb_angles_3d[i.as_usize()].polar,
            )
            .into();

            let stretched_z = -self.limbs_z[i.as_usize()] * 100.0;
            s.z = stretched_z.min(i16::MAX as f32).max(i16::MIN as f32) as i16;
            s
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

        let base_dir = direction.angle();
        let shoulder: Segment = Angle3d::new(
            base_dir + self.turn_shoulder + correction,
            SignedAngle::degree(90.0),
        )
        .into();

        let hip: Segment = Angle3d::new(
            base_dir + self.turn_hip + correction,
            SignedAngle::degree(90.0),
        )
        .into();

        Skeleton {
            left,
            right,
            shoulder,
            hip,
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

    fn angle(&self) -> SignedAngle {
        match self {
            Direction::North => SignedAngle::degree(90.0),
            Direction::East => SignedAngle::ZERO,
            Direction::South => SignedAngle::degree(-90.0),
            Direction::West => SignedAngle::degree(-180.0),
            Direction::Unknown => SignedAngle::ZERO,
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
        let skeleton = Skeleton3d::from_keypoints(&kp, &Default::default());
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
            fully_visible: true,
        }
    }
}
