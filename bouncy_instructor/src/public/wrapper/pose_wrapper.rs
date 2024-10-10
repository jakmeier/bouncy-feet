use std::borrow::Borrow;
use std::rc::Rc;

use super::skeleton_wrapper::SkeletonWrapper;
use crate::intern::dance_collection::DanceCollection;
use crate::intern::geom::SignedAngle;
use crate::intern::pose::{Limb, LimbPosition, Pose};
use crate::pose_file;
use crate::skeleton::{Cartesian2d, Skeleton, SkeletonField};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct PoseWrapper {
    /// The source of truth. Modification must only go here first and then
    /// propagate to the other fields.
    pose_definition: pose_file::Pose,

    // cached values
    // id: Option<String>,
    // name: Option<TranslatedString>,
    skeleton_cache: Option<Skeleton>,
    // TODO: use as cache
    // pose: Option<intern::pose::Pose>,
    // display_name_cache: Option<String>,

    // TODO: The full DB here is a bit awkward
    pub(crate) db: Rc<DanceCollection>,
}

impl PoseWrapper {
    pub(crate) fn new(skeleton: &SkeletonWrapper) -> Self {
        Self {
            pose_definition: pose_file::Pose::from_with_db(&skeleton.skeleton_3d(), &skeleton.db),
            // id: None,
            // name: None,
            // pose: None,
            // display_name_cache: None,
            skeleton_cache: None,
            db: skeleton.db.clone(),
        }
    }

    fn invalidate_cache(&mut self) {
        self.skeleton_cache = None;
    }

    pub(crate) fn pose(&self) -> Pose {
        // FIXME: this is hacky, copying various code from find_or_insert_limb
        // and add_poses.
        let def = &self.pose_definition;
        let limbs = def
            .limbs
            .iter()
            .map(|def| {
                let limb = Limb::from(def.limb.clone());
                let db: &DanceCollection = self.db.borrow();
                let index = db
                    .limbs()
                    .find(|(_index, l)| **l == limb)
                    .expect("limb not found")
                    .0;
                LimbPosition::new(
                    index,
                    SignedAngle::degree(def.angle as f32),
                    SignedAngle::degree(def.tolerance as f32),
                    def.weight,
                )
            })
            .collect();
        let z_order = def.z.order.iter().cloned().map(From::from).collect();
        let z_absolute = def
            .z
            .absolute
            .iter()
            .map(|(k, v)| (k.clone().into(), *v))
            .collect();

        let shift = Cartesian2d::new(def.x_shift, def.y_shift);
        let turn_shoulder = SignedAngle::degree(def.turn_shoulder as f32);
        let turn_hip = SignedAngle::degree(def.turn_hip as f32);

        Pose::new(
            def.direction.into(),
            limbs,
            shift,
            turn_shoulder,
            turn_hip,
            z_absolute,
            z_order,
        )
    }
}

#[wasm_bindgen]
impl PoseWrapper {
    pub fn skeleton(&mut self) -> Skeleton {
        if self.skeleton_cache.is_none() {
            // let skeleton_3d = Skeleton3d::from_with_db(&self.pose(), &self.db, direction);
            // TODO: select direction
            let direction = crate::intern::skeleton_3d::Direction::East;
            let skeleton = Skeleton::from_pose(&self.pose(), &self.db, direction);
            self.skeleton_cache = Some(skeleton);
        }
        self.skeleton_cache.unwrap()
    }

    #[wasm_bindgen(js_name = "setAngle")]
    pub fn set_angle(&mut self, field: SkeletonField, degree: i16) {
        let limb = match field {
            SkeletonField::LeftThigh => pose_file::Limb::LeftThigh,
            SkeletonField::LeftShin => pose_file::Limb::LeftShin,
            SkeletonField::LeftArm => pose_file::Limb::LeftArm,
            SkeletonField::LeftForearm => pose_file::Limb::LeftForearm,
            SkeletonField::LeftFoot => pose_file::Limb::LeftFoot,
            SkeletonField::RightThigh => pose_file::Limb::RightThigh,
            SkeletonField::RightShin => pose_file::Limb::RightShin,
            SkeletonField::RightArm => pose_file::Limb::RightArm,
            SkeletonField::RightForearm => pose_file::Limb::RightForearm,
            SkeletonField::RightFoot => pose_file::Limb::RightFoot,
        };
        match self
            .pose_definition
            .limbs
            .iter()
            .position(|def| def.limb == limb)
        {
            Some(i) => self.pose_definition.limbs[i].angle = degree,
            None => self.pose_definition.limbs.push(pose_file::LimbPosition {
                limb,
                weight: 1.0,
                angle: degree,
                tolerance: 10,
            }),
        }
        // self.pose_definition.limbs[limb_index].angle = value;
        self.invalidate_cache();
    }

    /// Angle in degree
    #[wasm_bindgen(js_name = "getAngle")]
    pub fn angle(&mut self, field: SkeletonField) -> f32 {
        match field {
            SkeletonField::LeftThigh => self.skeleton().left.thigh.angle,
            SkeletonField::LeftShin => self.skeleton().left.shin.angle,
            SkeletonField::LeftArm => self.skeleton().left.arm.angle,
            SkeletonField::LeftForearm => self.skeleton().left.forearm.angle,
            SkeletonField::LeftFoot => self.skeleton().left.foot.angle,
            SkeletonField::RightThigh => self.skeleton().right.thigh.angle,
            SkeletonField::RightShin => self.skeleton().right.shin.angle,
            SkeletonField::RightArm => self.skeleton().right.arm.angle,
            SkeletonField::RightForearm => self.skeleton().right.forearm.angle,
            SkeletonField::RightFoot => self.skeleton().right.foot.angle,
        }
        .to_degrees()
        .round()
    }
}
