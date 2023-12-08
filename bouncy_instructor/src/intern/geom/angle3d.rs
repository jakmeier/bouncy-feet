use super::SignedAngle;
use crate::keypoints::Cartesian3d;

/// A direction in 3D space.
#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) struct Angle3d {
    /// angle to z-axis, -PI to PI
    pub azimuth: SignedAngle,
    /// angle to y axis, -PI to PI
    pub polar: SignedAngle,
}

impl Angle3d {
    pub(crate) fn new(azimuth: SignedAngle, polar: SignedAngle) -> Self {
        Self { azimuth, polar }
    }

    /// Combines two rotations to one and returns the spherical coordinate of the final result.
    ///
    /// The combination is done rotationally, first applying the
    /// forward angles (pitch) and then the sideward angle (yaw).
    pub(crate) fn from_rotations(forward: SignedAngle, right: SignedAngle) -> Self {
        let right = *right;
        let forward = *forward;

        // The multiplication of the rotation matrices, multiplied by (0,0,1) simplifies to this.
        let cartesian = Cartesian3d::new(
            -forward.cos() * right.sin(),
            forward.cos() * right.cos(),
            -forward.sin(),
        );
        // Now use cartesian to spherical conversion
        // (note: could these two steps be combined for better precision and performance?)
        Self::from(cartesian)
    }

    pub(crate) const ZERO: Self = Angle3d {
        azimuth: SignedAngle::ZERO,
        polar: SignedAngle::ZERO,
    };

    #[allow(dead_code)]
    pub(crate) fn degree(azimuth: f32, polar: f32) -> Self {
        Self {
            azimuth: SignedAngle::degree(azimuth),
            polar: SignedAngle::degree(polar),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn radian(azimuth: f32, polar: f32) -> Self {
        Self {
            azimuth: SignedAngle::radian(azimuth),
            polar: SignedAngle::radian(polar),
        }
    }

    /// Distance in a sphere with r = 0.5, result is in [0.0,1.0]
    pub(crate) fn distance(&self, other: &Self) -> f32 {
        let a = self.polar.sin() * other.polar.sin() * (self.azimuth - other.azimuth).cos();
        let b = self.polar.cos() * other.polar.cos();
        // Distance in unit sphere
        let dist = (2.0 - 2.0 * (a + b)).sqrt();
        dist / 2.0
    }

    /// Mirrors left/right, doesn't affect up/down or forward/backward
    pub(crate) fn x_mirror(&self) -> Self {
        Self {
            azimuth: self.azimuth.mirror(),
            polar: self.polar,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::assert_angle_3d_eq;

    #[test]
    fn test_combine_angles() {
        // check_combine_angle((forward, right), (azimuth, polar));
        check_combine_angle((90.0, 0.0), (0.0, 90.0));
        check_combine_angle((120.0, 0.0), (0.0, 120.0));
        check_combine_angle((30.0, 0.0), (0.0, 30.0));
        check_combine_angle((-30.0, 0.0), (180.0, 30.0));
        check_combine_angle((0.0, 90.0), (90.0, 90.0));
        check_combine_angle((0.0, -90.0), (270.0, 90.0));
        check_combine_angle((0.0, 45.0), (90.0, 45.0));
        check_combine_angle((0.1, 45.0), (89.86, 45.0));
        check_combine_angle((45.0, 90.0), (45.0, 90.0));
        check_combine_angle((45.0, 45.0), (35.26, 60.0));
        check_combine_angle((45.0, 135.0), (35.26, 120.0));
        check_combine_angle((45.0, -45.0), (-35.26, 60.0));
        check_combine_angle((45.0, 180.0), (0.0, 135.0));
    }

    #[track_caller]
    fn check_combine_angle((forward, right): (f32, f32), (azimuth, polar): (f32, f32)) {
        let expected = Angle3d::degree(azimuth, polar);
        let angle =
            Angle3d::from_rotations(SignedAngle::degree(forward), SignedAngle::degree(right));
        assert_angle_3d_eq(expected, angle);
    }
}
