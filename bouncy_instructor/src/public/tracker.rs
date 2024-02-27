mod frame_output;
mod pose_output;
mod step_output;

pub use pose_output::PoseApproximation;
pub use step_output::DetectedStep;

use crate::intern::dance_collection::{DanceCollection, ForeignCollectionError};
use crate::intern::skeleton_3d::Skeleton3d;
use crate::keypoints::Keypoints;
use crate::skeleton::Skeleton;
use std::rc::Rc;
use wasm_bindgen::prelude::wasm_bindgen;

type Timestamp = u32;

#[wasm_bindgen]
pub struct Tracker {
    pub(crate) db: Rc<DanceCollection>,

    /// invariant: ordered by timestamp
    pub(crate) timestamps: Vec<Timestamp>,
    /// full keypoints as originally recorded
    pub(crate) keypoints: Vec<Keypoints>,
    /// tracked limbs
    pub(crate) skeletons: Vec<Skeleton3d>,

    // below are "Dance Detector" fields, maybe it should be its own struct?
    pub(crate) bpm: f32,
    // todo: head and tail for what was already detected, to not re-compute all every time
    // todo: active steps filter instead of global steps
}

#[wasm_bindgen]
pub struct Skeletons {
    pub front: Skeleton,
    pub side: Skeleton,
}

#[wasm_bindgen]
impl Tracker {
    /// Create a tracker for all known steps.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Tracker {
            db: crate::STATE.with_borrow(|state| Rc::clone(&state.db)),
            // order by timestamp satisfied for empty list
            timestamps: vec![],
            keypoints: vec![],
            skeletons: vec![],
            bpm: 120.0,
        }
    }

    /// Track one specific step, by name, including its variations (with the same name).
    #[wasm_bindgen(js_name = "StepTracker")]
    pub fn new_step_tracker(step_name: String) -> Result<Tracker, ForeignCollectionError> {
        let mut db = DanceCollection::default();
        crate::STATE.with_borrow(|state| {
            for step in state.db.steps_by_name(&step_name) {
                db.add_foreign_step(&state.db, &step.id)?;
            }
            // also add idle steps, those should always be included in a tracker
            for step in state.db.idle_steps() {
                db.add_foreign_step(&state.db, &step.id)?;
            }
            Ok(())
        })?;
        Ok(Tracker {
            db: Rc::new(db),
            // order by timestamp satisfied for empty list
            timestamps: vec![],
            keypoints: vec![],
            skeletons: vec![],
            bpm: 120.0,
        })
    }

    pub fn clear(&mut self) {
        self.keypoints.clear();
        self.timestamps.clear();
        self.skeletons.clear();
    }

    /// Insert keypoints of a new frame for tracking.
    ///
    /// This is the main method to insert data into the tracker.
    #[wasm_bindgen(js_name = addKeypoints)]
    pub fn add_keypoints(&mut self, keypoints: Keypoints, timestamp: Timestamp) -> Skeletons {
        if let Some(last) = self.timestamps.last() {
            if *last >= timestamp {
                panic!("inserted data not strictly monotonically increasing");
            }
        }

        // modification preserves timestamp order if it was true before
        self.timestamps.push(timestamp);
        self.keypoints.push(keypoints);
        let skeleton_info = Skeleton3d::from_keypoints(&keypoints, &self.db);
        let front = skeleton_info.to_skeleton(0.0);
        let side = skeleton_info.to_skeleton(90.0);
        self.skeletons.push(skeleton_info);
        Skeletons { front, side }
    }

    #[wasm_bindgen(js_name = setBpm)]
    pub fn set_bpm(&mut self, bpm: f32) {
        self.bpm = bpm;
    }

    #[wasm_bindgen(js_name = detectDance)]
    pub fn detect_dance(&self) -> Vec<DetectedStep> {
        let mut start = 0;
        let end = self.timestamps.len();

        let mut out = vec![];
        while let Some(step) = self.find_first_step(start, end) {
            let end_t = step.end;
            start = start + self.timestamps[start..end].partition_point(|t| *t <= end_t);
            out.push(step);
        }
        out
    }

    /// Fit frames in a time interval against all poses and return the best fit.
    ///
    /// This API is exported mostly for debugging. To extract fitted dances, use
    /// `detect_dance` instead.
    #[wasm_bindgen(js_name = bestFitPose)]
    pub fn best_fit_pose(
        &self,
        start: Timestamp,
        end: Timestamp,
    ) -> Option<pose_output::PoseApproximation> {
        let first = self.timestamps.partition_point(|t| *t < start);
        let last = self.timestamps.partition_point(|t| *t <= end);
        if first == last {
            return None;
        }
        self.best_fit_pose_impl(first, last)
    }

    /// Fit a single frame against all poses and return all errors
    #[wasm_bindgen(js_name = allPoseErrors)]
    pub fn all_pose_errors(&self, timestamp: Timestamp) -> Vec<pose_output::PoseApproximation> {
        self.all_pose_approximations(timestamp)
    }

    #[wasm_bindgen(js_name = skeletonAt)]
    pub fn skeleton_at(&self, timestamp: Timestamp) -> Option<Skeleton> {
        let i = self.timestamps.partition_point(|t| *t < timestamp);
        self.skeletons
            .get(i)
            .map(|skeleton_info| skeleton_info.to_skeleton(0.0))
    }
}
