mod intern;
mod public;
mod web_utils;

#[cfg(test)]
mod test_utils;

pub use public::*;

use intern::dance_collection::DanceCollection;
use intern::step::Step;
use public::parsing::ParseFileError;
use std::cell::RefCell;

/// Singleton internal state, shared between `Tracker` instances that run in the
/// same JS worker thread.
struct State {
    db: DanceCollection,
}
thread_local! {
    static STATE: RefCell<State> =
        State {
            db: Default::default(),
        }.into();
}

impl State {
    fn add_poses(
        &mut self,
        poses: Vec<pose_file::Pose>,
    ) -> Result<(), intern::dance_collection::AddPoseError> {
        self.db.add(poses)
    }

    fn add_steps(&mut self, steps: &[step_file::Step]) -> Result<(), AddStepError> {
        self.db.add_steps(steps)
    }

    fn add_dances(&mut self, dances: Vec<dance_file::Dance>) -> Result<(), AddDanceError> {
        self.db.add_dances(dances)
    }

    fn step(&self, id: &str) -> Option<&Step> {
        self.db.step(id)
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
