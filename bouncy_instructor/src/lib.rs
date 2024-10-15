mod intern;
mod public;
mod web_utils;

#[cfg(test)]
mod test_utils;

pub use public::*;

use intern::content_collection::ContentCollection;
use intern::step::{Step, StepSource};
use public::parsing::ParseFileError;
use std::cell::RefCell;

/// Singleton internal state, shared between `Tracker` instances that run in the
/// same JS worker thread.
struct State {
    /// The global collection of poses/steps/dances/courses.
    global_db: ContentCollection,
}
thread_local! {
    static STATE: RefCell<State> =
        State {
            global_db: Default::default(),
        }.into();
}

impl State {
    fn reset_language(&mut self, lang: String) {
        self.global_db.set_language(lang);
    }

    fn add_poses(
        &mut self,
        poses: Vec<pose_file::Pose>,
    ) -> Result<(), intern::tracker_dance_collection::AddPoseError> {
        self.global_db.add_poses(poses)
    }

    fn add_steps(
        &mut self,
        steps: Vec<step_file::Step>,
        source: String,
    ) -> Result<(), AddStepError> {
        self.global_db.add_steps(steps, StepSource::new(source))
    }

    fn add_dances(&mut self, dances: Vec<dance_file::Dance>) -> Result<(), AddDanceError> {
        self.global_db.add_dances(dances)
    }

    fn step(&self, id: &str) -> Option<&Step> {
        self.global_db.tracker_view.step(id)
    }
}

#[derive(Debug)]
enum AddStepError {
    MissingPose(String),
}

impl From<AddStepError> for ParseFileError {
    fn from(error: AddStepError) -> Self {
        match error {
            AddStepError::MissingPose(id) => Self::UnknownPoseReference(id),
        }
    }
}

#[derive(Debug)]
enum AddDanceError {
    MissingStep(String),
}

impl From<AddDanceError> for ParseFileError {
    fn from(error: AddDanceError) -> Self {
        match error {
            AddDanceError::MissingStep(name) => Self::UnknownStepName(name),
        }
    }
}
