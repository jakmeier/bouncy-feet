use std::cell::{Ref, RefCell};
use std::rc::Rc;

use crate::intern::skeleton_3d::Skeleton3d;
use crate::intern::tracker_dance_collection::TrackerDanceCollection;
use crate::skeleton::Skeleton;
use crate::{Keypoints, STATE};
use wasm_bindgen::prelude::wasm_bindgen;

use super::pose_wrapper::PoseWrapper;

#[wasm_bindgen]
#[derive(Clone)]
pub struct SkeletonWrapper {
    /// The source of truth. Modification must only go here first and then
    /// propagate to the other fields.
    keypoints: Keypoints,

    // cached values
    skeleton: Rc<RefCell<Option<Skeleton3d>>>,
    skeleton2d_cache: RefCell<Option<Skeleton>>,
}

#[wasm_bindgen]
impl SkeletonWrapper {
    pub fn pose(&self) -> PoseWrapper {
        PoseWrapper::from_skeleton(self)
    }

    pub fn skeleton(&self) -> Skeleton {
        *self.skeleton_2d()
    }

    pub fn set(&self) -> Skeleton {
        *self.skeleton_2d()
    }
}

impl SkeletonWrapper {
    pub(crate) fn new(keypoints: Keypoints) -> Self {
        Self {
            keypoints,
            skeleton: Rc::default(),
            skeleton2d_cache: RefCell::new(None),
        }
    }

    pub(crate) fn skeleton_3d(&self) -> Ref<Skeleton3d> {
        STATE.with_borrow(|state| {
            let db: &TrackerDanceCollection = &state.global_db.tracker_view;

            if self.skeleton.borrow().is_none() {
                *self.skeleton.borrow_mut() = Some(Skeleton3d::from_keypoints(&self.keypoints, db));
            }
            Ref::map(self.skeleton.borrow(), |option| option.as_ref().unwrap())
        })
    }

    pub(crate) fn skeleton_2d(&self) -> Ref<Skeleton> {
        if self.skeleton2d_cache.borrow().is_none() {
            *self.skeleton2d_cache.borrow_mut() = Some(self.skeleton_3d().to_skeleton(0.0));
        }
        Ref::map(self.skeleton2d_cache.borrow(), |option| {
            option.as_ref().unwrap()
        })
    }
}
