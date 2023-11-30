/* tslint:disable */
/* eslint-disable */
/**
* @param {string} url
* @returns {Promise<void>}
*/
export function loadPoseFile(url: string): Promise<void>;
/**
*/
export class Coordinate3d {
  free(): void;
/**
* @param {number} x
* @param {number} y
* @param {number} z
*/
  constructor(x: number, y: number, z: number);
/**
*/
  x: number;
/**
*/
  y: number;
/**
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
* @param {Coordinate3d} shoulder
* @param {Coordinate3d} hip
* @param {Coordinate3d} knee
* @param {Coordinate3d} ankle
* @param {Coordinate3d} heel
* @param {Coordinate3d} toes
* @param {Coordinate3d} elbow
* @param {Coordinate3d} wrist
*/
  constructor(shoulder: Coordinate3d, hip: Coordinate3d, knee: Coordinate3d, ankle: Coordinate3d, heel: Coordinate3d, toes: Coordinate3d, elbow: Coordinate3d, wrist: Coordinate3d);
/**
*/
  ankle: Coordinate3d;
/**
*/
  elbow: Coordinate3d;
/**
*/
  heel: Coordinate3d;
/**
*/
  hip: Coordinate3d;
/**
*/
  knee: Coordinate3d;
/**
*/
  shoulder: Coordinate3d;
/**
*/
  toes: Coordinate3d;
/**
*/
  wrist: Coordinate3d;
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
* Projected segment, with a x-y angle and a length factor.
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
}
