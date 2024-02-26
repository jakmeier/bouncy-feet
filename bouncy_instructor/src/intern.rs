//! Wrapper module for all internal code of the instructor, which can be
//! modified without affecting the WASM module interface.

pub(crate) mod dance;
pub(crate) mod dance_collection;
pub(crate) mod geom;
pub(crate) mod keypoints_iter;
pub(crate) mod pose;
pub(crate) mod pose_score;
pub(crate) mod skeleton_3d;
pub(crate) mod step;
