use super::renderable::RenderableSkeleton;
use crate::intern::body_shift::BodyShift;
use crate::intern::pose::{BodyPart, BodyPoint};
use crate::intern::skeleton_3d::Skeleton3d;
use crate::intern::step::Step;
use crate::skeleton::{Cartesian2d, Side, Skeleton};
use crate::STATE;
use wasm_bindgen::prelude::wasm_bindgen;

/// Information about a step for display in the frontend.
#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct StepInfo {
    // TODO: stronger typing
    id: String,
    name: String,
    step_variation: Option<String>,
    pub(crate) skeletons: Vec<Skeleton>,
    body_shift: BodyShift,
}

#[wasm_bindgen]
impl StepInfo {
    /// The unique identifier for the step.
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.id.clone()
    }

    /// The descriptive name for the step. The same name is used for variations
    /// of the same step.
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn skeleton(&self, beat: usize) -> Skeleton {
        debug_assert!(!self.skeletons.is_empty());
        self.skeletons[beat % self.skeletons.len()]
    }

    /// How much the body position deviates from the origin.
    #[wasm_bindgen(js_name = "bodyShift")]
    pub fn body_shift(&self, beat: usize) -> Cartesian2d {
        self.body_shift.at_beat(beat)
    }

    /// Applies a rotation (in degree) and returns the resulting skelton.
    #[wasm_bindgen(js_name = "rotatedSkeleton")]
    pub fn rotated_skeleton(&self, beat: usize, rotation: f32) -> Skeleton {
        debug_assert!(!self.skeletons.is_empty());
        STATE.with_borrow(|state| {
            let step = state.step(&self.id).expect("missing step");
            let pose_index = step.poses[beat % step.poses.len()];
            let direction = step.directions[beat % step.poses.len()];
            let pose = &state.db.poses()[pose_index];
            Skeleton3d::from_with_db(pose, &state.db, direction).to_skeleton(rotation)
        })
    }

    /// Description identifier for the translated text which describes how the
    /// variation is different from the original.
    ///
    /// For example: "left-first" can be used for all steps which are the same
    /// as the original but instead of starting with the right foot, it starts
    /// with the left foot first. The app shows a translated text like "Left Leg First".
    #[wasm_bindgen(getter)]
    pub fn variation(&self) -> Option<String> {
        self.step_variation.clone()
    }

    /// The number of beats the step takes for one repetition.
    #[wasm_bindgen(getter)]
    pub fn beats(&self) -> usize {
        self.skeletons.len()
    }

    // Cursed: Sometimes I need to initiate a Rust clone from within JS to avoid
    // use-after-free.
    #[wasm_bindgen(js_name = "rustClone")]
    pub fn rust_clone(&self) -> Self {
        self.clone()
    }
}

impl From<Step> for StepInfo {
    fn from(step: Step) -> Self {
        let mut skeletons = vec![];
        let mut body_shift = BodyShift::new();

        STATE.with_borrow(|state| {
            for (pose_index, direction) in step.poses.iter().zip(&step.directions) {
                let pose = &state.db.poses()[*pose_index];
                skeletons.push(Skeleton::from_pose(pose, &state.db, *direction));
            }
            body_shift.add_step(&step, &skeletons, &state.db);
        });

        Self {
            id: step.id,
            name: step.name,
            step_variation: step.variation,
            skeletons,
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
