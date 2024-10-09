use crate::intern::dance_collection::DanceCollection;
use crate::intern::geom::Angle3d;
use crate::intern::pose::Pose;
use crate::intern::skeleton_3d::{Direction, Skeleton3d};
use std::f32::consts::TAU;
use std::fmt::Debug;
use wasm_bindgen::prelude::wasm_bindgen;

/// A position- and size-independent description of a body pose snapshot for 2d
/// rendering. An intermediate step for [`RenderableSkeleton`].
///
/// Each limb has a 2D angle in the x-y plane plus a length factor to simulate
/// the third dimension in a 2D projection. X grows to the right, y grows down.
/// Plus, there is a z-index for the order in which segments should be drawn.
///
/// This format is for exporting to other modules. JS code can easily read it
/// and potentially render it.
///
/// Note that the skeleton is stripped of position information, it only has
/// angles of all body parts. This means it cannot be used to overlay a video.
/// Use the original keypoints for such matters.
///
/// TODO: I  don't think there is a good reason to expose internals of this. JS
/// should only worry about final coordinates, which it gets from the
/// RenderableSkeleton struct.
#[wasm_bindgen(js_name = "Skeleton")]
#[derive(Clone, Copy, Debug)]
pub struct Skeleton {
    // TODO: pub fields with wasm_bindgen are error-prone, as something like
    // `skeleton.left.arm.angle = 5` on the JS side allocates a new side, and a
    // new segment, before changing the angle on the segment, leaving the angle
    // on the original skeleton untouched.
    // I think I should ensure the JS side only does reads. But even that is not
    // ideal, thinking of the performance hit of allocations on every read.
    // I need a deep think about this to decide what's the best way out of this
    // situation.
    pub left: Side,
    pub right: Side,
    pub hip: Segment,
    pub shoulder: Segment,
    /// Does the dancer look more to the side han they face the camera?
    pub sideway: bool,
    /// Does the dancer face away more than they face the camera?
    pub backwards: bool,
}

#[wasm_bindgen(js_name = SkeletonSide)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Side {
    pub thigh: Segment,
    pub shin: Segment,
    pub arm: Segment,
    pub forearm: Segment,
    pub foot: Segment,
    // torso?
    // head?
}

/// Projected line segment, with a x-y angle and a length factor.
///
/// This format is usable for 2D drawing.
#[wasm_bindgen(js_name = Segment)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Segment {
    /// The 2D projected angle of the segment, counter-clock wise to the x-axis,
    /// in [0, 2*PI).
    pub angle: f32,
    /// The factor to multiply lengths when drawing the projected segment in 2D.
    pub r: f32,
    /// Z-Index for draw ordering
    pub z: i16,
}

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Default)]
pub struct Cartesian2d {
    pub x: f32,
    pub y: f32,
}

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq)]
pub enum SkeletonField {
    LeftThigh,
    LeftShin,
    LeftArm,
    LeftForearm,
    LeftFoot,
    RightThigh,
    RightShin,
    RightArm,
    RightForearm,
    RightFoot,
}

#[wasm_bindgen]
impl Skeleton {
    pub fn resting(sideway: bool) -> Self {
        let mut left = Side::default();
        let mut right = Side::default();
        let shoulder;
        let hip;
        if sideway {
            left.foot.angle = 180.0_f32.to_radians();
            right.foot.angle = 180.0_f32.to_radians();
            shoulder = Segment::default();
            hip = Segment::default();
        } else {
            left.foot.angle = 60.0_f32.to_radians();
            right.foot.angle = 120.0_f32.to_radians();
            shoulder = Segment::from(Angle3d::degree(90.0, 90.0));
            hip = Segment::from(Angle3d::degree(90.0, 90.0));
        }
        Skeleton {
            left,
            right,
            shoulder,
            hip,
            sideway,
            backwards: false,
        }
    }

    #[wasm_bindgen(js_name=restingPose)]
    pub fn resting_pose(&self) -> Self {
        Self::resting(self.sideway)
    }

    #[wasm_bindgen(js_name = "debugString")]
    pub fn debug_string(&self) -> String {
        format!("{self:?}")
    }

    #[wasm_bindgen(js_name = "setAngle")]
    pub fn set_angle(&mut self, field: SkeletonField, value: f32) {
        match field {
            SkeletonField::LeftThigh => self.left.thigh.angle = value,
            SkeletonField::LeftShin => self.left.shin.angle = value,
            SkeletonField::LeftArm => self.left.arm.angle = value,
            SkeletonField::LeftForearm => self.left.forearm.angle = value,
            SkeletonField::LeftFoot => self.left.foot.angle = value,
            SkeletonField::RightThigh => self.right.thigh.angle = value,
            SkeletonField::RightShin => self.right.shin.angle = value,
            SkeletonField::RightArm => self.right.arm.angle = value,
            SkeletonField::RightForearm => self.right.forearm.angle = value,
            SkeletonField::RightFoot => self.right.foot.angle = value,
        }
    }
}

impl Skeleton {
    pub(crate) fn from_pose(pose: &Pose, db: &DanceCollection, direction: Direction) -> Self {
        let rotation = 0.0;
        Skeleton3d::from_with_db(pose, db, direction).to_skeleton(rotation)
    }
}

impl From<Angle3d> for Segment {
    fn from(value: Angle3d) -> Self {
        // polar angle of 0 means 90° in the projected 2D system
        let x = -value.polar.sin() * value.azimuth.sin();
        let y = value.polar.cos();
        let xy_len = x.hypot(y);
        if xy_len.abs() <= 1e-6 {
            Self {
                angle: 0.0,
                r: 0.0,
                z: 0,
            }
        } else {
            let angle = (y.atan2(x) + TAU) % TAU;
            Self {
                angle,
                r: xy_len,
                z: 0,
            }
        }
    }
}

impl From<Segment> for Cartesian2d {
    fn from(segment: Segment) -> Self {
        Self {
            x: segment.angle.cos() * segment.r,
            y: segment.angle.sin() * segment.r,
        }
    }
}

impl Default for Segment {
    fn default() -> Self {
        Self {
            angle: 90.0_f32.to_radians(),
            r: 1.0,
            z: 0,
        }
    }
}

// Custom debug mainly for better floats. Used in snapshot tests.
impl Debug for Cartesian2d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = self.x;
        let y = self.y;
        write!(f, "({x:.3}, {y:.3})")
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
            z: 0,
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
    /// both the y axis grows down.
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
