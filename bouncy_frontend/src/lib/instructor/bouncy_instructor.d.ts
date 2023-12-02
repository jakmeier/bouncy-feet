/* tslint:disable */
/* eslint-disable */
/**
* @param {string} url
* @returns {Promise<void>}
*/
export function loadPoseFile(url: string): Promise<void>;
/**
* Coordinate for Keypoints
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
* The result of fitting keypoints to poses.
*/
export class PoseApproximation {
  free(): void;
/**
*/
  error: number;
/**
*/
  readonly name: string;
/**
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
* The 2D projected angle of the segment.
*/
  angle: number;
/**
* the factor to multiply lengths when drawing the projected segment in 2D
*/
  r: number;
}
/**
* A self-sufficient description of a body position snapshot for 2d rendering.
*
* Each limb has a 2D angle in the x-y plane plus a length factor to simulate
* the third dimension in a 2D projection.
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
* @param {number} start
* @param {number} end
* @returns {PoseApproximation | undefined}
*/
  bestFitPosition(start: number, end: number): PoseApproximation | undefined;
/**
* @param {number} timestamp
* @returns {string}
*/
  exportFrame(timestamp: number): string;
}
