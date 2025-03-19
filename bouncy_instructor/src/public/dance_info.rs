use crate::intern::body_shift::BodyShift;
use crate::intern::dance::Dance;
use crate::public::skeleton::Skeleton;
use crate::skeleton::Cartesian2d;
use crate::{StepInfo, STATE};

/// Information about a dance for display in the frontend.
///
/// No longer exposed directly, use DanceWrapper instead
#[derive(Debug, Clone)]
pub struct DanceInfo {
    pub(crate) steps: Vec<StepInfo>,
    /// invariant: total_beats is the sum of all steps beat lengths
    pub(crate) total_subbeats: usize,
    body_shift: BodyShift,
}

impl DanceInfo {
    pub fn length(&self) -> usize {
        self.steps.len()
    }

    pub fn steps(&self) -> Vec<StepInfo> {
        self.steps.clone()
    }

    pub fn skeleton(&self, beat: usize) -> Option<Skeleton> {
        if self.steps.is_empty() {
            return None;
        }
        debug_assert!(self.total_subbeats > 0);
        let mut offset = beat % self.total_subbeats;

        for step in &self.steps {
            if step.num_poses() <= offset {
                offset -= step.num_poses();
            } else {
                return Some(step.skeleton(offset));
            }
        }
        unreachable!("must find a skeleton");
    }

    /// The number of subbeats the dance takes for one repetition.
    pub fn subbeats(&self) -> usize {
        self.total_subbeats
    }

    /// How much the body position deviates from the origin.
    pub fn body_shift(&self, beat: usize) -> Cartesian2d {
        self.body_shift.after_pose(beat)
    }
}

impl From<&Dance> for DanceInfo {
    fn from(dance: &Dance) -> Self {
        let mut body_shift = BodyShift::new();
        let mut steps: Vec<StepInfo> = vec![];
        STATE.with_borrow(|state| {
            for (step_name, &flipped) in dance.step_ids.iter().zip(&dance.flip_orientation) {
                let mut step = state.step(step_name).expect("step must exist").clone();
                if flipped {
                    step = step.flipped();
                }
                let step_info = StepInfo::from_step(step.clone(), &state.global_db.tracker_view);
                body_shift.add_step(&step, &step_info.skeletons, &state.global_db.tracker_view);
                steps.push(step_info);
            }
        });
        let total_subbeats = steps.iter().map(|step| step.num_poses()).sum();

        Self {
            steps,
            total_subbeats,
            body_shift,
        }
    }
}
