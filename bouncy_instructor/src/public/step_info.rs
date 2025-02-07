use super::renderable::RenderableSkeleton;
use crate::intern::body_shift::BodyShift;
use crate::intern::pose::{BodyPart, BodyPoint};
use crate::intern::skeleton_3d::Skeleton3d;
use crate::intern::step::Step;
use crate::intern::tracker_dance_collection::TrackerDanceCollection;
use crate::skeleton::{Cartesian2d, Side, Skeleton};
use crate::STATE;

/// Information about a step for display in the frontend.
///
/// This type is no longer exposed to JS, use StepWrapper instead.
#[derive(Debug, Clone)]
pub struct StepInfo {
    // TODO: stronger typing
    id: String,
    name: String,
    step_variation: Option<String>,
    pub(crate) skeletons: Vec<Skeleton>,
    pub(crate) jump_heights: Vec<Option<f32>>,
    body_shift: BodyShift,
}

impl StepInfo {
    /// The unique identifier for the step.
    pub fn id(&self) -> String {
        self.id.clone()
    }

    /// The descriptive name for the step. The same name is used for variations
    /// of the same step.
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn skeleton(&self, beat: usize) -> Skeleton {
        if self.skeletons.is_empty() {
            Skeleton::resting(false)
        } else {
            self.skeletons[beat % self.skeletons.len()]
        }
    }

    /// How much the body position deviates from the origin.
    pub fn body_shift(&self, subbeat: usize) -> Cartesian2d {
        self.body_shift.at_subbeat(subbeat)
    }

    /// Applies a rotation (in degree) and returns the resulting skelton.
    pub fn rotated_skeleton(&self, beat: usize, rotation: f32) -> Skeleton {
        debug_assert!(!self.skeletons.is_empty());
        STATE.with_borrow(|state| {
            let step = state.step(&self.id).expect("missing step");
            let pose_index = step.poses[beat % step.poses.len()];
            let direction = step.directions[beat % step.poses.len()];
            let pose = &state.global_db.tracker_view.poses()[pose_index];
            Skeleton3d::from_with_db(pose, &state.global_db.tracker_view, direction)
                .to_skeleton(rotation)
        })
    }

    pub fn jump_height(&self, beat: usize) -> Option<f32> {
        if self.jump_heights.is_empty() {
            return None;
        }
        self.jump_heights[beat % self.jump_heights.len()]
    }

    /// Description identifier for the translated text which describes how the
    /// variation is different from the original.
    ///
    /// For example: "left-first" can be used for all steps which are the same
    /// as the original but instead of starting with the right foot, it starts
    /// with the left foot first. The app shows a translated text like "Left Leg First".
    pub fn variation(&self) -> Option<String> {
        self.step_variation.clone()
    }

    /// The number of poses the step takes for one repetition.
    pub fn num_poses(&self) -> usize {
        self.skeletons.len()
    }
}

impl StepInfo {
    pub(crate) fn from_step(step: Step, db: &TrackerDanceCollection) -> Self {
        let mut skeletons = vec![];
        let mut body_shift = BodyShift::new();

        for (pose_index, direction) in step.poses.iter().zip(&step.directions) {
            let pose = &db.poses()[*pose_index];
            skeletons.push(Skeleton::from_pose(pose, db, *direction));
        }
        body_shift.add_step(&step, &skeletons, db);

        Self {
            id: step.id,
            name: step.name,
            step_variation: step.variation,
            skeletons,
            jump_heights: step.jump_heights,
            body_shift,
        }
    }
}

impl Skeleton {
    pub(crate) fn position(&self, body_point: BodyPoint) -> Cartesian2d {
        let side = match body_point.side {
            crate::intern::pose::BodySide::Left => &self.left,
            crate::intern::pose::BodySide::Right => &self.right,
        };
        match body_point.part {
            BodyPart::Shoulder => todo!("unsupported pivot"),
            BodyPart::Hip => Cartesian2d::default(),
            BodyPart::Knee => side.knee_position(),
            BodyPart::Ankle => side.ankle_position(),
            BodyPart::Elbow => todo!("unsupported pivot"),
            BodyPart::Wrist => todo!("unsupported pivot"),
            BodyPart::Heel => side.heel_position(),
            BodyPart::Toes => side.toes_position(),
        }
    }
}
impl Side {
    #[inline]
    fn knee_position(&self) -> Cartesian2d {
        Cartesian2d::from(self.thigh) * RenderableSkeleton::THIGH_LEN
    }

    #[inline]
    fn ankle_position(&self) -> Cartesian2d {
        self.knee_position() + Cartesian2d::from(self.shin) * RenderableSkeleton::SHIN_LEN
    }

    #[inline]
    fn heel_position(&self) -> Cartesian2d {
        self.ankle_position()
    }

    #[inline]
    fn toes_position(&self) -> Cartesian2d {
        self.heel_position() + Cartesian2d::from(self.foot) * RenderableSkeleton::FOOT_LEN
    }
}
