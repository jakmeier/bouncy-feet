//! Wrapper module for all internal code of the instructor, which can be
//! modified without affecting the WASM module interface.

pub(crate) mod body_shift;
pub(crate) mod dance;
pub(crate) mod dance_collection;
pub(crate) mod dance_detector;
pub(crate) mod geom;
pub(crate) mod keypoints_iter;
pub(crate) mod lfsr;
pub(crate) mod pose;
pub(crate) mod pose_score;
pub(crate) mod skeleton_3d;
pub(crate) mod step;
