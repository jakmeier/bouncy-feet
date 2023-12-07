use crate::intern::geom::Angle3d;
use std::f32::consts::TAU;
use wasm_bindgen::prelude::wasm_bindgen;

/// A self-sufficient description of a body position snapshot for 2d rendering.
///
/// Each limb has a 2D angle in the x-y plane plus a length factor to simulate
/// the third dimension in a 2D projection. X grows to the right, y grows down.
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
    pub foot: Segment,
    // torso?
    // head?
}

/// Projected lin segment, with a x-y angle and a length factor.
///
/// This format is perfect for 2D drawing.
#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Segment {
    /// The 2D projected angle of the segment, counter-clock wise to the x-axis,
    /// in [0, 2*PI).
    pub angle: f32,
    /// The factor to multiply lengths when drawing the projected segment in 2D.
    pub r: f32,
}

impl From<Angle3d> for Segment {
    fn from(value: Angle3d) -> Self {
        // polar angle of 0 means 90° in the projected 2D system
        let x = -value.polar.sin() * value.azimuth.sin();
        let y = value.polar.cos();
        let xy_len = x.hypot(y);
        if xy_len.abs() <= 1e-6 {
            Self { angle: 0.0, r: 0.0 }
        } else {
            let angle = (y.atan2(x) + TAU) % TAU;
            Self { angle, r: xy_len }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::test_utils::*;

    use std::f32::consts::FRAC_1_SQRT_2;

    #[track_caller]
    fn check_angle_to_segment(
        (azimuth, polar): (f32, f32),
        (expected_angle, expected_len): (f32, f32),
    ) {
        let angle = Angle3d::degree(azimuth, polar);
        let actual: Segment = angle.into();
        let expected = Segment {
            angle: expected_angle.to_radians(),
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
    /// At least both systems use clock-wise angles (left-handed system) and in
    /// both the y axis gros down.
    #[test]
    fn test_angle_to_segment() {
        // (azimuth, polar), (expected_angle, expected_len)
        // straight down
        check_angle_to_segment((0.0, 0.0), (90.0, 1.0));
        check_angle_to_segment((45.0, 0.0), (90.0, 1.0));
        check_angle_to_segment((-45.0, 0.0), (90.0, 1.0));
        check_angle_to_segment((-90.0, 0.0), (90.0, 1.0));

        // to the dancers right => left on screen
        check_angle_to_segment((90.0, 90.0), (180.0, 1.0));
        check_angle_to_segment((90.0, 45.0), (135.0, 1.0));
        check_angle_to_segment((90.0, 100.0), (190.0, 1.0));

        // to the dancers left => right on screen
        check_angle_to_segment((270.0, 90.0), (0.0, 1.0));
        check_angle_to_segment((270.0, 30.0), (60.0, 1.0));
        check_angle_to_segment((270.0, 0.0), (90.0, 1.0));
        check_angle_to_segment((270.0, -30.0), (120.0, 1.0));

        // facing the camera: length is shortened
        check_angle_to_segment((0.0, 30.0), (90.0, 3.0f32.sqrt() / 2.0));
        check_angle_to_segment((0.0, 45.0), (90.0, FRAC_1_SQRT_2));
        check_angle_to_segment((0.0, 60.0), (90.0, 0.5));
        // here the azimuth could be anything, since the length is 0.0, but we
        // want it to be 0.0 for uniqueness of coordinates
        check_angle_to_segment((0.0, 90.0), (0.0, 0.0));
    }
}
