use super::skeleton_wrapper::SkeletonWrapper;
use crate::intern::geom::SignedAngle;
use crate::intern::pose::{Limb, LimbPosition, Pose};
use crate::intern::tracker_dance_collection::TrackerDanceCollection;
use crate::skeleton::{Cartesian2d, Skeleton, SkeletonField};
use crate::{pose_file, STATE};
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug)]
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
}

impl PoseWrapper {
    pub(crate) fn new(pose_definition: pose_file::Pose) -> Self {
        Self {
            pose_definition,
            skeleton_cache: None,
        }
    }

    pub(crate) fn from_skeleton(skeleton: &SkeletonWrapper) -> Self {
        STATE.with_borrow(|state| Self {
            pose_definition: pose_file::Pose::from_with_db(
                &skeleton.skeleton_3d(),
                &state.global_db.tracker_view,
            ),
            // id: None,
            // name: None,
            // pose: None,
            // display_name_cache: None,
            skeleton_cache: None,
        })
    }

    fn invalidate_cache(&mut self) {
        self.skeleton_cache = None;
    }

    pub(crate) fn definition(&self) -> &pose_file::Pose {
        &self.pose_definition
    }

    pub(crate) fn pose(&self) -> Pose {
        // FIXME: this is hacky, copying various code from find_or_insert_limb
        // and add_poses.

        STATE.with_borrow(|state| {
            let db: &TrackerDanceCollection = &state.global_db.tracker_view;

            let def = &self.pose_definition;
            let limbs = def
                .limbs
                .iter()
                .map(|def| {
                    let limb = Limb::from(def.limb.clone());
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
        })
    }

    fn find_limb_position(&self, limb: pose_file::Limb) -> Option<&pose_file::LimbPosition> {
        self.pose_definition
            .limbs
            .iter()
            .find(|def| def.limb == limb)
    }

    fn find_or_insert_limb_position(
        &mut self,
        limb: pose_file::Limb,
    ) -> &mut pose_file::LimbPosition {
        let i = match self
            .pose_definition
            .limbs
            .iter()
            .position(|def| def.limb == limb)
        {
            Some(i) => i,
            None => {
                self.pose_definition.limbs.push(pose_file::LimbPosition {
                    limb,
                    weight: 1.0,
                    angle: 0,
                    tolerance: 10,
                });
                self.pose_definition.limbs.len() - 1
            }
        };
        &mut self.pose_definition.limbs[i]
    }
}

#[wasm_bindgen]
impl PoseWrapper {
    pub fn skeleton(&mut self) -> Skeleton {
        if self.skeleton_cache.is_none() {
            // let skeleton_3d = Skeleton3d::from_with_db(&self.pose(), &self.db, direction);
            // TODO: select direction
            STATE.with_borrow(|state| {
                let db: &TrackerDanceCollection = &state.global_db.tracker_view;
                let direction = crate::intern::skeleton_3d::Direction::East;
                let skeleton = Skeleton::from_pose(&self.pose(), db, direction);
                self.skeleton_cache = Some(skeleton);
            })
        }
        self.skeleton_cache.unwrap()
    }

    pub fn id(&self) -> String {
        self.pose_definition.id.clone()
    }

    pub fn name(&self, lang: String) -> Option<String> {
        if let Some(translated) = &self.pose_definition.names {
            translated.get(&lang)
        } else {
            None
        }
    }

    #[wasm_bindgen(js_name = "setName")]
    pub fn set_name(&mut self, name: String, lang: String) {
        let translated = self
            .pose_definition
            .names
            .get_or_insert_with(Default::default);

        translated.set(lang, name)
    }

    #[wasm_bindgen(js_name = "setAngle")]
    pub fn set_angle(&mut self, field: SkeletonField, degree: i16) {
        let limb: pose_file::Limb = field.into();
        let limb_position = self.find_or_insert_limb_position(limb);
        limb_position.angle = degree;
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

    #[wasm_bindgen(js_name = "setWeight")]
    pub fn set_weight(&mut self, field: SkeletonField, weight: f32) {
        let limb: pose_file::Limb = field.into();
        let limb_position = self.find_or_insert_limb_position(limb);
        limb_position.weight = weight;
        self.invalidate_cache();
    }

    /// Weight of limb in pose detection
    #[wasm_bindgen(js_name = "getWeight")]
    pub fn weight(&self, field: SkeletonField) -> f32 {
        let limb: pose_file::Limb = field.into();
        let limb_position = self.find_limb_position(limb);
        limb_position.map(|p| p.weight).unwrap_or(0.0)
    }
}

impl From<SkeletonField> for pose_file::Limb {
    fn from(field: SkeletonField) -> Self {
        match field {
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
        }
    }
}
