use super::{Angle3d, SignedAngle};
use crate::keypoints::Cartesian3d;
use crate::skeleton::Cartesian2d;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
impl Cartesian2d {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    #[wasm_bindgen(js_name = "add")]
    pub fn js_add(&self, other: &Cartesian2d) -> Self {
        *self + *other
    }
}

impl Cartesian2d {
    pub fn mirror(&self) -> Self {
        Self {
            x: -self.x,
            y: self.y,
        }
    }
}

impl Cartesian3d {
    const ZERO: Self = Cartesian3d {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    /// The polar angle is measured against the y-axis, which goes from the
    /// ground to the sky.
    ///
    /// The polar angle is between 0° and +180°, with 0° pointing to
    /// the ground, 180° to the sky.
    ///
    /// Returned values are in radian, hence [0, PI]
    pub(crate) fn polar_angle(&self, other: Cartesian3d) -> SignedAngle {
        // only the sign of dy matters here, and we want it to grow down when we
        // use acos to compute the polar angle
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let dz = other.z - self.z;

        let r = (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt();
        if !r.is_normal() {
            // Handle vectors of lengths very close to zero, NaN, or infinity.
            // Returning 0° is as good as any other angle.
            return SignedAngle(0.0);
        }
        // note: potentially this could be computed more efficiently
        // note 2: what about Math.acos() instead of wasm ?
        SignedAngle::radian((dy / r).acos())
    }

    /// The azimuth is the clock-wise angle to the negative z-axis.
    ///
    /// The azimuth is between -180° and 180°. Someone facing the camera has an
    /// azimuth of 0°, which is also known as north.
    ///
    /// Returned values are in radian, (-PI to PI].
    ///
    /// Just like in cartography, east is +90° (PI/2) and west is -90° (-PI/2)
    /// for the dancer. However, in videos, the angles are therefore
    /// counter-clock-wise as seen by the camera.
    ///
    /// Note that in the keypoint coordinate system, the x-axis grows to the
    /// right. In a (non-mirrored) video this means we see the left arm of the
    /// dance (west) in the positive x-direction. Which is the opposite of how
    /// angles grow in our spherical coordinates. Also confusing, the positive
    /// z-axis faces south, not north.
    pub(crate) fn azimuth(&self, other: Cartesian3d) -> SignedAngle {
        // usually you should expect other - self, but we need to flip both signs
        let dz = self.z - other.z;
        let dx = self.x - other.x;
        let r = dx.hypot(dz);
        if !r.is_normal() {
            // Handle vectors of lengths very close to zero, NaN, or infinity.
            // Returning 0° is as good as any other angle.
            return SignedAngle(0.0);
        }
        // note: potentially this could be computed more efficiently, esp. the sign
        // note 2: what about Math.acos() instead of wasm ?
        SignedAngle::radian(dx.signum() * (dz / r).acos())
    }

    #[allow(dead_code)]
    pub(crate) fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}

impl From<Cartesian3d> for Angle3d {
    fn from(p: Cartesian3d) -> Self {
        Self::new(
            Cartesian3d::ZERO.azimuth(p),
            Cartesian3d::ZERO.polar_angle(p),
        )
    }
}

impl From<Angle3d> for Cartesian3d {
    fn from(angle: Angle3d) -> Self {
        let x = -angle.polar.sin() * angle.azimuth.sin();
        let y = angle.polar.cos();
        let z = -angle.polar.sin() * angle.azimuth.cos();
        Self { x, y, z }
    }
}

impl std::ops::Add<Cartesian3d> for Cartesian3d {
    type Output = Self;

    fn add(self, rhs: Cartesian3d) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub<Cartesian3d> for Cartesian3d {
    type Output = Self;

    fn sub(self, rhs: Cartesian3d) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Mul<f32> for Cartesian3d {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Add<Cartesian2d> for Cartesian2d {
    type Output = Self;

    fn add(self, rhs: Cartesian2d) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub<Cartesian2d> for Cartesian2d {
    type Output = Self;

    fn sub(self, rhs: Cartesian2d) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Mul<f32> for Cartesian2d {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Mul<usize> for Cartesian2d {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Self {
            x: self.x * rhs as f32,
            y: self.y * rhs as f32,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::FRAC_1_SQRT_2;

    use super::*;
    use crate::intern::geom::Angle3d;
    use crate::test_utils::{assert_angle_eq, assert_cartesian_eq};

    const SQRT_3: f32 = 1.732_050_8;

    #[test]
    fn test_cartesian_to_angle() {
        // input, azimuth, polar

        // in keypoint coordinates, the negative x direction is the right hand
        // of the dancer, which is the positive angle direction
        check_cartesian_to_angle(Cartesian3d::new(1.0, 0.0, 0.0), -90.0, 90.0);
        check_cartesian_to_angle(Cartesian3d::new(-1.0, 0.0, 0.0), 90.0, 90.0);

        // down is 0° (and camera y grows down)
        check_cartesian_to_angle(Cartesian3d::new(0.0, 1.0, 0.0), 0.0, 0.0);
        // up is 180°
        check_cartesian_to_angle(Cartesian3d::new(0.0, -1.0, 0.0), 0.0, 180.0);

        // away from the camera means south => azimuth = 180°
        check_cartesian_to_angle(Cartesian3d::new(0.0, 0.0, 1.0), 180.0, 90.0);
        // to the camera means north => azimuth = 0
        check_cartesian_to_angle(Cartesian3d::new(0.0, 0.0, -1.0), 0.0, 90.0);

        check_cartesian_to_angle(Cartesian3d::new(1.0, 0.0, 0.0), -90.0, 90.0);
        check_cartesian_to_angle(Cartesian3d::new(1.0, 1.0, 0.0), -90.0, 45.0);
        check_cartesian_to_angle(Cartesian3d::new(1.0, SQRT_3, 0.0), -90.0, 30.0);
        check_cartesian_to_angle(Cartesian3d::new(1.0, 1.0 / SQRT_3, 0.0), -90.0, 60.0);
    }

    #[track_caller]
    fn check_cartesian_to_angle(
        cartesian: Cartesian3d,
        expected_azimuth: f32,
        expected_polar: f32,
    ) {
        let origin = Cartesian3d::new(0.0, 0.0, 0.0);
        assert_angle_eq(
            SignedAngle::degree(expected_azimuth),
            origin.azimuth(cartesian),
        );
        assert_angle_eq(
            SignedAngle::degree(expected_polar),
            origin.polar_angle(cartesian),
        );
    }

    #[test]
    fn test_cartesian_to_angle_to_2d_projected() {
        // with z = 0, everything should be like 2D
        check_cartesian_to_projected(Cartesian3d::new(-1.0, 0.0, 0.0), 90.0);
        check_cartesian_to_projected(Cartesian3d::new(-1.0, 1.0, 0.0), 45.0);
        check_cartesian_to_projected(Cartesian3d::new(-1.0, SQRT_3, 0.0), 30.0);
        check_cartesian_to_projected(Cartesian3d::new(-1.0, 1.0 / SQRT_3, 0.0), 60.0);

        // with z != 0, everything should actually just be the same with the 2D projection
        check_cartesian_to_projected(Cartesian3d::new(-1.0, 0.0, 1.0), 90.0);
        check_cartesian_to_projected(Cartesian3d::new(-1.0, 1.0, 1.0), 45.0);
        check_cartesian_to_projected(Cartesian3d::new(-1.0, SQRT_3, 1.0), 30.0);
        check_cartesian_to_projected(Cartesian3d::new(-1.0, 1.0 / SQRT_3, 1.0), 60.0);
    }

    #[track_caller]
    fn check_cartesian_to_projected(cartesian: Cartesian3d, expected_angle: f32) {
        let origin = Cartesian3d::new(0.0, 0.0, 0.0);
        let angle = Angle3d::new(origin.azimuth(cartesian), origin.polar_angle(cartesian));
        assert_angle_eq(SignedAngle::degree(expected_angle), angle.project_2d());
    }

    #[test]
    fn test_angle_to_cartesian() {
        // azimuth, polar, (x,y,z)
        check_angle_to_cartesian(0.0, 0.0, (0.0, 1.0, 0.0));
        check_angle_to_cartesian(-90.0, 90.0, (1.0, 0.0, 0.0));
        check_angle_to_cartesian(90.0, 90.0, (-1.0, 0.0, 0.0));
        check_angle_to_cartesian(0.0, 180.0, (0.0, -1.0, 0.0));
        check_angle_to_cartesian(180.0, 90.0, (0.0, 0.0, 1.0));
        check_angle_to_cartesian(0.0, 90.0, (0.0, 0.0, -1.0));
        check_angle_to_cartesian(0.0, 45.0, (0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
        check_angle_to_cartesian(45.0, 45.0, (-0.5, FRAC_1_SQRT_2, -0.5));
        check_angle_to_cartesian(135.0, 45.0, (-0.5, FRAC_1_SQRT_2, 0.5));
    }

    #[track_caller]
    fn check_angle_to_cartesian(azimuth: f32, polar: f32, expected_cartesian: (f32, f32, f32)) {
        let (x, y, z) = expected_cartesian;
        let want = Cartesian3d::new(x, y, z);

        let angle = Angle3d::degree(azimuth, polar);
        let actual = Cartesian3d::from(angle);
        assert_cartesian_eq(want, actual);
    }

    #[test]
    fn test_cartesian_to_angle_and_back() {
        check_cartesian_to_angle_and_back(1.0, 0.0, 0.0);
        check_cartesian_to_angle_and_back(0.0, 1.0, 0.0);
        check_cartesian_to_angle_and_back(0.0, 0.0, 1.0);
        check_cartesian_to_angle_and_back(1.0, 1.0, 1.0);
        check_cartesian_to_angle_and_back(1.0, -1.0, 1.0);
        check_cartesian_to_angle_and_back(0.0, 1.0, -1.0);
        check_cartesian_to_angle_and_back(-3.2, 7.1, -1.1);
        check_cartesian_to_angle_and_back(0.0, 0.0, 0.0);
    }

    #[track_caller]
    fn check_cartesian_to_angle_and_back(x: f32, y: f32, z: f32) {
        let start = Cartesian3d::new(x, y, z);
        let origin = Cartesian3d::new(0.0, 0.0, 0.0);
        let angle = Angle3d::new(origin.azimuth(start), origin.polar_angle(start));

        let end = Cartesian3d::from(angle) * start.length();
        assert_cartesian_eq(start, end);
    }
}
