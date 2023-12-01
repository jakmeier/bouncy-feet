//! Computing the error score between a pose and a skeleton.

use super::geom::{Angle3d, SignedAngle};
use super::pose::Pose;
use super::skeleton_3d::Skeleton3d;

/// Find the pose with the lowest error score.
pub(crate) fn best_fit_pose(skeleton: &Skeleton3d, poses: &[Pose]) -> (f32, usize) {
    assert!(!poses.is_empty());

    let mut best_error = f32::INFINITY;
    let mut best_i = 0;
    for (i, pose) in poses.iter().enumerate() {
        let err = pose.error(skeleton.angles());
        if err < best_error {
            best_error = err;
            best_i = i;
        }
    }
    return (best_error, best_i);
}

impl Pose {
    /// Error is between 0.0  and 1.0
    fn error(&self, angles: &[Angle3d]) -> f32 {
        let mut err = 0.0;
        let mut w = 0.0;
        for limb in &self.limbs {
            w += limb.weight;
            let angle = angles[limb.limb];
            err += range_error(angle.azimuth, limb.azimuth_range);
            err += range_error(angle.polar, limb.polar_range);
        }
        // (sus) normalize such that 45° away is 1.0
        let normalized = err / w / std::f32::consts::FRAC_PI_4.powi(2);
        // anything above 45° is a flat 100% error
        return normalized.min(1.0);
    }
}

fn range_error(value: SignedAngle, range: (SignedAngle, SignedAngle)) -> f32 {
    // TODO: add tests and fix (e.g. error around positive to negative ranges, which is broken now)
    let min = *range.0;
    let max = *range.1;
    if *value < min {
        (min - *value).powi(2)
    } else if *value > max {
        (*value - max).powi(2)
    } else {
        0.0
    }
}
