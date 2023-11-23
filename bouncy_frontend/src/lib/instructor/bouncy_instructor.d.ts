/* tslint:disable */
/* eslint-disable */
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
  add_keypoints(keypoints: Keypoints, timestamp: number): void;
}
