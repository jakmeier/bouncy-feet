use super::SignedAngle;

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
    #[allow(dead_code)]
    pub(crate) fn distance(&self, other: &Self) -> f32 {
        let a = self.polar.sin() * other.polar.sin() * (self.azimuth - other.azimuth).cos();
        let b = self.polar.cos() * other.polar.cos();
        // Distance in unit sphere
        let dist = (2.0 - 2.0 * (a + b)).sqrt();
        dist / 2.0
    }

    /// From spherical 3D coordinates to a spherical 2D projection as seen by the camera.
    pub(crate) fn project_2d(&self) -> SignedAngle {
        // polar angle of 0 also means 0 in the projected 2D system
        let x = self.polar.sin() * self.azimuth.sin();
        let y = self.polar.cos();
        let xy_len = x.hypot(y);
        if xy_len.abs() <= 1e-6 {
            SignedAngle::ZERO
        } else {
            SignedAngle::radian(x.atan2(y))
        }
    }
}
