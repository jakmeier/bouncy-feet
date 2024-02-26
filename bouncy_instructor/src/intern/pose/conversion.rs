//! Boilerplate code for converting between the pose definition types and the
//! internal types.
//!
//! Keeping the code here keeps other modules small and tidy. Also, it
//! encapsulates the glue between public / internal. But it needs to be in a
//! ancestor module of pose.rs because it needs access to private fields and
//! types.
//!
//! These conversions are in here.
//! 1) pose_file::Pose -> intern::pose::Pose
//! 2) intern::pose::Pose -> Skeleton3d
//! 3) Skeleton3d -> pose_file::Pose

use super::{BodyPartOrdering, Pose, PoseDirection};
use crate::intern::geom::{Angle3d, SignedAngle};
use crate::intern::pose::{BodyPart, BodyPoint, BodySide, Limb};
use crate::intern::pose_db::LimbPositionDatabase;
use crate::intern::skeleton_3d::{Direction, Skeleton3d};
use crate::pose_file;

impl From<pose_file::Limb> for Limb {
    fn from(other: pose_file::Limb) -> Self {
        match other {
            pose_file::Limb::LeftShin => Self {
                start: BodyPoint {
                    part: BodyPart::Knee,
                    side: BodySide::Left,
                },
                end: BodyPoint {
                    part: BodyPart::Ankle,
                    side: BodySide::Left,
                },
            },
            pose_file::Limb::LeftThigh => Self {
                start: BodyPoint {
                    part: BodyPart::Hip,
                    side: BodySide::Left,
                },
                end: BodyPoint {
                    part: BodyPart::Knee,
                    side: BodySide::Left,
                },
            },
            pose_file::Limb::LeftLeg => Self {
                start: BodyPoint {
                    part: BodyPart::Hip,
                    side: BodySide::Left,
                },
                end: BodyPoint {
                    part: BodyPart::Ankle,
                    side: BodySide::Left,
                },
            },
            pose_file::Limb::LeftFoot => Self {
                start: BodyPoint {
                    part: BodyPart::Heel,
                    side: BodySide::Left,
                },
                end: BodyPoint {
                    part: BodyPart::Toes,
                    side: BodySide::Left,
                },
            },
            pose_file::Limb::LeftArm => Self {
                start: BodyPoint {
                    part: BodyPart::Shoulder,
                    side: BodySide::Left,
                },
                end: BodyPoint {
                    part: BodyPart::Elbow,
                    side: BodySide::Left,
                },
            },
            pose_file::Limb::LeftForearm => Self {
                start: BodyPoint {
                    part: BodyPart::Elbow,
                    side: BodySide::Left,
                },
                end: BodyPoint {
                    part: BodyPart::Wrist,
                    side: BodySide::Left,
                },
            },
            pose_file::Limb::RightShin => Self {
                start: BodyPoint {
                    part: BodyPart::Knee,
                    side: BodySide::Right,
                },
                end: BodyPoint {
                    part: BodyPart::Ankle,
                    side: BodySide::Right,
                },
            },
            pose_file::Limb::RightThigh => Self {
                start: BodyPoint {
                    part: BodyPart::Hip,
                    side: BodySide::Right,
                },
                end: BodyPoint {
                    part: BodyPart::Knee,
                    side: BodySide::Right,
                },
            },
            pose_file::Limb::RightLeg => Self {
                start: BodyPoint {
                    part: BodyPart::Hip,
                    side: BodySide::Right,
                },
                end: BodyPoint {
                    part: BodyPart::Ankle,
                    side: BodySide::Right,
                },
            },
            pose_file::Limb::RightFoot => Self {
                start: BodyPoint {
                    part: BodyPart::Heel,
                    side: BodySide::Right,
                },
                end: BodyPoint {
                    part: BodyPart::Toes,
                    side: BodySide::Right,
                },
            },
            pose_file::Limb::RightArm => Self {
                start: BodyPoint {
                    part: BodyPart::Shoulder,
                    side: BodySide::Right,
                },
                end: BodyPoint {
                    part: BodyPart::Elbow,
                    side: BodySide::Right,
                },
            },
            pose_file::Limb::RightForearm => Self {
                start: BodyPoint {
                    part: BodyPart::Elbow,
                    side: BodySide::Right,
                },
                end: BodyPoint {
                    part: BodyPart::Wrist,
                    side: BodySide::Right,
                },
            },

            pose_file::Limb::Custom { start, end } => Limb {
                start: start.into(),
                end: end.into(),
            },
        }
    }
}

impl From<pose_file::BodyPoint> for BodyPoint {
    fn from(other: pose_file::BodyPoint) -> Self {
        Self {
            part: other.part.into(),
            side: other.side.into(),
        }
    }
}

impl From<BodyPoint> for pose_file::BodyPoint {
    fn from(other: BodyPoint) -> Self {
        Self {
            part: other.part.into(),
            side: other.side.into(),
        }
    }
}

impl From<pose_file::BodyPart> for BodyPart {
    fn from(other: pose_file::BodyPart) -> Self {
        match other {
            pose_file::BodyPart::Shoulder => Self::Shoulder,
            pose_file::BodyPart::Hip => Self::Hip,
            pose_file::BodyPart::Knee => Self::Knee,
            pose_file::BodyPart::Ankle => Self::Ankle,
            pose_file::BodyPart::Elbow => Self::Elbow,
            pose_file::BodyPart::Wrist => Self::Wrist,
            pose_file::BodyPart::Heel => Self::Heel,
            pose_file::BodyPart::Toes => Self::Toes,
        }
    }
}

impl From<BodyPart> for pose_file::BodyPart {
    fn from(other: BodyPart) -> Self {
        match other {
            BodyPart::Shoulder => Self::Shoulder,
            BodyPart::Hip => Self::Hip,
            BodyPart::Knee => Self::Knee,
            BodyPart::Ankle => Self::Ankle,
            BodyPart::Elbow => Self::Elbow,
            BodyPart::Wrist => Self::Wrist,
            BodyPart::Heel => Self::Heel,
            BodyPart::Toes => Self::Toes,
        }
    }
}

impl From<pose_file::BodyPartOrdering> for BodyPartOrdering {
    fn from(other: pose_file::BodyPartOrdering) -> Self {
        Self {
            forward: other.forward.into(),
            backward: other.backward.into(),
        }
    }
}

impl From<BodyPartOrdering> for pose_file::BodyPartOrdering {
    fn from(other: BodyPartOrdering) -> Self {
        Self {
            forward: other.forward.into(),
            backward: other.backward.into(),
        }
    }
}

impl From<pose_file::BodySide> for BodySide {
    fn from(other: pose_file::BodySide) -> Self {
        match other {
            pose_file::BodySide::Left => Self::Left,
            pose_file::BodySide::Right => Self::Right,
        }
    }
}

impl From<BodySide> for pose_file::BodySide {
    fn from(other: BodySide) -> Self {
        match other {
            BodySide::Left => Self::Left,
            BodySide::Right => Self::Right,
        }
    }
}

impl From<pose_file::PoseDirection> for PoseDirection {
    fn from(other: pose_file::PoseDirection) -> Self {
        match other {
            pose_file::PoseDirection::Front => PoseDirection::Front,
            pose_file::PoseDirection::Right => PoseDirection::Right,
        }
    }
}

impl From<PoseDirection> for pose_file::PoseDirection {
    fn from(other: PoseDirection) -> Self {
        match other {
            PoseDirection::Front => pose_file::PoseDirection::Front,
            PoseDirection::Right => pose_file::PoseDirection::Right,
        }
    }
}

impl From<Direction> for PoseDirection {
    fn from(other: Direction) -> Self {
        match other {
            Direction::North | Direction::South => Self::Front,
            Direction::East | Direction::West => Self::Right,
            Direction::Unknown => Self::Front,
        }
    }
}

impl From<Direction> for pose_file::PoseDirection {
    fn from(other: Direction) -> Self {
        PoseDirection::from(other).into()
    }
}

impl pose_file::Pose {
    /// Take a skeleton and compute a matching pose definition for it.
    pub(crate) fn from_with_db(skeleton: &Skeleton3d, db: &LimbPositionDatabase) -> Self {
        let limbs = skeleton
            .angles()
            .iter()
            .zip(db.limbs())
            .map(
                |(angle, (_limb_index, &limb))| crate::pose_file::LimbPosition {
                    limb: limb.into(),
                    weight: 1.0,
                    angle: angle.as_degree().round() as i16,
                    tolerance: 0,
                },
            )
            .collect::<Vec<_>>();
        Self {
            direction: skeleton.direction().into(),
            limbs,
            name: "Generated Pose".to_owned(),
            mirror_of: String::new(),
            z: Vec::new(),
        }
    }
}

impl From<Limb> for pose_file::Limb {
    fn from(other: Limb) -> Self {
        use BodyPart::{Ankle, Elbow, Heel, Hip, Knee, Shoulder, Toes, Wrist};
        use BodySide::{Left, Right};
        match (
            other.start.side,
            other.start.part,
            other.end.side,
            other.end.part,
        ) {
            (Left, Knee, Left, Ankle) => pose_file::Limb::LeftShin,
            (Left, Hip, Left, Knee) => pose_file::Limb::LeftThigh,
            (Left, Hip, Left, Ankle) => pose_file::Limb::LeftLeg,
            (Left, Heel, Left, Toes) => pose_file::Limb::LeftFoot,
            (Left, Shoulder, Left, Elbow) => pose_file::Limb::LeftArm,
            (Left, Elbow, Left, Wrist) => pose_file::Limb::LeftForearm,
            (Right, Knee, Right, Ankle) => pose_file::Limb::RightShin,
            (Right, Hip, Right, Knee) => pose_file::Limb::RightThigh,
            (Right, Hip, Right, Ankle) => pose_file::Limb::RightLeg,
            (Right, Heel, Right, Toes) => pose_file::Limb::RightFoot,
            (Right, Shoulder, Right, Elbow) => pose_file::Limb::RightArm,
            (Right, Elbow, Right, Wrist) => pose_file::Limb::RightForearm,
            _ => Self::Custom {
                start: other.start.into(),
                end: other.end.into(),
            },
        }
    }
}

impl Skeleton3d {
    /// Creates a skeleton with all limbs set to perfectly match the pose.
    /// Angles which are not defined in the pose are set to 0.
    pub(crate) fn from_with_db(pose: &Pose, db: &LimbPositionDatabase) -> Self {
        let num_limbs = db.limbs().count();
        let mut limb_angles = vec![Angle3d::ZERO; num_limbs];
        let azimuth = SignedAngle::degree(90.0);
        let direction = match pose.direction {
            PoseDirection::Front => Direction::North,
            PoseDirection::Right => Direction::East,
        };
        for limb in &pose.limbs {
            limb_angles[limb.limb.as_usize()] = Angle3d::new(azimuth, limb.target.angle());
        }
        let azimuth_correction = SignedAngle::ZERO;
        Skeleton3d::new(
            direction,
            limb_angles,
            azimuth_correction,
            Default::default(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::intern::pose_db::LimbPositionDatabase;
    use crate::test_utils::assert_angle_eq;

    use super::*;

    #[test]
    fn test_skeleton_to_pose() {
        // azimuth, polar => angle
        check_skeleton_to_pose(0.0, 0.0, 0);
        check_skeleton_to_pose(0.0, 90.0, 0);
        check_skeleton_to_pose(90.0, 90.0, 90);
        check_skeleton_to_pose(35.26, 60.0, 45);

        check_skeleton_to_pose(180.0, 90.0, 0);
        check_skeleton_to_pose(-90.0, 90.0, -90);
        check_skeleton_to_pose(-180.0 + 35.26, 60.0, -45);
        check_skeleton_to_pose(-35.26, 60.0, -45);
        check_skeleton_to_pose(180.0 - 35.26, 60.0, 45);
    }

    #[track_caller]
    fn check_skeleton_to_pose(azimuth: f32, polar: f32, angle: i16) {
        let skeleton = Skeleton3d::from_angles(
            vec![Angle3d::degree(azimuth, polar)],
            SignedAngle::degree(90.0),
            Default::default(),
        );
        let db = LimbPositionDatabase::test(Angle3d::degree(azimuth, polar));

        let pose = pose_file::Pose::from_with_db(&skeleton, &db);
        assert_eq!(
            1,
            pose.limbs.len(),
            "test DB should produce one limb skeleton"
        );
        assert_eq!(
            angle, pose.limbs[0].angle,
            "wrong angle, expected {angle:?} was {:?}",
            pose.limbs[0]
        );
    }

    #[test]
    fn test_pose_from_file() {
        // input angle, is_front => want angle
        check_pose_from_file(0, true);
        check_pose_from_file(0, false);
        check_pose_from_file(30, true);
        check_pose_from_file(-30, true);
        check_pose_from_file(-30, false);
        check_pose_from_file(-30, false);
    }

    #[track_caller]
    fn check_pose_from_file(angle: i16, is_front: bool) {
        let input = ron_string_pose(angle, is_front, false);
        // Note: With the current definition format, there is no conversion
        // happening, just the original should be in. But this might change
        // again.
        let want = angle as f32;

        let mut db = LimbPositionDatabase::default();
        let input_pose: pose_file::Pose = ron::from_str(&input).unwrap();
        db.add(vec![input_pose]).unwrap();

        assert_eq!(db.poses().len(), 1, "test expects only 1 pose");
        let pose = db.poses().first().unwrap();
        assert_eq!(pose.limbs.len(), 1, "test expects only 1 limb in pose");
        let expected = SignedAngle::degree(want);
        assert_angle_eq(expected, pose.limbs[0].target.angle());
    }

    #[test]
    fn test_pose_conversion_circle_1() {
        check_pose_conversion_circle(90, true, false);
        check_pose_conversion_circle(90, false, false);
        check_pose_conversion_circle(70, true, false);
        check_pose_conversion_circle(20, false, false);
    }

    #[test]
    fn test_pose_conversion_circle_2() {
        check_pose_conversion_circle(-90, true, true);
        check_pose_conversion_circle(-90, false, true);
        check_pose_conversion_circle(-30, true, true);
        check_pose_conversion_circle(-60, false, true);
    }

    #[test]
    fn test_pose_conversion_circle_3() {
        check_pose_conversion_circle(120, true, false);
        check_pose_conversion_circle(120, false, false);
        check_pose_conversion_circle(135, true, false);
        check_pose_conversion_circle(-135, false, false);
    }

    /// Ensure a pose_file::Pose -> Pose -> Skeleton3d -> pose_file::Pose
    /// returns an output equivalent to the input.
    #[track_caller]
    fn check_pose_conversion_circle(angle: i16, is_front: bool, add_z: bool) {
        let input = ron_string_pose(angle, is_front, add_z);

        let mut db = LimbPositionDatabase::default();
        let input_pose: pose_file::Pose = ron::from_str(&input).unwrap();
        db.add(vec![input_pose]).unwrap();
        let input_pose: pose_file::Pose = ron::from_str(&input).unwrap();

        let mut poses = db
            .poses()
            .iter()
            .map(|pose| Skeleton3d::from_with_db(pose, &db))
            .map(|skeleton| pose_file::Pose::from_with_db(&skeleton, &db));

        let output_pose = poses.next().unwrap();

        // This is expected to not work, as there is currently no way for the
        // generated pose to know which body parts to look for.
        // assert_eq!(
        //     &input_pose.z, &output_pose.z,
        //     "input Z does not match output Z"
        // );

        // the output contains all limbs, even those not part of the pose
        // remove all zero angle to reduce it to only the relevant angles
        let output = output_pose
            .limbs
            .iter()
            .filter(|limb| limb.angle != 0)
            .cloned()
            .collect::<Vec<_>>();

        assert_eq!(&input_pose.limbs, &output, "input does not match output");
        assert!(poses.next().is_none(), "more poses than expected");
    }

    fn ron_string_pose(angle: i16, front: bool, add_z: bool) -> String {
        let direction = if front { "Front" } else { "Right" };
        let z = if add_z {
            "z: [(forward: (side: Right, part: Ankle), backward: (side: Left, part: Ankle))]"
        } else {
            ""
        };
        format!(
            r#"(
            name: "Generated Pose",
            direction: {direction},
            limbs: [
              (limb: LeftThigh, angle: {angle}, tolerance: 0, weight: 1.0),
            ],
            {z}
          )"#
        )
    }
}
