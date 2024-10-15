//! Wrapper types provide a handle to data stored inside the WASM module. It may
//! contain different components of the same entity but has access to all the
//! data without an ECS-style container per component.
//!
//! At least that is kind of the idea. I'm still experimenting with what
//! architecture makes sense.

pub(crate) mod dance_wrapper;
pub(crate) mod pose_wrapper;
pub(crate) mod skeleton_wrapper;
pub(crate) mod step_wrapper;
