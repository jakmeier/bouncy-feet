//! Wrapper module for all internal code of the instructor, which can be
//! modified without affecting the WASM module interface.

pub(crate) mod geom;
pub(crate) mod pose;
pub(crate) mod pose_db;
pub(crate) mod pose_score;
pub(crate) mod skeleton_3d;
