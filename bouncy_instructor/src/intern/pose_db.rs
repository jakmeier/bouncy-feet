//! Types and code for keeping a list of poses to detect across method calls.
//!
//! Poses are defined with regard to a list of limb definitions, in order to
//! avoid storing the limb definitions repeatedly. The downside is that each
//! data set (one data set per video frame) on its own lacks context. You have
//! to combine it with the list of poses. This module contains the context
//!  structs required for this.

use std::f32::consts::PI;

use super::geom::SignedAngle;
use super::pose::{Limb, LimbPosition, Pose};

/// List of registered poses to recognize during tracking.
///
/// Each pose is a description of a body position. This includes the exact
/// desired position, a name, and most importantly, the formulas for computing
/// an error.
///
/// Actual poses are defined in external files and loaded in at runtime. Here
/// data they are stored in the most convenient way, which will see many
/// refactoring over time.
///
/// Errors are always between 0 and 1, where 0 is a perfect match.
/// For now, the error formula is implicitly the same for all limbs.
pub(crate) struct LimbPositionDatabase {
    /// Pose definitions
    poses: Vec<Pose>,
    /// Pose names, shares the index with `.poses`
    names: Vec<String>,

    /// list of limbs to track, referenced by `LimbPosition.limb`.
    ///
    /// invariant: must contain only unique values
    /// contract: append only
    limbs: Vec<Limb>,
}

impl Default for LimbPositionDatabase {
    fn default() -> Self {
        Self {
            poses: vec![],
            names: vec![],
            limbs: Limb::base_limbs(),
        }
    }
}

impl LimbPositionDatabase {
    pub(crate) fn add(&mut self, poses: Vec<crate::pose_file::Pose>) {
        for pose in poses {
            let limbs = pose
                .limbs
                .into_iter()
                .map(|def| {
                    let limb = Limb::from(def.limb);
                    let index;
                    if let Some(i) = self.limbs.iter().position(|l| *l == limb) {
                        index = i;
                    } else {
                        index = self.limbs.len();
                        self.limbs.push(limb);
                    }
                    // definition is in °, internal usage is in rad
                    let alpha = SignedAngle::degree(def.angle as f32);
                    let tolerance = SignedAngle::degree(def.tolerance as f32);

                    // convert from pose definition coordinates to spherical coordinates
                    //
                    // iterative implementation:
                    // now: only forward/backward angles are allowed
                    // future: explicitly define whether it is a side angle or a forward/backward angle
                    let azimuth = SignedAngle(if alpha.is_sign_positive() { 0.0 } else { PI });
                    let polar = SignedAngle(alpha.abs());

                    LimbPosition::new(index, azimuth, polar, tolerance, def.weight)
                })
                .collect();
            self.poses.push(Pose::new(limbs));
            self.names.push(pose.name);
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.poses.is_empty()
    }

    pub(crate) fn pose_name(&self, i: usize) -> &str {
        &self.names[i]
    }

    pub(crate) fn limbs(&self) -> impl Iterator<Item = &Limb> {
        self.limbs.iter()
    }

    pub(crate) fn poses(&self) -> &[Pose] {
        &self.poses
    }
}
