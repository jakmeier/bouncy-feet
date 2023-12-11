//! Computing the error score between a pose and a skeleton.

use super::geom::{Angle3d, SignedAngle};
use super::pose::Pose;
use super::pose_db::LimbIndex;
use super::skeleton_3d::Skeleton3d;

/// Describe the target angle and how to compute an error score from it.
///
/// If the angle is not important, set the weight to 0. But the angle will still
/// matter for rendering the perfect pose skeleton.
#[derive(Clone, Debug)]
pub(crate) struct AngleTarget {
    /// the perfect angle to achive
    angle: Angle3d,
    /// how many degrees in any direction is still considered no error
    tolerance: SignedAngle,
    /// weight factor for error computation
    ///
    /// for now, applied equally to azimuth and polar angle
    weight: f32,
}

/// Error details for all limbs
#[derive(Default)]
pub(crate) struct ErrorDetails {
    /// Index of limbs which were part of the compared pose
    pub limbs: Vec<LimbIndex>,
    /// error per limb, already normalized to [0.0,1.0]
    pub errors: Vec<f32>,
    /// weights to compute full error score
    pub weights: Vec<f32>,
}

/// Find the pose with the lowest error score.
pub(crate) fn best_fit_pose(skeleton: &Skeleton3d, poses: &[Pose]) -> (f32, ErrorDetails, usize) {
    assert!(!poses.is_empty());

    let mut best_error = f32::INFINITY;
    let mut best_details = ErrorDetails::default();
    let mut best_i = 0;
    for (i, pose) in poses.iter().enumerate() {
        let details = pose.error(skeleton.angles());
        let err = details.error_score();
        if err < best_error {
            best_error = err;
            best_i = i;
            best_details = details;
        }
    }
    return (best_error, best_details, best_i);
}

impl AngleTarget {
    pub(crate) fn new(angle: Angle3d, tolerance: SignedAngle, weight: f32) -> Self {
        Self {
            angle,
            tolerance,
            weight,
        }
    }

    /// Error between 0.0 and 1.0
    fn target_error(&self, value: Angle3d) -> f32 {
        // note: not sure if this error fromula is any good at all
        // especially the tolerance is super weird
        let tolerance_distance =
            Angle3d::ZERO.distance(&Angle3d::new(SignedAngle::ZERO, self.tolerance));
        0.0f32
            .max(self.angle.distance(&value) - tolerance_distance)
            .powi(2)
    }

    pub(crate) fn weight(&self) -> f32 {
        self.weight
    }

    pub(crate) fn angle(&self) -> Angle3d {
        self.angle
    }

    /// Mirrors left/right, doesn't affect up/down or forward/backward
    pub(crate) fn x_mirror(&self) -> AngleTarget {
        Self {
            angle: self.angle.x_mirror(),
            tolerance: self.tolerance,
            weight: self.weight,
        }
    }
}

impl Pose {
    /// Error is between 0.0  and 1.0
    pub(crate) fn error(&self, angles: &[Angle3d]) -> ErrorDetails {
        let mut errors = Vec::with_capacity(2 * self.limbs.len());
        let mut weights = Vec::with_capacity(2 * self.limbs.len());
        let mut limbs = Vec::with_capacity(2 * self.limbs.len());
        for limb in &self.limbs {
            limbs.push(limb.limb);
            let angle = angles[limb.limb.as_usize()];

            errors.push(limb.target.target_error(angle));
            weights.push(limb.weight());
        }
        return ErrorDetails {
            limbs,
            errors,
            weights,
        };
    }
}

impl ErrorDetails {
    pub(crate) fn error_score(&self) -> f32 {
        let (total_err, total_weight) = self
            .errors
            .iter()
            .zip(&self.weights)
            .fold((0.0, 0.0), |(e_acc, w_acc), (e, w)| {
                (e_acc + e * w, w_acc + w)
            });
        if total_weight > 0.0 {
            total_err / total_weight
        } else {
            1.0
        }
    }

    /// returns indices of limbs in increasing order of how much they contribute to the total error
    pub(crate) fn sorted_by_error(&self, increasing: bool, weighted: bool) -> Vec<usize> {
        let mut indices = (0..self.errors.len()).collect::<Vec<_>>();
        indices.sort_by(|&left, &right| {
            let lhs = if weighted {
                self.errors[left] * self.weights[left]
            } else {
                self.errors[left]
            };
            let rhs = if weighted {
                self.errors[right] * self.weights[right]
            } else {
                self.errors[right]
            };
            if increasing {
                lhs.partial_cmp(&rhs).unwrap()
            } else {
                rhs.partial_cmp(&lhs).unwrap()
            }
        });
        indices
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::intern::geom::SignedAngle;
    use crate::intern::pose::{Limb, LimbPosition};
    use expect_test::expect;
    use std::f32::consts::PI;

    /// test several angle for perfect matches, they should always return zero error
    #[test]
    fn test_single_limb_perfect_score() {
        check_single_limb_perfect_score(0.0, 0.0);
        check_single_limb_perfect_score(90.0, 0.0);
        check_single_limb_perfect_score(0.0, 90.0);
        // TODO; this seems to have mathematical instability
        // check_single_limb_perfect_score(45.0, 45.0);
        check_single_limb_perfect_score(123.0, -12.0);
    }

    /// asserts that given angle for a limb and the same in the pose results in zero error
    #[track_caller]
    fn check_single_limb_perfect_score(azimuth: f32, polar: f32) {
        let azimuth = SignedAngle::degree(azimuth);
        let polar = SignedAngle::degree(polar);
        let limb = LimbPosition::new(Limb::LEFT_THIGH, azimuth, polar, SignedAngle::ZERO, 1.0);
        let pose = Pose::new(vec![limb]);
        let mut angles = zero_skeleton();
        angles[Limb::LEFT_THIGH.as_usize()] = Angle3d::new(azimuth, polar);
        let error = pose.error(&angles);
        assert_eq!(0.0, error.error_score());
    }

    // Below are several tests that define a specific skeleton and combine it
    // with a fixed pose to see if the results are stable error scores.
    // And then some more tests the other way around, defining several poses and
    // testing against a fixed skeleton.
    //
    // When fine-tuning the error function, the exact numbers will change.
    // Snapshot testing helps to quickly update the exact scores. Set the
    // environment variable `UPDATE_EXPECT` to do it automatically while
    // running tests. Then check that the changes actually make sense.

    #[test]
    fn test_standing_straight_pose_score() {
        let tol = SignedAngle::degree(5.0);
        #[rustfmt::skip]
        let pose = Pose::new(vec![
            LimbPosition::new(Limb::LEFT_THIGH, SignedAngle(0.0), SignedAngle(0.0), tol, 1.0),
            LimbPosition::new(Limb::LEFT_SHIN, SignedAngle(0.0), SignedAngle(0.0), tol, 1.0),
            LimbPosition::new(Limb::LEFT_ARM, SignedAngle(0.0), SignedAngle(0.0), tol, 1.0),
            LimbPosition::new(Limb::LEFT_FOREARM, SignedAngle(0.0), SignedAngle(0.0), tol, 1.0),
            LimbPosition::new(Limb::RIGHT_THIGH, SignedAngle(0.0), SignedAngle(0.0), tol, 1.0),
            LimbPosition::new(Limb::RIGHT_SHIN, SignedAngle(0.0), SignedAngle(0.0), tol, 1.0),
            LimbPosition::new(Limb::RIGHT_ARM, SignedAngle(0.0), SignedAngle(0.0), tol, 1.0),
            LimbPosition::new(Limb::RIGHT_FOREARM, SignedAngle(0.0), SignedAngle(0.0), tol, 1.0),
        ]);
        check_score_fixed_skeleton(&pose, expect!["0.193822"]);
    }

    #[test]
    fn test_t_pose_score() {
        let tol = SignedAngle::degree(5.0);
        #[rustfmt::skip]
        let pose = Pose::new(vec![
            LimbPosition::new(Limb::LEFT_THIGH, SignedAngle(0.0), SignedAngle(0.0), tol, 1.0),
            LimbPosition::new(Limb::LEFT_SHIN, SignedAngle(0.0), SignedAngle(0.0), tol, 1.0),
            LimbPosition::new(Limb::LEFT_ARM, SignedAngle(3.0 * PI), SignedAngle(PI), tol, 1.0),
            LimbPosition::new(Limb::LEFT_FOREARM, SignedAngle(3.0 * PI), SignedAngle(PI), tol, 1.0),
            LimbPosition::new(Limb::RIGHT_THIGH, SignedAngle(0.0), SignedAngle(0.0), tol, 1.0),
            LimbPosition::new(Limb::RIGHT_SHIN, SignedAngle(0.0), SignedAngle(0.0), tol, 1.0),
            LimbPosition::new(Limb::RIGHT_ARM, SignedAngle(PI), SignedAngle(PI), tol, 1.0),
            LimbPosition::new(Limb::RIGHT_FOREARM, SignedAngle(PI), SignedAngle(PI), tol, 1.0),
        ]);
        check_score_fixed_skeleton(&pose, expect!["0.3906417"]);
    }

    #[test]
    fn test_close_to_correct_pose_score() {
        let mut pose = fixed_pose(1.0);
        pose.limbs[Limb::RIGHT_THIGH.as_usize()]
            .target
            .angle
            .polar
            .0 += PI / 5.0;
        pose.limbs[Limb::LEFT_ARM.as_usize()].target.angle.polar.0 += PI / 17.0;

        pose.limbs[Limb::RIGHT_FOREARM.as_usize()]
            .target
            .angle
            .azimuth
            .0 += PI / 17.0;

        check_score_fixed_skeleton(&pose, expect!["0.013016654"]);
    }

    #[test]
    fn test_close_to_correct_skeleton_score() {
        let mut skeleton = fixed_skeleton();
        skeleton[0].polar.0 += PI / 37.0;
        skeleton[1].polar.0 += PI / 17.0;
        skeleton[3].azimuth.0 -= PI / 19.0;
        check_score_fixed_pose(&skeleton, expect!["0.0003231239"]);
    }

    #[test]
    fn test_standing_straight_skeleton_score() {
        let skeleton = zero_skeleton();
        check_score_fixed_pose(&skeleton, expect!["0.193822"]);
    }

    /// asserts that a pose evaluated against a fixed skeleton results in the expected error score
    #[track_caller]
    fn check_score_fixed_skeleton(pose: &Pose, expect: expect_test::Expect) {
        let angles = fixed_skeleton();
        let error = pose.error(&angles).error_score();
        expect.assert_eq(&error.to_string());
    }

    /// asserts that a skeleton evaluated against a fixed pose results in the expected error score
    #[track_caller]
    fn check_score_fixed_pose(skeleton: &[Angle3d], expect: expect_test::Expect) {
        let pose = fixed_pose(5.0);
        let error = pose.error(&skeleton).error_score();
        expect.assert_eq(&error.to_string());
    }

    fn zero_skeleton() -> Vec<Angle3d> {
        vec![Angle3d::ZERO; Limb::base_limbs().len()]
    }

    /// using a somewhat random skeleton, doesn't really matter what it is
    /// just don't make it too complicated, for interpretability's sake
    fn fixed_skeleton() -> Vec<Angle3d> {
        let mut angles = zero_skeleton();

        angles[Limb::RIGHT_THIGH.as_usize()] = Angle3d::degree(0.0, 90.0);
        angles[Limb::RIGHT_SHIN.as_usize()] = Angle3d::degree(0.0, 45.0);
        angles[Limb::RIGHT_ARM.as_usize()] = Angle3d::degree(90.0, 90.0);
        angles[Limb::RIGHT_FOREARM.as_usize()] = Angle3d::degree(90.0, 90.0);
        angles[Limb::LEFT_ARM.as_usize()] = Angle3d::degree(270.0, 45.0);
        angles[Limb::LEFT_FOREARM.as_usize()] = Angle3d::degree(270.0, 0.0);
        angles
    }

    /// using the same angles as used in `fixed_skeleton`
    #[rustfmt::skip]
    fn fixed_pose(tolerance: f32) -> Pose {
        let tol = SignedAngle::degree(tolerance);
        Pose::new(vec![
            LimbPosition::new(Limb::LEFT_THIGH, SignedAngle(0.0), SignedAngle(0.0), tol, 1.0),
            LimbPosition::new(Limb::LEFT_SHIN, SignedAngle(0.0), SignedAngle(0.0), tol, 1.0),
            LimbPosition::new(Limb::LEFT_FOOT, SignedAngle(0.0), SignedAngle(0.0), tol, 0.0),
            LimbPosition::new(Limb::LEFT_ARM, SignedAngle(3.0 * PI/2.0), SignedAngle(PI/4.0), tol, 1.0),
            LimbPosition::new(Limb::LEFT_FOREARM, SignedAngle(3.0 * PI/2.0), SignedAngle(0.0), tol, 1.0),
            LimbPosition::new(Limb::RIGHT_THIGH, SignedAngle(0.0), SignedAngle(PI/2.0), tol, 1.0),
            LimbPosition::new(Limb::RIGHT_SHIN, SignedAngle(0.0), SignedAngle(PI/4.0), tol, 1.0),
            LimbPosition::new(Limb::RIGHT_FOOT, SignedAngle(0.0), SignedAngle(0.0), tol, 0.0),
            LimbPosition::new(Limb::RIGHT_ARM, SignedAngle(PI/2.0), SignedAngle(PI/2.0), tol, 1.0),
            LimbPosition::new(Limb::RIGHT_FOREARM, SignedAngle(PI/2.0), SignedAngle(PI/2.0), tol, 1.0),
        ])
    }
}
