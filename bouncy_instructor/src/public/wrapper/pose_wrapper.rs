use super::skeleton_wrapper::SkeletonWrapper;
use crate::intern::geom::SignedAngle;
use crate::intern::pose::{Limb, LimbPosition, Pose};
use crate::intern::skeleton_3d::{Direction, Skeleton3d};
use crate::intern::tracker_dance_collection::TrackerDanceCollection;
use crate::pose_file::{BodyPoint, PoseDirection};
use crate::skeleton::{Cartesian2d, Skeleton, SkeletonLimb, SkeletonPoint};
use crate::{pose_file, STATE};
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct PoseWrapper {
    /// The source of truth. Modification must only go here first and then
    /// propagate to the other fields.
    pose_definition: pose_file::Pose,

    // cached values
    // id: Option<String>,
    // name: Option<TranslatedString>,
    skeleton_cache: Option<Skeleton>,
    side_skeleton_cache: Option<Skeleton>,
    // TODO: use as cache
    // pose: Option<intern::pose::Pose>,
    // display_name_cache: Option<String>,
}

impl PoseWrapper {
    pub(crate) fn new(pose_definition: pose_file::Pose) -> Self {
        Self {
            pose_definition,
            skeleton_cache: None,
            side_skeleton_cache: None,
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
            side_skeleton_cache: None,
        })
    }

    fn invalidate_cache(&mut self) {
        self.skeleton_cache = None;
        self.side_skeleton_cache = None;
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
            STATE.with_borrow(|state| {
                let db: &TrackerDanceCollection = &state.global_db.tracker_view;
                let pose = self.pose();
                // TODO: the direction here should be set by the parent step position
                let direction = pose.direction.into();
                let skeleton = Skeleton::from_pose(&pose, db, direction);
                self.skeleton_cache = Some(skeleton);
            })
        }
        self.skeleton_cache.unwrap()
    }

    #[wasm_bindgen(js_name = "sideSkeleton")]
    pub fn side_skeleton(&mut self) -> Skeleton {
        if self.side_skeleton_cache.is_none() {
            STATE.with_borrow(|state| {
                let db: &TrackerDanceCollection = &state.global_db.tracker_view;
                let pose = self.pose();
                // TODO: the direction here should be set by the parent step position
                let direction = Direction::from(pose.direction).rotate_one();
                let rotation = 90.0;
                let skeleton = Skeleton3d::from_with_db(&pose, db, direction).to_skeleton(rotation);
                self.side_skeleton_cache = Some(skeleton);
            })
        }
        self.side_skeleton_cache.unwrap()
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
    pub fn set_angle(&mut self, field: SkeletonLimb, degree: i16) {
        let limb: pose_file::Limb = field.into();
        let limb_position = self.find_or_insert_limb_position(limb);
        limb_position.angle = degree;
        self.invalidate_cache();
    }

    /// Angle in degree
    #[wasm_bindgen(js_name = "getAngle")]
    pub fn angle(&mut self, field: SkeletonLimb) -> f32 {
        match field {
            SkeletonLimb::LeftThigh => self.skeleton().left.thigh.angle,
            SkeletonLimb::LeftShin => self.skeleton().left.shin.angle,
            SkeletonLimb::LeftArm => self.skeleton().left.arm.angle,
            SkeletonLimb::LeftForearm => self.skeleton().left.forearm.angle,
            SkeletonLimb::LeftFoot => self.skeleton().left.foot.angle,
            SkeletonLimb::RightThigh => self.skeleton().right.thigh.angle,
            SkeletonLimb::RightShin => self.skeleton().right.shin.angle,
            SkeletonLimb::RightArm => self.skeleton().right.arm.angle,
            SkeletonLimb::RightForearm => self.skeleton().right.forearm.angle,
            SkeletonLimb::RightFoot => self.skeleton().right.foot.angle,
        }
        .to_degrees()
        .round()
    }

    #[wasm_bindgen(js_name = "setZ")]
    pub fn set_z(&mut self, field: SkeletonPoint, z: f32) {
        let point: pose_file::BodyPoint = field.into();
        self.pose_definition.z.absolute.insert(point, z);
        self.invalidate_cache();
    }

    #[wasm_bindgen(js_name = "getZ")]
    pub fn get_z(&self, field: SkeletonPoint) -> f32 {
        let point: pose_file::BodyPoint = field.into();
        self.pose_definition
            .z
            .absolute
            .get(&point)
            .copied()
            .unwrap_or(0.0)
    }

    // TODO: set relative Z for limbs
    // TODO: Z should be estimated from input video

    #[wasm_bindgen(js_name = "setWeight")]
    pub fn set_weight(&mut self, field: SkeletonLimb, weight: f32) {
        let limb: pose_file::Limb = field.into();
        let limb_position = self.find_or_insert_limb_position(limb);
        limb_position.weight = weight;
        self.invalidate_cache();
    }

    /// Weight of limb in pose detection
    #[wasm_bindgen(js_name = "getWeight")]
    pub fn weight(&self, field: SkeletonLimb) -> f32 {
        let limb: pose_file::Limb = field.into();
        let limb_position = self.find_limb_position(limb);
        limb_position.map(|p| p.weight).unwrap_or(0.0)
    }

    #[wasm_bindgen(getter, js_name = direction)]
    pub fn direction(&self) -> PoseDirection {
        self.pose_definition.direction
    }

    #[wasm_bindgen(js_name = setDirection)]
    pub fn set_direction(&mut self, direction: PoseDirection) {
        self.pose_definition.direction = direction;
        self.invalidate_cache();
    }

    #[wasm_bindgen(getter, js_name = xShift)]
    pub fn x_shift(&self) -> f32 {
        self.pose_definition.x_shift
    }

    #[wasm_bindgen(setter, js_name = xShift)]
    pub fn set_x_shift(&mut self, x_shift: f32) {
        self.pose_definition.x_shift = x_shift;
        self.invalidate_cache();
    }

    #[wasm_bindgen(getter, js_name = yShift)]
    pub fn get_y_shift(&self) -> f32 {
        self.pose_definition.y_shift
    }

    #[wasm_bindgen(setter, js_name = yShift)]
    pub fn y_shift(&mut self, y_shift: f32) {
        self.pose_definition.y_shift = y_shift;
        self.invalidate_cache();
    }

    #[wasm_bindgen(getter, js_name = turnShoulder)]
    pub fn get_turn_shoulder(&self) -> i16 {
        self.pose_definition.turn_shoulder
    }

    #[wasm_bindgen(setter, js_name = turnShoulder)]
    pub fn set_turn_shoulder(&mut self, turn_shoulder: i16) {
        self.pose_definition.turn_shoulder = turn_shoulder;
        self.invalidate_cache();
    }

    #[wasm_bindgen(getter, js_name = turnHip)]
    pub fn turn_hip(&self) -> i16 {
        self.pose_definition.turn_hip
    }

    #[wasm_bindgen(setter, js_name = turnHip)]
    pub fn set_turn_hip(&mut self, turn_hip: i16) {
        self.pose_definition.turn_hip = turn_hip;
        self.invalidate_cache();
    }
}

impl From<SkeletonLimb> for pose_file::Limb {
    fn from(field: SkeletonLimb) -> Self {
        match field {
            SkeletonLimb::LeftThigh => pose_file::Limb::LeftThigh,
            SkeletonLimb::LeftShin => pose_file::Limb::LeftShin,
            SkeletonLimb::LeftArm => pose_file::Limb::LeftArm,
            SkeletonLimb::LeftForearm => pose_file::Limb::LeftForearm,
            SkeletonLimb::LeftFoot => pose_file::Limb::LeftFoot,
            SkeletonLimb::RightThigh => pose_file::Limb::RightThigh,
            SkeletonLimb::RightShin => pose_file::Limb::RightShin,
            SkeletonLimb::RightArm => pose_file::Limb::RightArm,
            SkeletonLimb::RightForearm => pose_file::Limb::RightForearm,
            SkeletonLimb::RightFoot => pose_file::Limb::RightFoot,
        }
    }
}

impl From<SkeletonPoint> for pose_file::BodyPoint {
    fn from(field: SkeletonPoint) -> Self {
        match field {
            SkeletonPoint::LeftShoulder => BodyPoint {
                side: pose_file::BodySide::Left,
                part: pose_file::BodyPart::Shoulder,
            },
            SkeletonPoint::LeftElbow => BodyPoint {
                side: pose_file::BodySide::Left,
                part: pose_file::BodyPart::Elbow,
            },
            SkeletonPoint::LeftWrist => BodyPoint {
                side: pose_file::BodySide::Left,
                part: pose_file::BodyPart::Wrist,
            },
            SkeletonPoint::LeftHip => BodyPoint {
                side: pose_file::BodySide::Left,
                part: pose_file::BodyPart::Hip,
            },
            SkeletonPoint::LeftKnee => BodyPoint {
                side: pose_file::BodySide::Left,
                part: pose_file::BodyPart::Knee,
            },
            SkeletonPoint::LeftAnkle => BodyPoint {
                side: pose_file::BodySide::Left,
                part: pose_file::BodyPart::Ankle,
            },
            SkeletonPoint::LeftHeel => BodyPoint {
                side: pose_file::BodySide::Left,
                part: pose_file::BodyPart::Heel,
            },
            SkeletonPoint::LeftToes => BodyPoint {
                side: pose_file::BodySide::Left,
                part: pose_file::BodyPart::Toes,
            },
            SkeletonPoint::RightShoulder => BodyPoint {
                side: pose_file::BodySide::Right,
                part: pose_file::BodyPart::Shoulder,
            },
            SkeletonPoint::RightElbow => BodyPoint {
                side: pose_file::BodySide::Right,
                part: pose_file::BodyPart::Elbow,
            },
            SkeletonPoint::RightWrist => BodyPoint {
                side: pose_file::BodySide::Right,
                part: pose_file::BodyPart::Wrist,
            },
            SkeletonPoint::RightHip => BodyPoint {
                side: pose_file::BodySide::Right,
                part: pose_file::BodyPart::Hip,
            },
            SkeletonPoint::RightKnee => BodyPoint {
                side: pose_file::BodySide::Right,
                part: pose_file::BodyPart::Knee,
            },
            SkeletonPoint::RightAnkle => BodyPoint {
                side: pose_file::BodySide::Right,
                part: pose_file::BodyPart::Ankle,
            },
            SkeletonPoint::RightHeel => BodyPoint {
                side: pose_file::BodySide::Right,
                part: pose_file::BodyPart::Heel,
            },
            SkeletonPoint::RightToes => BodyPoint {
                side: pose_file::BodySide::Right,
                part: pose_file::BodyPart::Toes,
            },
        }
    }
}
