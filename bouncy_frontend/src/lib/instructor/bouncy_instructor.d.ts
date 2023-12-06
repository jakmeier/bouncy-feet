/* tslint:disable */
/* eslint-disable */
/**
* @param {string} url
* @returns {Promise<void>}
*/
export function loadPoseFile(url: string): Promise<void>;
/**
* Coordinate for Keypoints
*
* The coordinate system is growing down (y-axis), right (x-axis), and away
* from the camera (z-axis).
*
* See the Keypoints section in bouncy_instructor/coordinates.md for visuals
* and rationale.
*/
export class Cartesian3d {
  free(): void;
/**
* @param {number} x
* @param {number} y
* @param {number} z
*/
  constructor(x: number, y: number, z: number);
/**
* left-right direction
*/
  x: number;
/**
* up-down direction
*/
  y: number;
/**
* distance to camera
*/
  z: number;
}
/**
* Information of a recorded frame in RON format.
*
* Can be useful for creating new poses, new keypoint inputs for tests, or just
* for general debugging.
*/
export class ExportedFrame {
  free(): void;
/**
*/
  readonly keypoints: string;
/**
*/
  readonly pose: string;
}
/**
*/
export class Keypoints {
  free(): void;
/**
* @param {KeypointsSide} left
* @param {KeypointsSide} right
*/
  constructor(left: KeypointsSide, right: KeypointsSide);
/**
*/
  left: KeypointsSide;
/**
*/
  right: KeypointsSide;
}
/**
*/
export class KeypointsSide {
  free(): void;
/**
* @param {Cartesian3d} shoulder
* @param {Cartesian3d} hip
* @param {Cartesian3d} knee
* @param {Cartesian3d} ankle
* @param {Cartesian3d} heel
* @param {Cartesian3d} toes
* @param {Cartesian3d} elbow
* @param {Cartesian3d} wrist
*/
  constructor(shoulder: Cartesian3d, hip: Cartesian3d, knee: Cartesian3d, ankle: Cartesian3d, heel: Cartesian3d, toes: Cartesian3d, elbow: Cartesian3d, wrist: Cartesian3d);
/**
*/
  ankle: Cartesian3d;
/**
*/
  elbow: Cartesian3d;
/**
*/
  heel: Cartesian3d;
/**
*/
  hip: Cartesian3d;
/**
*/
  knee: Cartesian3d;
/**
*/
  shoulder: Cartesian3d;
/**
*/
  toes: Cartesian3d;
/**
*/
  wrist: Cartesian3d;
}
/**
* Self-describing error score for a specific limbb
*/
export class LimbError {
  free(): void;
/**
*/
  error: number;
/**
*/
  readonly name: string;
/**
*/
  weight: number;
}
/**
* The result of fitting keypoints to poses.
*/
export class PoseApproximation {
  free(): void;
/**
* List all limbs, order by how well they fit, best fit first.
* @returns {(LimbError)[]}
*/
  limbErrors(): (LimbError)[];
/**
* List the `n` limbs with the highest error contribution to the pose error.
* @param {number} n
* @returns {(LimbError)[]}
*/
  worstLimbs(n: number): (LimbError)[];
/**
* Total error between 0.0 and 1.0.
*/
  error: number;
/**
*/
  readonly name: string;
/**
* Timestamp for which Keypoints were added
*/
  timestamp: number;
}
/**
* Projected lin segment, with a x-y angle and a length factor.
*
* This format is perfect for 2D drawing.
*/
export class Segment {
  free(): void;
/**
* The 2D projected angle of the segment, counter-clock wise to the x-axis,
* in [0, 2*PI).
*/
  angle: number;
/**
* The factor to multiply lengths when drawing the projected segment in 2D.
*/
  r: number;
}
/**
* A self-sufficient description of a body position snapshot for 2d rendering.
*
* Each limb has a 2D angle in the x-y plane plus a length factor to simulate
* the third dimension in a 2D projection. X grows to the right, y grows down.
*
* This format is for exporting to other modules. JS code can easily read it
* and potentially render it.
*
* Note that the skeleton is stripped of position information, it only has
* angles of all body parts. This means it cannot be used to overlay a video.
* Use the original keypoints for such matters.
*/
export class Skeleton {
  free(): void;
/**
*/
  left: SkeletonSide;
/**
*/
  right: SkeletonSide;
}
/**
*/
export class SkeletonSide {
  free(): void;
/**
*/
  arm: Segment;
/**
*/
  forearm: Segment;
/**
*/
  shin: Segment;
/**
*/
  thigh: Segment;
}
/**
*/
export class Tracker {
  free(): void;
/**
*/
  constructor();
/**
* @param {Keypoints} keypoints
* @param {number} timestamp
* @returns {Skeleton}
*/
  addKeypoints(keypoints: Keypoints, timestamp: number): Skeleton;
/**
* Fit frames in a time interval against all poses and return the best fit.
* @param {number} start
* @param {number} end
* @returns {PoseApproximation | undefined}
*/
  bestFitPose(start: number, end: number): PoseApproximation | undefined;
/**
* Fit a single frame against all poses and return all errors
* @param {number} timestamp
* @returns {(PoseApproximation)[]}
*/
  allPoseErrors(timestamp: number): (PoseApproximation)[];
/**
* @param {number} timestamp
* @returns {ExportedFrame}
*/
  exportFrame(timestamp: number): ExportedFrame;
}
