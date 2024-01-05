mod intern;
mod public;
mod web_utils;

#[cfg(test)]
mod test_utils;

pub use public::*;

use intern::dance::Dance;
use intern::pose_db::LimbPositionDatabase;
use intern::skeleton_3d::Direction;
use intern::step::Step;
use public::parsing::ParseFileError;
use std::cell::RefCell;

/// Singleton internal state, shared between `Tracker` instances that run in the
/// same JS worker thread.
struct State {
    // TODO: refactor/rename the `db` field, it makes no sense
    db: LimbPositionDatabase,
    steps: Vec<Step>,
    dances: Vec<Dance>,
}
thread_local! {
    static STATE: RefCell<State> =
        State {
            db: Default::default(),
            steps: Default::default(),
            dances: Default::default(),
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
            let poses = def
                .keyframes
                .iter()
                .map(|frame| {
                    self.db
                        .pose_by_id(&frame.pose)
                        .ok_or_else(|| AddStepError::MissingPose(frame.pose.clone()))
                })
                .collect::<Result<_, _>>()?;

            let directions = def
                .keyframes
                .iter()
                .map(|frame| match frame.orientation {
                    step_file::Orientation::ToCamera => Direction::North,
                    step_file::Orientation::Right => Direction::East,
                    step_file::Orientation::Away => Direction::South,
                    step_file::Orientation::Left => Direction::West,
                    step_file::Orientation::Any => Direction::Unknown,
                })
                .collect();

            let new_step = Step {
                id: def.id.clone(),
                name: def.name.clone(),
                variation: def.variation.clone(),
                poses,
                directions,
            };
            self.steps.push(new_step);
        }
        Ok(())
    }

    fn add_dances(&mut self, dances: Vec<dance_file::Dance>) -> Result<(), AddDanceError> {
        for def in dances {
            if let Some(missing) = def
                .steps
                .iter()
                .find(|step_name| self.steps_by_name(&step_name).next().is_none())
            {
                return Err(AddDanceError::MissingStep(missing.clone()));
            }

            let dance = Dance {
                id: def.id,
                steps: def.steps,
            };
            self.dances.push(dance);
        }
        Ok(())
    }

    fn step(&self, id: &str) -> Option<&Step> {
        self.steps.iter().find(|step| step.id == id)
    }

    fn steps_by_name<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &Step> {
        self.steps.iter().filter(move |step| step.name == name)
    }

    fn first_step_by_name<'a>(&'a self, name: &'a str) -> Option<&Step> {
        self.steps_by_name(name).next()
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
