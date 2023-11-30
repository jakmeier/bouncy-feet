use crate::pose::Limb;
use crate::Keypoints;
use std::f32::consts::{FRAC_PI_2, PI, TAU};
use wasm_bindgen::prelude::wasm_bindgen;

/// A self-sufficient description of a body position snapshot for 2d rendering.
///
/// Each limb has a 2D angle in the x-y plane plus a length factor to simulate
/// the third dimension in a 2D projection.
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
    pub thigh: Segment,
    pub shin: Segment,
    pub arm: Segment,
    pub forearm: Segment,
    // torso?
    // head?
    // feet?
}

/// Projected segment, with a x-y angle and a length factor.
#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Segment {
    /// The 2D projected angle of the segment.
    pub angle: f32,
    /// the factor to multiply lengths when drawing the projected segment in 2D
    pub r: f32,
}

/// A normalized representation of a body position snapshot, including all tracked information.
///
/// This format is optimal for comparisons against many different poses.
///
/// Keypoints can be converted to a Skeleton3d object and then compared to poses.
///
/// Note that the a Skeleton3d object needs a list of limb definitions to be meaningful.
#[derive(Debug)]
pub(crate) struct Skeleton3d {
    /// A list of angles of the skeleton.
    ///
    /// Which limbs are included depends on the poses we want to detect.
    limb_angles: Vec<Angle3d>,
    /// In which direction the original keypoints were facing
    // original_direction: Direction,
    /// The angle between the standardized direction as stored (East) and what
    /// was recorded.
    azimuth_correction: SignedAngle,
}

/// A direction in 3D space.
#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) struct Angle3d {
    /// angle to z-axis, -PI to PI
    pub azimuth: SignedAngle,
    /// angle to y axis, -PI to PI
    pub polar: SignedAngle,
}

/// Represents angles from -PI to PI
#[derive(Clone, Copy, PartialEq)]
pub(crate) struct SignedAngle(pub(crate) f32);

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

impl Skeleton3d {
    pub(crate) fn from_keypoints(kp: &Keypoints) -> Self {
        let mut limb_angles =
            super::STATE.with(|state| state.borrow().db.angles_from_keypoints(kp));
        // Shoulder defines where he person is looking
        let shoulder_angle = kp.left.shoulder.azimuth(kp.right.shoulder);
        // let original_direction = match shoulder_angle {
        //     alpha if alpha <= 45.0 && alpha >= -45.0 => Direction::West,
        //     alpha if alpha < 135.0 && alpha > 45.0 => Direction::North,
        //     alpha if alpha <= -135.0 || alpha >= 135.0 => Direction::East,
        //     alpha if alpha < -45.0 && alpha > -135.0 => Direction::South,
        //     _ => Direction::Unknown,
        // };

        // Rotate skelton to face north.
        let azimuth_correction = shoulder_angle - SignedAngle::degree(90.0);
        for angle in &mut limb_angles {
            angle.azimuth = angle.azimuth - azimuth_correction;
        }

        Self {
            limb_angles,
            // original_direction,
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

impl Angle3d {
    pub(crate) fn new(azimuth: SignedAngle, polar: SignedAngle) -> Self {
        Self { azimuth, polar }
    }

    pub(crate) const ZERO: Self = Angle3d {
        azimuth: SignedAngle::ZERO,
        polar: SignedAngle::ZERO,
    };

    pub(crate) fn degree(azimuth: f32, polar: f32) -> Self {
        Self {
            azimuth: SignedAngle::degree(azimuth),
            polar: SignedAngle::degree(polar),
        }
    }

    pub(crate) fn radian(azimuth: f32, polar: f32) -> Self {
        Self {
            azimuth: SignedAngle(azimuth),
            polar: SignedAngle(polar),
        }
    }
}

impl SignedAngle {
    pub(crate) const ZERO: Self = SignedAngle(0.0);

    pub(crate) fn degree(alpha: f32) -> Self {
        Self(alpha * PI / 180.0).ensure_signed()
    }

    pub(crate) fn as_degree(&self) -> f32 {
        self.0 * 180.0 / PI
    }

    /// Returns a copy of the angle where values are guaranteed to be in [-PI and PI)
    #[inline]
    fn ensure_signed(self) -> Self {
        // using add -> mod -> sub avoids branching
        // unclear if it's worth it or not
        // no performance testing was done so far
        let alpha = (self.0 + PI) % TAU - PI;
        Self(alpha)
    }
}

impl From<Angle3d> for Segment {
    fn from(value: Angle3d) -> Self {
        let x = value.polar.sin() * value.azimuth.sin();
        let y = -value.polar.cos();
        let xy_len = x.hypot(y);
        let angle = if xy_len == 0.0 {
            0.0
        } else {
            (y.atan2(x) + FRAC_PI_2) % TAU
        };
        Self { angle, r: xy_len }
    }
}

impl std::ops::Deref for SignedAngle {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::Add for SignedAngle {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0).ensure_signed()
    }
}

impl std::ops::Sub for SignedAngle {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0).ensure_signed()
    }
}

impl std::fmt::Debug for SignedAngle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let alpha = self.as_degree();
        write!(f, "{alpha:.2}°")
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::{FRAC_1_SQRT_2, FRAC_PI_4};

    use crate::keypoints::Coordinate3d;
    use crate::pose;

    use super::*;

    /// Tests `SignedAngle::degree`
    ///
    /// The inputs of are in ° in [f32::MIN, f32::MAX]
    /// The internal representation must be in radians in [-PI, +PI).
    #[test]
    fn test_angle_degree_to_radian() {
        assert_float_angle_eq(0.0, SignedAngle::degree(0.0));
        assert_float_angle_eq(FRAC_PI_4, SignedAngle::degree(45.0));
        assert_float_angle_eq(-FRAC_PI_4, SignedAngle::degree(-45.0));
        assert_float_angle_eq(-FRAC_PI_4, SignedAngle::degree(315.0));
        assert_float_angle_eq(-PI, SignedAngle::degree(-180.0));
        assert_float_angle_eq(-PI, SignedAngle::degree(180.0));
    }

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
                shoulder: Coordinate3d::new(6.0, 1.0, 1.0),
                hip: Coordinate3d::new(5.0, 3.0, 1.0),
                knee: Coordinate3d::new(5.0, 4.0, 1.0),
                ankle: Coordinate3d::new(5.0, 5.0, 1.0),
                heel: Coordinate3d::new(5.0, 5.0, 1.0),
                toes: Coordinate3d::new(6.0, 5.0, 1.0),
                elbow: Coordinate3d::new(7.0, 2.0, 1.0),
                wrist: Coordinate3d::new(8.0, 3.0, 1.0),
            },
            right: crate::keypoints::Side {
                shoulder: Coordinate3d::new(2.0, 1.0, 1.0),
                hip: Coordinate3d::new(3.0, 3.0, 1.0),
                knee: Coordinate3d::new(3.0, 4.0, 1.0),
                ankle: Coordinate3d::new(3.0, 5.0, 1.0),
                heel: Coordinate3d::new(3.0, 5.0, 1.0),
                toes: Coordinate3d::new(2.0, 5.0, 1.0),
                elbow: Coordinate3d::new(1.0, 2.0, 1.0),
                wrist: Coordinate3d::new(0.0, 3.0, 1.0),
            },
        };
        kp
    }

    #[track_caller]
    fn check_angle_to_segment(
        (azimuth, polar): (f32, f32),
        (expected_angle, expected_len): (f32, f32),
    ) {
        let angle = Angle3d::degree(azimuth, polar);
        let actual: Segment = angle.into();
        let expected = Segment {
            angle: expected_angle / 180.0 * PI,
            r: expected_len,
        };
        assert!(
            float_eq(expected.angle, actual.angle),
            "{expected:?} == {actual:?}"
        );
        assert!(float_eq(expected.r, actual.r), "{expected:?} == {actual:?}");
    }

    /// Test conversion from internal angles (Angle3D) to exported Segments.
    ///
    /// Note that 0° in the exported format is the x-axis, as is common in mathematics.
    /// But in the internal format, 0° is down, along the y-axis.
    #[test]
    fn test_angle_to_segment() {
        // (azimuth, polar), (expected_angle, expected_len)
        check_angle_to_segment((0.0, 0.0), (0.0, 1.0));
        check_angle_to_segment((0.0, 90.0), (0.0, 0.0));
        check_angle_to_segment((90.0, 90.0), (90.0, 1.0));
        check_angle_to_segment((270.0, 90.0), (-90.0, 1.0));
        check_angle_to_segment((90.0, 45.0), (45.0, 1.0));
        check_angle_to_segment((45.0, 0.0), (0.0, 1.0));
        check_angle_to_segment((0.0, 45.0), (0.0, FRAC_1_SQRT_2));
    }

    #[track_caller]
    fn assert_float_angle_eq(expected: f32, actual: SignedAngle) {
        let radian = *actual;
        assert!(
            float_eq(expected, radian),
            "{expected} == {radian} ({actual:?})"
        );
    }

    #[track_caller]
    fn assert_angle_eq(expected: SignedAngle, actual: SignedAngle) {
        let expected_radian = *expected;
        let actual_radian = *actual;
        assert!(
            float_eq(expected_radian, actual_radian),
            "{expected_radian} == {actual_radian} ({expected:?} == {actual:?})"
        );
    }

    #[track_caller]
    fn assert_angle_3d_eq(expected: Angle3d, actual: Angle3d) {
        assert_angle_eq(expected.azimuth, actual.azimuth);
        assert_angle_eq(expected.polar, actual.polar);
    }

    #[track_caller]
    fn float_eq(expected: f32, actual: f32) -> bool {
        // first try strict equality
        if expected == actual {
            return true;
        }
        // special cases
        if expected.is_nan() || expected.is_infinite() {
            return expected == actual;
        }
        // next try relative equality, within a permille is equal enough
        if expected.abs() > 0.0 {
            // test (expected - actual) / expected < 1/1024
            return (expected - actual).abs() <= expected.abs() / 1024.0;
        }
        // fall back to absolute tolerance if expectation is 0.0
        actual.abs() < 1e-6
    }
}
