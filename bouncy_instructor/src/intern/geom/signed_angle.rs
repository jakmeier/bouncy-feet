use std::f32::consts::{PI, TAU};

/// Represents angles from -PI (exclusive) to PI (inclusive)
#[derive(Clone, Copy, PartialEq)]
pub(crate) struct SignedAngle(pub(crate) f32);

impl SignedAngle {
    pub(crate) const ZERO: Self = SignedAngle(0.0);

    /// Constructor from angle in degrees.
    pub(crate) fn degree(alpha: f32) -> Self {
        Self(alpha.to_radians()).ensure_signed()
    }

    /// Constructor from angle in radians.
    pub(crate) fn radian(alpha: f32) -> Self {
        Self(alpha).ensure_signed()
    }

    /// Return the angle in degrees
    pub(crate) fn as_degree(&self) -> f32 {
        self.0.to_degrees()
    }

    /// Return the angle in radians
    pub(crate) fn as_radians(&self) -> f32 {
        self.0
    }

    /// Return the angle in radians in range [0,2PI]
    pub(crate) fn as_positive_radians(&self) -> f32 {
        (self.0 + TAU) % TAU
    }

    /// Return the angle in radians
    ///
    /// Important: Don't delete this function, or else `angle.to_radians()` will
    /// still compile with an auto deref but do an extra conversion.
    #[allow(dead_code)]
    pub(crate) fn to_radians(&self) -> f32 {
        self.as_radians()
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

    #[allow(dead_code)]
    pub(crate) fn abs(mut self) -> Self {
        self.0 = self.0.abs();
        self
    }

    pub(crate) fn mirror(self) -> SignedAngle {
        Self(-*self).ensure_signed()
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

    use std::f32::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4};

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

    #[test]
    fn test_mirror_signed_angle() {
        assert_eq!(SignedAngle::ZERO, SignedAngle::ZERO.mirror());
        assert_eq!(SignedAngle(PI), SignedAngle(PI).mirror());
        assert_angle_eq(
            SignedAngle(FRAC_PI_2),
            SignedAngle(3.0 * FRAC_PI_2).mirror(),
        );
        assert_angle_eq(
            SignedAngle::degree(60.0),
            SignedAngle::degree(300.0).mirror(),
        );
        assert_angle_eq(
            SignedAngle(FRAC_PI_3),
            SignedAngle(FRAC_PI_3).mirror().mirror(),
        );
        assert_angle_eq(
            SignedAngle(FRAC_PI_4),
            SignedAngle(FRAC_PI_4).mirror().mirror(),
        );
    }
}
