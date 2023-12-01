use std::f32::consts::{FRAC_PI_2, TAU};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::intern::geom::Angle3d;

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

/// Projected lin segment, with a x-y angle and a length factor.
///
/// This format is perfect for 2D drawing.
#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Segment {
    /// The 2D projected angle of the segment.
    pub angle: f32,
    /// the factor to multiply lengths when drawing the projected segment in 2D
    pub r: f32,
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

#[cfg(test)]
mod tests {

    use super::*;
    use crate::test_utils::*;

    use std::f32::consts::{FRAC_1_SQRT_2, PI};

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
}
