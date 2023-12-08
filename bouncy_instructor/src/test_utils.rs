//! Utilities for unit tests.

use crate::intern::geom::{Angle3d, SignedAngle};
use crate::keypoints::Cartesian3d;

#[track_caller]
pub(crate) fn assert_float_angle_eq(expected: f32, actual: SignedAngle) {
    let radian = *actual;
    assert!(
        float_eq(expected, radian),
        "{expected} == {radian} ({actual:?})"
    );
}

#[track_caller]
pub(crate) fn assert_angle_eq(expected: SignedAngle, actual: SignedAngle) {
    let expected_radian = *expected;
    let actual_radian = *actual;
    assert!(
        float_eq(expected_radian, actual_radian),
        "{expected_radian} == {actual_radian} ({expected:?} == {actual:?})"
    );
}

#[track_caller]
pub(crate) fn assert_angle_3d_eq(expected: Angle3d, actual: Angle3d) {
    assert_angle_eq(expected.azimuth, actual.azimuth);
    assert_angle_eq(expected.polar, actual.polar);
}

#[track_caller]
pub(crate) fn assert_cartesian_eq(expected: Cartesian3d, actual: Cartesian3d) {
    assert!(float_eq(expected.x, actual.x), "{expected:?} == {actual:?}");
    assert!(float_eq(expected.y, actual.y), "{expected:?} == {actual:?}");
    assert!(float_eq(expected.z, actual.z), "{expected:?} == {actual:?}");
}

pub(crate) fn float_eq(expected: f32, actual: f32) -> bool {
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
