use super::keypoints::Cartesian3d;
use super::skeleton::SkeletonLimb;
use crate::intern::pose::{BodyPart, BodyPoint, BodySide};
use crate::skeleton::{Cartesian2d, Segment, Side, Skeleton};
use std::collections::HashMap;
use wasm_bindgen::prelude::wasm_bindgen;

/// A self-sufficient description of a body position snapshot for 2.5d
/// rendering.
///
/// In this format, x,y,z values have been computed to fit in a specific area,
/// assuming specific body part lengths. JS code can take it and directly draw
/// it on a Canvas or as an SVG. The z information is an integer describing draw
/// order conditions for the renderer to respect.
#[wasm_bindgen(js_name = "SkeletonV2")]
#[derive(Clone, Copy, Debug)]
pub struct RenderableSkeleton {
    pub left: RenderableSide,
    pub right: RenderableSide,
    pub hip: RenderableSegment,
    pub shoulder: RenderableSegment,
    /// Does the dancer look more to the side han they face the camera?
    pub sideway: bool,
    /// Does the dancer face away more than they face the camera?
    pub backwards: bool,
}

#[wasm_bindgen(js_name = SkeletonSideV2)]
#[derive(Clone, Copy, Debug)]
pub struct RenderableSide {
    pub thigh: RenderableSegment,
    pub shin: RenderableSegment,
    pub arm: RenderableSegment,
    pub forearm: RenderableSegment,
    pub foot: RenderableSegment,
}

/// Projected line segment with two coordinates and a Z index.
///
/// This format is perfect for 2D drawing.
#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct RenderableSegment {
    /// Start of the line segment in the xy plane.
    pub start: Cartesian2d,
    /// End of the line segment in the xy plane.
    pub end: Cartesian2d,
    /// Z-Index for draw order
    pub z: i16,
}

impl RenderableSegment {
    pub(crate) fn new(start: Cartesian2d, end: Cartesian2d, z: i16) -> Self {
        Self { start, end, z }
    }

    pub(crate) fn from_3d(start: Cartesian3d, end: Cartesian3d, width: f32, height: f32) -> Self {
        Self::new(
            Cartesian2d::new(width * start.x, height * start.y),
            Cartesian2d::new(width * end.x, height * end.y),
            ((start.z + end.z) / 2.0)
                .round()
                .max(i16::MIN as f32)
                .min(i16::MAX as f32) as i16,
        )
    }
}

#[wasm_bindgen(js_class = SkeletonV2)]
impl RenderableSkeleton {
    pub fn segment(&self, field: SkeletonLimb) -> RenderableSegment {
        match field {
            SkeletonLimb::LeftThigh => self.left.thigh,
            SkeletonLimb::LeftShin => self.left.shin,
            SkeletonLimb::LeftArm => self.left.arm,
            SkeletonLimb::LeftForearm => self.left.forearm,
            SkeletonLimb::LeftFoot => self.left.foot,
            SkeletonLimb::RightThigh => self.right.thigh,
            SkeletonLimb::RightShin => self.right.shin,
            SkeletonLimb::RightArm => self.right.arm,
            SkeletonLimb::RightForearm => self.right.forearm,
            SkeletonLimb::RightFoot => self.right.foot,
        }
    }
}

impl RenderableSkeleton {
    pub(crate) const THIGH_LEN: f32 = 0.2;
    pub(crate) const SHIN_LEN: f32 = 0.2;
    pub(crate) const TORSO_LEN: f32 = 0.25;
    pub(crate) const ARM_LEN: f32 = 0.1;
    pub(crate) const FOREARM_LEN: f32 = 0.15;
    pub(crate) const FOOT_LEN: f32 = 0.075;
    pub(crate) const SHOULDER_LEN: f32 = 0.1;
    pub(crate) const HIP_LEN: f32 = 0.075;

    /// Take already computed coordinates and create a RenderableSkeleton.
    pub(crate) fn from_coordinates(
        c: &HashMap<BodyPoint, Cartesian3d>,
        width: f32,
        height: f32,
    ) -> Self {
        let left_hip = c[&BodyPoint {
            side: BodySide::Left,
            part: BodyPart::Hip,
        }];
        let right_hip = c[&BodyPoint {
            side: BodySide::Right,
            part: BodyPart::Hip,
        }];
        let left_shoulder = c[&BodyPoint {
            side: BodySide::Left,
            part: BodyPart::Shoulder,
        }];
        let right_shoulder = c[&BodyPoint {
            side: BodySide::Right,
            part: BodyPart::Shoulder,
        }];
        let hip = RenderableSegment::from_3d(left_hip, right_hip, width, height);
        let shoulder = RenderableSegment::from_3d(left_shoulder, right_shoulder, width, height);

        RenderableSkeleton {
            left: RenderableSide::from_coordinates(BodySide::Left, c, width, height),
            right: RenderableSide::from_coordinates(BodySide::Right, c, width, height),
            hip,
            shoulder,
            // TODO: can I find sensible values for these two boolean flags?
            sideway: false,
            backwards: false,
        }
    }
}

#[wasm_bindgen]
impl Skeleton {
    /// Compute 2d coordinates for the skeleton for rendering.
    ///
    /// The skeleton will be rendered assuming hard-coded values for body part
    /// proportional lengths, multiplied with the size parameter. The hip
    /// segment will have its center at the given position.
    pub fn render(&self, hip_center: Cartesian2d, size: f32) -> RenderableSkeleton {
        let half_hip = Cartesian2d::from(self.hip) * (0.5 * RenderableSkeleton::HIP_LEN * size);
        let mut hip =
            RenderableSegment::new(hip_center - half_hip, hip_center + half_hip, self.hip.z);

        let shoulder_center =
            hip_center + Cartesian2d::new(0.0, -RenderableSkeleton::TORSO_LEN * size);
        let half_shoulder =
            Cartesian2d::from(self.shoulder) * (0.5 * RenderableSkeleton::SHOULDER_LEN * size);
        let mut shoulder = RenderableSegment::new(
            shoulder_center - half_shoulder,
            shoulder_center + half_shoulder,
            self.shoulder.z,
        );

        // when the dance looks away from the camera, we need to switch sides
        if self.backwards {
            std::mem::swap(&mut hip.start, &mut hip.end);
            std::mem::swap(&mut shoulder.start, &mut shoulder.end);
        }

        RenderableSkeleton {
            left: self.left.render(hip.start, shoulder.start, size),
            right: self.right.render(hip.end, shoulder.end, size),
            hip,
            shoulder,
            sideway: self.sideway,
            backwards: self.backwards,
        }
    }
}

impl Side {
    fn render(&self, hip: Cartesian2d, shoulder: Cartesian2d, size: f32) -> RenderableSide {
        let thigh = self.thigh.render(hip, size * RenderableSkeleton::THIGH_LEN);
        let shin = self
            .shin
            .render(thigh.end, size * RenderableSkeleton::SHIN_LEN);
        let foot = self
            .foot
            .render(shin.end, size * RenderableSkeleton::FOOT_LEN);
        let arm = self
            .arm
            .render(shoulder, size * RenderableSkeleton::ARM_LEN);
        let forearm = self
            .forearm
            .render(arm.end, size * RenderableSkeleton::FOREARM_LEN);
        RenderableSide {
            thigh,
            shin,
            arm,
            forearm,
            foot,
        }
    }
}

impl RenderableSide {
    pub(crate) fn from_coordinates(
        side: BodySide,
        c: &HashMap<BodyPoint, Cartesian3d>,
        width: f32,
        height: f32,
    ) -> Self {
        let heel = c[&BodyPoint {
            side,
            part: BodyPart::Heel,
        }];
        let toes = c[&BodyPoint {
            side,
            part: BodyPart::Toes,
        }];
        let ankle = c[&BodyPoint {
            side,
            part: BodyPart::Ankle,
        }];
        let knee = c[&BodyPoint {
            side,
            part: BodyPart::Knee,
        }];
        let hip = c[&BodyPoint {
            side,
            part: BodyPart::Hip,
        }];
        let shoulder = c[&BodyPoint {
            side,
            part: BodyPart::Shoulder,
        }];
        let elbow = c[&BodyPoint {
            side,
            part: BodyPart::Elbow,
        }];
        let wrist = c[&BodyPoint {
            side,
            part: BodyPart::Wrist,
        }];

        Self {
            thigh: RenderableSegment::from_3d(hip, knee, width, height),
            shin: RenderableSegment::from_3d(knee, ankle, width, height),
            arm: RenderableSegment::from_3d(shoulder, elbow, width, height),
            forearm: RenderableSegment::from_3d(elbow, wrist, width, height),
            foot: RenderableSegment::from_3d(heel, toes, width, height),
        }
    }
}

impl Segment {
    fn render(&self, start: Cartesian2d, len: f32) -> RenderableSegment {
        RenderableSegment {
            start,
            end: start + (Cartesian2d::from(*self) * len),
            z: self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use expect_test::expect;

    use super::*;

    #[test]
    fn test_resting_skeleton_render() {
        check_skeleton_render(
            Skeleton::resting(true),
            expect![[r#"
                RenderableSkeleton {
                    left: RenderableSide {
                        thigh: RenderableSegment {
                            start: (0.000, -3.750),
                            end: (-0.000, 16.250),
                            z: 0,
                        },
                        shin: RenderableSegment {
                            start: (-0.000, 16.250),
                            end: (-0.000, 36.250),
                            z: 0,
                        },
                        arm: RenderableSegment {
                            start: (0.000, -30.000),
                            end: (-0.000, -20.000),
                            z: 0,
                        },
                        forearm: RenderableSegment {
                            start: (-0.000, -20.000),
                            end: (-0.000, -5.000),
                            z: 0,
                        },
                        foot: RenderableSegment {
                            start: (-0.000, 36.250),
                            end: (-7.500, 36.250),
                            z: 0,
                        },
                    },
                    right: RenderableSide {
                        thigh: RenderableSegment {
                            start: (-0.000, 3.750),
                            end: (-0.000, 23.750),
                            z: 0,
                        },
                        shin: RenderableSegment {
                            start: (-0.000, 23.750),
                            end: (-0.000, 43.750),
                            z: 0,
                        },
                        arm: RenderableSegment {
                            start: (-0.000, -20.000),
                            end: (-0.000, -10.000),
                            z: 0,
                        },
                        forearm: RenderableSegment {
                            start: (-0.000, -10.000),
                            end: (-0.000, 5.000),
                            z: 0,
                        },
                        foot: RenderableSegment {
                            start: (-0.000, 43.750),
                            end: (-7.500, 43.750),
                            z: 0,
                        },
                    },
                    hip: RenderableSegment {
                        start: (0.000, -3.750),
                        end: (-0.000, 3.750),
                        z: 0,
                    },
                    shoulder: RenderableSegment {
                        start: (0.000, -30.000),
                        end: (-0.000, -20.000),
                        z: 0,
                    },
                    sideway: true,
                    backwards: false,
                }
            "#]],
        );
        check_skeleton_render(
            Skeleton::resting(false),
            expect![[r#"
                RenderableSkeleton {
                    left: RenderableSide {
                        thigh: RenderableSegment {
                            start: (3.750, 0.000),
                            end: (3.750, 20.000),
                            z: 0,
                        },
                        shin: RenderableSegment {
                            start: (3.750, 20.000),
                            end: (3.750, 40.000),
                            z: 0,
                        },
                        arm: RenderableSegment {
                            start: (5.000, -25.000),
                            end: (5.000, -15.000),
                            z: 0,
                        },
                        forearm: RenderableSegment {
                            start: (5.000, -15.000),
                            end: (5.000, 0.000),
                            z: 0,
                        },
                        foot: RenderableSegment {
                            start: (3.750, 40.000),
                            end: (7.500, 46.495),
                            z: 0,
                        },
                    },
                    right: RenderableSide {
                        thigh: RenderableSegment {
                            start: (-3.750, -0.000),
                            end: (-3.750, 20.000),
                            z: 0,
                        },
                        shin: RenderableSegment {
                            start: (-3.750, 20.000),
                            end: (-3.750, 40.000),
                            z: 0,
                        },
                        arm: RenderableSegment {
                            start: (-5.000, -25.000),
                            end: (-5.000, -15.000),
                            z: 0,
                        },
                        forearm: RenderableSegment {
                            start: (-5.000, -15.000),
                            end: (-5.000, -0.000),
                            z: 0,
                        },
                        foot: RenderableSegment {
                            start: (-3.750, 40.000),
                            end: (-7.500, 46.495),
                            z: 0,
                        },
                    },
                    hip: RenderableSegment {
                        start: (3.750, 0.000),
                        end: (-3.750, -0.000),
                        z: 0,
                    },
                    shoulder: RenderableSegment {
                        start: (5.000, -25.000),
                        end: (-5.000, -25.000),
                        z: 0,
                    },
                    sideway: false,
                    backwards: false,
                }
            "#]],
        );
    }

    fn check_skeleton_render(input: Skeleton, expect: expect_test::Expect) {
        let size = 100.0;
        let center = Cartesian2d::default();
        let output = input.render(center, size);

        expect.assert_debug_eq(&output);
    }
}
