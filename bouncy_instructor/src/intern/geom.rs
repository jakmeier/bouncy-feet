//! Geometry primitives.

use std::f32::consts::{PI, TAU};

/// A direction in 3D space.
#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) struct Angle3d {
    /// angle to z-axis, -PI to PI
    pub azimuth: SignedAngle,
    /// angle to y axis, -PI to PI
    pub polar: SignedAngle,
}

/// Represents angles from -PI (exclusive) to PI (inclusive)
#[derive(Clone, Copy, PartialEq)]
pub(crate) struct SignedAngle(pub(crate) f32);

impl Angle3d {
    pub(crate) fn new(azimuth: SignedAngle, polar: SignedAngle) -> Self {
        Self { azimuth, polar }
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
}

impl SignedAngle {
    #[allow(dead_code)]
    pub(crate) const ZERO: Self = SignedAngle(0.0);

    pub(crate) fn degree(alpha: f32) -> Self {
        Self(alpha.to_radians()).ensure_signed()
    }

    pub(crate) fn as_degree(&self) -> f32 {
        self.0.to_degrees()
    }

    /// Returns a copy of the angle where values are guaranteed to be in (-PI and PI]
    #[inline]
    fn ensure_signed(mut self) -> Self {
        self.0 = self.0 % TAU;
        // maybe branching here is bad for performance?
        // no performance testing was done so far
        if self.0 > PI {
            self.0 -= TAU;
        } else if self.0 <= -PI {
            self.0 += TAU;
        }
        self
    }

    pub(crate) fn abs(mut self) -> Self {
        self.0 = self.0.abs();
        self
    }

    pub(crate) fn radian(alpha: f32) -> Self {
        Self(alpha).ensure_signed()
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
    use super::*;
    use crate::test_utils::*;

    use std::f32::consts::FRAC_PI_4;

    /// Tests `SignedAngle::degree`
    ///
    /// The inputs of are in ° in [f32::MIN, f32::MAX]
    /// The internal representation must be in radians in (-PI, +PI].
    #[test]
    fn test_angle_degree_to_radian() {
        assert_float_angle_eq(0.0, SignedAngle::degree(0.0));
        assert_float_angle_eq(FRAC_PI_4, SignedAngle::degree(45.0));
        assert_float_angle_eq(-FRAC_PI_4, SignedAngle::degree(-45.0));
        assert_float_angle_eq(-FRAC_PI_4, SignedAngle::degree(315.0));
        assert_float_angle_eq(PI, SignedAngle::degree(-180.0));
        assert_float_angle_eq(PI, SignedAngle::degree(180.0));
    }
}
