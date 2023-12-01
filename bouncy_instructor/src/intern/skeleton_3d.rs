use super::geom::{Angle3d, SignedAngle};
use super::pose::Limb;
use crate::skeleton::{Segment, Side, Skeleton};
use crate::{Keypoints, STATE};

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
    /// A list of angles of the skeleton.
    ///
    /// Which limbs are included depends on the poses we want to detect.
    limb_angles: Vec<Angle3d>,
    /// The angle between the standardized direction as stored (East) and what
    /// was recorded.
    azimuth_correction: SignedAngle,
}

impl Skeleton3d {
    pub(crate) fn from_keypoints(kp: &Keypoints) -> Self {
        let mut limb_angles = STATE.with(|state| state.borrow().db.angles_from_keypoints(kp));
        // Shoulder defines where he person is looking
        let shoulder_angle = kp.left.shoulder.azimuth(kp.right.shoulder);

        // Rotate skelton to face north.
        let azimuth_correction = shoulder_angle - SignedAngle::degree(90.0);
        for angle in &mut limb_angles {
            angle.azimuth = angle.azimuth - azimuth_correction;
        }

        Self {
            limb_angles,
            azimuth_correction,
        }
    }

    pub(crate) fn angles(&self) -> &[Angle3d] {
        &self.limb_angles
    }

    pub(crate) fn to_skeleton(&self) -> Skeleton {
        // TODO: add an option to NOT undo the normalization azimuth rotation
        let correction = self.azimuth_correction;
        let segment = |i: usize| -> Segment {
            Angle3d::new(
                self.limb_angles[i].azimuth + correction,
                self.limb_angles[i].polar,
            )
            .into()
        };

        let left = Side {
            thigh: segment(Limb::LEFT_THIGH),
            shin: segment(Limb::LEFT_SHIN),
            arm: segment(Limb::LEFT_ARM),
            forearm: segment(Limb::LEFT_FOREARM),
        };
        let right = Side {
            thigh: segment(Limb::RIGHT_THIGH),
            shin: segment(Limb::RIGHT_SHIN),
            arm: segment(Limb::RIGHT_ARM),
            forearm: segment(Limb::RIGHT_FOREARM),
        };

        Skeleton { left, right }
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
        assert_angle_3d_eq(Angle3d::ZERO, skeleton.limb_angles[pose::Limb::LEFT_THIGH]);
        assert_angle_3d_eq(Angle3d::ZERO, skeleton.limb_angles[pose::Limb::LEFT_SHIN]);
        assert_angle_3d_eq(
            Angle3d::degree(-90.0, 45.0),
            skeleton.limb_angles[pose::Limb::LEFT_ARM],
        );
        assert_angle_3d_eq(
            Angle3d::degree(-90.0, 45.0),
            skeleton.limb_angles[pose::Limb::LEFT_FOREARM],
        );
        assert_angle_3d_eq(Angle3d::ZERO, skeleton.limb_angles[pose::Limb::RIGHT_THIGH]);
        assert_angle_3d_eq(Angle3d::ZERO, skeleton.limb_angles[pose::Limb::RIGHT_SHIN]);
        assert_angle_3d_eq(
            Angle3d::degree(90.0, 45.0),
            skeleton.limb_angles[pose::Limb::RIGHT_ARM],
        );
        assert_angle_3d_eq(
            Angle3d::degree(90.0, 45.0),
            skeleton.limb_angles[pose::Limb::RIGHT_FOREARM],
        );
    }

    ///     O
    ///  /\   /\
    /// /  |_|  \
    ///    | |
    ///   _| |_
    fn straight_standing_keypoints() -> Keypoints {
        let kp = Keypoints {
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
        };
        kp
    }
}
