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
*/
  constructor(shoulder: Coordinate3d, hip: Coordinate3d, knee: Coordinate3d, ankle: Coordinate3d);
/**
*/
  ankle: Coordinate3d;
/**
*/
  hip: Coordinate3d;
/**
*/
  knee: Coordinate3d;
/**
*/
  shoulder: Coordinate3d;
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
*/
export class Tracker {
  free(): void;
/**
*/
  constructor();
/**
* @param {Keypoints} keypoints
* @param {number} timestamp
*/
  addKeypoints(keypoints: Keypoints, timestamp: number): void;
/**
* @param {number} start
* @param {number} end
* @returns {PoseApproximation | undefined}
*/
  bestFitPosition(start: number, end: number): PoseApproximation | undefined;
}
