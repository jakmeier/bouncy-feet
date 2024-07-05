mod detection_output;
mod frame_output;
mod pose_output;
mod step_output;

pub use detection_output::DetectionResult;
pub use pose_output::PoseApproximation;
pub use step_output::DetectedStep;

use crate::intern::dance_collection::{DanceCollection, ForeignCollectionError};
use crate::intern::skeleton_3d::Skeleton3d;
use crate::keypoints::Keypoints;
use crate::skeleton::{Cartesian2d, Skeleton};
use crate::StepInfo;
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
    /// (experimenting with live instructor, I probably want to change this when cleaning up the impl)
    /// only set for unique step tracking
    pub(crate) intermediate_result: Option<DetectionResult>,
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
            intermediate_result: None,
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
            intermediate_result: None,
        })
    }

    /// Track one specific step, by ID, excluding its variations (with the same name).
    ///
    /// This is not intended for general dance detection but rather for a
    /// specific training session without much regard for timing etc.
    #[wasm_bindgen(js_name = "UniqueStepTracker")]
    pub fn new_unique_step_tracker(step_id: String) -> Result<Tracker, ForeignCollectionError> {
        // FIXME: using only a part of it reveals some bugs in how pose indics are used...
        let db = crate::STATE.with_borrow(|state| Rc::clone(&state.db));
        let step_info = db
            .step(&step_id)
            .cloned()
            .expect("just added the step")
            .into();
        Ok(Tracker {
            db,
            // order by timestamp satisfied for empty list
            timestamps: vec![],
            keypoints: vec![],
            skeletons: vec![],
            bpm: 120.0,
            intermediate_result: Some(DetectionResult::for_unique_step_tracker(step_info)),
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

    /// Goes over all data and detects the best fitting dance.
    ///
    /// There is no re-use or consistency between calls. It always starts at 0
    /// and computes the global best fit.
    ///
    /// Use [`Tracker::detect_next_pose`] for incremental detection.
    #[wasm_bindgen(js_name = detectDance)]
    pub fn detect_dance(&self) -> DetectionResult {
        let mut start = 0;
        let end = self.timestamps.len();

        let mut out = vec![];
        while let Some(step) = self.find_first_step(start, end) {
            let end_t = step.end;
            start = start + self.timestamps[start..end].partition_point(|t| *t <= end_t);
            out.push(step);
        }
        DetectionResult::new(out)
    }

    /// Take a previous detection and try adding one more pose to it.
    ///
    /// For now this only looks at the very last frame, but this is an
    /// implementation detail. Callers should assume it reads everything since
    /// the last detected step.
    #[wasm_bindgen(js_name = detectNextPose)]
    pub fn detect_next_pose(&mut self) -> DetectionResult {
        let prev_detection = self
            .intermediate_result
            .as_ref()
            .expect("requires intermediate_result");

        let end_t = *self.timestamps.last().unwrap_or(&0);
        let last_step = prev_detection
            .partial
            .as_ref()
            .or_else(|| prev_detection.steps.last());
        let start_t = last_step.map_or(0, |step| step.end);

        // skip at least a quarter beat
        if end_t < start_t + ((1000.0 / (self.bpm * 4.0 / 60.0)).round() as u32) {
            return prev_detection.clone();
        }

        if let Some(tracked_skeleton) = self.skeletons.last() {
            let beat = self.expected_pose_beat();
            let step_info = self.tracked_step();
            let step = self
                .db
                .step(&step_info.id())
                .expect("tracked step must exist");

            let pose_idx = step.poses[beat % step.poses.len()];
            let pose = &self.db.poses()[pose_idx];
            let error_details = pose.error(tracked_skeleton.angles(), tracked_skeleton.positions());
            let error = error_details.error_score();
            // TODO threshold config
            if error < 0.075 {
                self.add_pose(PoseApproximation {
                    name: self.db.pose_name(pose_idx).to_owned(),
                    error,
                    timestamp: end_t,
                    error_details,
                });
            }
        }

        self.intermediate_result.clone().unwrap()
    }

    /// Return a skeleton that's expected next.
    ///
    /// Only implemented to work properly for trackers of unique steps.
    ///
    /// (experimenting with live instructor, I probably want to change this when cleaning up the impl)
    #[wasm_bindgen(js_name = expectedPoseSkeleton)]
    pub fn expected_pose_skeleton(&self) -> Skeleton {
        let beat = self.expected_pose_beat();
        let step_info = self.tracked_step();
        step_info.skeleton(beat)
    }

    #[wasm_bindgen(js_name = expectedPoseBodyShift)]
    pub fn expected_pose_body_shift(&self) -> Cartesian2d {
        let beat = self.expected_pose_beat();
        let step_info = self.tracked_step();
        step_info.body_shift(beat)
    }

    #[wasm_bindgen(js_name = numDetectedPoses)]
    pub fn num_detected_poses(&self) -> u32 {
        if let Some(detection) = &self.intermediate_result {
            let mut out = 0;
            for step in &detection.steps {
                out += step.poses.len();
            }
            if let Some(partial) = &detection.partial {
                out += partial.poses.len();
            }
            out as u32
        } else {
            0
        }
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

impl Tracker {
    /// Return a pose that's expected next.
    ///
    /// Only implemented to work properly for trackers of unique steps.
    ///
    /// returns a beat number
    /// (experimenting with live instructor, I probably want to change this when cleaning up the impl)
    fn expected_pose_beat(&self) -> usize {
        let detection = self
            .intermediate_result
            .as_ref()
            .expect("requires intermediate_result");

        let full = detection.steps.len()
            * detection
                .target_step
                .as_ref()
                .expect("must have target step")
                .beats();

        let partial = if let Some(partial_step) = &detection.partial {
            partial_step.poses.len()
        } else {
            0
        };
        full + partial
    }

    /// Returns the single tracked step.
    ///
    /// (experimenting with live instructor, I probably want to change this when cleaning up the impl)
    fn tracked_step(&self) -> StepInfo {
        let detection = self
            .intermediate_result
            .as_ref()
            .expect("requires intermediate_result");
        detection.target_step.clone().expect("requires target_step")
    }
}
