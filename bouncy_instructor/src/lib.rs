mod intern;
mod public;
mod web_utils;

#[cfg(test)]
mod test_utils;

pub use public::*;

use intern::pose_db::LimbPositionDatabase;
use intern::step::Step;
use std::cell::RefCell;

/// Singleton internal state, shared between `Tracker` instances that run in the
/// same JS worker thread.
struct State {
    db: LimbPositionDatabase,
    steps: Vec<Step>,
}
thread_local! {
    static STATE: RefCell<State> =
        State {
            db: Default::default(),
            steps: Default::default()
        }.into();
}

impl State {
    fn add_poses(
        &mut self,
        poses: Vec<pose_file::Pose>,
    ) -> Result<(), intern::pose_db::AddPoseError> {
        self.db.add(poses)
    }

    fn add_steps(&mut self, steps: &[step_file::Step]) -> Result<(), AddStepError> {
        for def in steps {
            for frame in &def.keyframes {
                let _pose = self
                    .db
                    .pose_by_id(&frame.pose)
                    .ok_or_else(|| AddStepError::MissingPose(frame.pose.clone()))?;
            }

            let new_step = Step {
                name: def.name.clone(),
            };
            self.steps.push(new_step);
        }
        Ok(())
    }
}

#[derive(Debug)]
enum AddStepError {
    MissingPose(String),
}

impl From<AddStepError> for pose_file::ParseFileError {
    fn from(error: AddStepError) -> Self {
        match error {
            AddStepError::MissingPose(id) => Self::UnknownPoseReference(id),
        }
    }
}
