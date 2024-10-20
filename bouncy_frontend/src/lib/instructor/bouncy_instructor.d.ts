/* tslint:disable */
/* eslint-disable */
/**
* @param {number} random_seed
* @param {string} lang
*/
export function init(random_seed: number, lang: string): void;
/**
* @param {string} url
* @returns {Promise<void>}
*/
export function loadPoseFile(url: string): Promise<void>;
/**
* @param {string} data
*/
export function loadPoseString(data: string): void;
/**
* @param {string} data
*/
export function loadDanceString(data: string): void;
/**
* @param {string} url
* @param {string} source
* @returns {Promise<void>}
*/
export function loadStepFile(url: string, source: string): Promise<void>;
/**
* @param {string} url
* @returns {Promise<void>}
*/
export function loadDanceFile(url: string): Promise<void>;
/**
* @param {string} data
* @param {string} source
*/
export function loadStepString(data: string, source: string): void;
/**
* @param {string} data
* @param {string} lang
* @returns {Course}
*/
export function parseCourseString(data: string, lang: string): Course;
/**
* @returns {(PoseWrapper)[]}
*/
export function poses(): (PoseWrapper)[];
/**
* @returns {(StepWrapper)[]}
*/
export function steps(): (StepWrapper)[];
/**
* @param {string} source
* @returns {(StepWrapper)[]}
*/
export function stepsBySource(source: string): (StepWrapper)[];
/**
* @param {string} id
* @param {boolean} flipped
* @returns {StepWrapper | undefined}
*/
export function stepById(id: string, flipped: boolean): StepWrapper | undefined;
/**
* @param {string} step_name
* @returns {(StepWrapper)[]}
*/
export function stepsByName(step_name: string): (StepWrapper)[];
/**
* @returns {(DanceWrapper)[]}
*/
export function dances(): (DanceWrapper)[];
/**
* @param {string} dance_id
* @returns {DanceBuilder}
*/
export function danceBuilderFromDance(dance_id: string): DanceBuilder;
/**
* @param {(PoseWrapper)[]} poses
*/
export function addLocalPoses(poses: (PoseWrapper)[]): void;
/**
* @param {(StepWrapper)[]} steps
*/
export function loadLocalSteps(steps: (StepWrapper)[]): void;
/**
* Define in which direction a pose should be oriented.
*/
export enum Orientation {
  ToCamera = 0,
  Right = 1,
  Away = 2,
  Left = 3,
/**
* It doesn't matter in which direction the pose is done.
*/
  Any = 4,
}
/**
*/
export enum DetectionState {
/**
* Neutral state, not detecting anything.
*/
  Init = 1,
/**
* Dance is positioning themselves, detecting the idle position.
*/
  Positioning = 2,
/**
* About to go over to live tracking, playing a countdown audio.
*/
  CountDown = 3,
/**
* Tracking current movements.
*/
  LiveTracking = 4,
/**
* No longer tracking but the results of the previous tracking are
* available.
*/
  TrackingDone = 5,
}
/**
*/
export enum DetectionFailureReason {
/**
* The last match was too recent to have another match.
*/
  TooEarly = 1,
/**
* The timing is off.
*/
  NotOnBeat = 2,
/**
* Detection did not match an expected pose.
*/
  WrongPose = 3,
/**
* No data to run detection against.
*/
  NoData = 4,
/**
* Currently in a state that does not detect.
*/
  DetectionDisabled = 5,
/**
* No *new* data to run detection against.
*/
  NoNewData = 6,
}
/**
* Best guess for what the dancer needs to change to fit the pose.
*/
export enum PoseHint {
  DontKnow = 0,
  LeftRight = 1,
  ZOrder = 2,
  WrongDirection = 3,
}
/**
*/
export enum PoseDirection {
/**
* Dancer faces the camera.
*/
  Front = 0,
/**
* Dancer faces to their right. (Left in non-mirrored video.)
*/
  Right = 1,
}
/**
*/
export enum SkeletonField {
  LeftThigh = 0,
  LeftShin = 1,
  LeftArm = 2,
  LeftForearm = 3,
  LeftFoot = 4,
  RightThigh = 5,
  RightShin = 6,
  RightArm = 7,
  RightForearm = 8,
  RightFoot = 9,
}

import type { Readable } from "svelte/store";

type ReadableDetectionState = Readable<DetectionState>;


/**
*/
export class AudioEffect {
  free(): void;
/**
*/
  readonly soundId: string;
/**
* When the sound should be played, could be in the future.
*/
  timestamp: number;
}
/**
*/
export class Cartesian2d {
  free(): void;
/**
* @param {number} x
* @param {number} y
*/
  constructor(x: number, y: number);
/**
* @param {Cartesian2d} other
* @returns {Cartesian2d}
*/
  add(other: Cartesian2d): Cartesian2d;
/**
*/
  x: number;
/**
*/
  y: number;
}
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
*/
export class Course {
  free(): void;
/**
* @returns {StepWrapper | undefined}
*/
  featuredStep(): StepWrapper | undefined;
/**
* @param {number} lesson_index
* @returns {Tracker | undefined}
*/
  tracker(lesson_index: number): Tracker | undefined;
/**
*/
  readonly explanation: string;
/**
*/
  readonly id: string;
/**
*/
  readonly lessons: (Lesson)[];
/**
*/
  readonly name: string;
}
/**
*/
export class DanceBuilder {
  free(): void;
/**
* @param {string} id
*/
  constructor(id: string);
/**
* @returns {number}
*/
  length(): number;
/**
* @param {string} id
*/
  setId(id: string): void;
/**
* @param {string} step_id
*/
  addStep(step_id: string): void;
/**
* @param {number} pos
* @returns {string}
*/
  removeStep(pos: number): string;
/**
* @param {number} pos
* @param {string} step_id
*/
  insertStep(pos: number, step_id: string): void;
/**
* @param {number} pos
* @param {boolean} flipped
*/
  setOrientation(pos: number, flipped: boolean): void;
/**
* @param {number} pos
* @returns {boolean}
*/
  isFlipped(pos: number): boolean;
/**
*/
  clear(): void;
/**
* @returns {DanceWrapper}
*/
  danceInfo(): DanceWrapper;
}
/**
* Contains all information about a dance to be detected and has an interface
* to be used by a Tracker to match tracked skeletons to it.
*/
export class DanceDetector {
  free(): void;
}
/**
*/
export class DanceFileBuilder {
  free(): void;
/**
*/
  constructor();
/**
* @param {string} text
* @returns {DanceFileBuilder}
*/
  static fromRon(text: string): DanceFileBuilder;
/**
* @param {DanceBuilder} dance_builder
*/
  addDance(dance_builder: DanceBuilder): void;
/**
* @param {DanceBuilder} dance_builder
*/
  overwriteDance(dance_builder: DanceBuilder): void;
/**
* @param {string} id
*/
  removeDance(id: string): void;
/**
* @returns {string}
*/
  buildRon(): string;
/**
* @returns {(DanceWrapper)[]}
*/
  dances(): (DanceWrapper)[];
/**
* @param {string} dance_id
* @returns {DanceBuilder}
*/
  danceBuilder(dance_id: string): DanceBuilder;
}
/**
*/
export class DanceWrapper {
  free(): void;
/**
* @returns {number}
*/
  length(): number;
/**
* @returns {(StepWrapper)[]}
*/
  steps(): (StepWrapper)[];
/**
* @param {number} beat
* @returns {Skeleton | undefined}
*/
  skeleton(beat: number): Skeleton | undefined;
/**
* How much the body position deviates from the origin.
* @param {number} beat
* @returns {Cartesian2d}
*/
  bodyShift(beat: number): Cartesian2d;
/**
* The number of beats the dance takes for one repetition.
*/
  readonly beats: number;
/**
* The unique identifier for the dance.
*/
  readonly id: string;
}
/**
* A step detected on a video feed, ready for JS code to render.
*/
export class DetectedStep {
  free(): void;
/**
*/
  readonly bpm: number;
/**
*/
  end: number;
/**
*/
  error: number;
/**
*/
  readonly name: string;
/**
*/
  readonly poses: (PoseApproximation)[];
/**
*/
  start: number;
}
/**
* Result of a step or dance detection.
*
* A detection potentially includes a list of steps. It can be displayed in the
* frontend as is, or provided to a tracker to update the detection after more
* data has been added.
*/
export class DetectionResult {
  free(): void;
/**
*/
  constructor();
/**
* @returns {(DetectedStep)[]}
*/
  steps(): (DetectedStep)[];
/**
* @returns {PoseHint}
*/
  poseHint(): PoseHint;
/**
* @returns {PoseApproximation | undefined}
*/
  poseError(): PoseApproximation | undefined;
/**
* If the newest detection was negative, this fields contains information
* about the reason.
*/
  failureReason?: DetectionFailureReason;
/**
*/
  poseMatches: number;
/**
*/
  poseMisses: number;
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
* @param {boolean} fully_visible
*/
  constructor(left: KeypointsSide, right: KeypointsSide, fully_visible: boolean);
/**
*/
  fullyVisible: boolean;
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
*/
export class Lesson {
  free(): void;
/**
*/
  readonly explanation: string;
/**
*/
  readonly iconUrl: string;
/**
*/
  readonly name: string;
/**
*/
  readonly parts: (LessonPart)[];
/**
*/
  readonly video: string;
}
/**
*/
export class LessonPart {
  free(): void;
/**
*/
  readonly bpms: Uint16Array;
/**
*/
  readonly explanation: string;
/**
*/
  readonly step: StepWrapper;
/**
*/
  readonly stepName: string;
}
/**
* Self-describing error score for a specific limb
*/
export class LimbError {
  free(): void;
/**
* @param {SkeletonV2} skeleton
* @returns {RenderableSegment}
*/
  render(skeleton: SkeletonV2): RenderableSegment;
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
* @returns {(ZError)[]}
*/
  zErrors(): (ZError)[];
/**
* @returns {(ZWrongOrderError)[]}
*/
  zOrderErrors(): (ZWrongOrderError)[];
/**
* List the `n` limbs with the highest error contribution to the pose error.
* @param {number} n
* @returns {(LimbError)[]}
*/
  worstLimbs(n: number): (LimbError)[];
/**
* @returns {string}
*/
  debugString(): string;
/**
* Total error between 0.0 and 1.0.
*/
  error: number;
/**
*/
  readonly id: string;
/**
*/
  readonly name: string;
/**
* Timestamp for which Keypoints were added
*/
  timestamp: number;
}
/**
*/
export class PoseFileWrapper {
  free(): void;
/**
*/
  constructor();
/**
* @param {string} text
* @returns {PoseFileWrapper}
*/
  static fromRon(text: string): PoseFileWrapper;
/**
* @returns {(PoseWrapper)[]}
*/
  poses(): (PoseWrapper)[];
/**
* @param {PoseWrapper} new_pose
*/
  addPose(new_pose: PoseWrapper): void;
/**
* @param {PoseWrapper} new_pose
*/
  overwritePose(new_pose: PoseWrapper): void;
/**
* @param {string} id
*/
  removePose(id: string): void;
/**
* @returns {string}
*/
  buildRon(): string;
}
/**
*/
export class PoseWrapper {
  free(): void;
/**
* @returns {Skeleton}
*/
  skeleton(): Skeleton;
/**
* @returns {string}
*/
  id(): string;
/**
* @param {string} lang
* @returns {string}
*/
  name(lang: string): string;
/**
* @param {string} name
* @param {string} lang
*/
  setName(name: string, lang: string): void;
/**
* @param {SkeletonField} field
* @param {number} degree
*/
  setAngle(field: SkeletonField, degree: number): void;
/**
* Angle in degree
* @param {SkeletonField} field
* @returns {number}
*/
  getAngle(field: SkeletonField): number;
/**
* @param {SkeletonField} field
* @param {number} weight
*/
  setWeight(field: SkeletonField, weight: number): void;
/**
* Weight of limb in pose detection
* @param {SkeletonField} field
* @returns {number}
*/
  getWeight(field: SkeletonField): number;
/**
* @param {PoseDirection} direction
*/
  setDirection(direction: PoseDirection): void;
/**
*/
  readonly direction: PoseDirection;
/**
*/
  turnHip: number;
/**
*/
  turnShoulder: number;
/**
*/
  xShift: number;
/**
*/
  yShift: number;
}
/**
* Projected line segment with two coordinates and a Z index.
*
* This format is perfect for 2D drawing.
*/
export class RenderableSegment {
  free(): void;
/**
* End of the line segment in the xy plane.
*/
  end: Cartesian2d;
/**
* Start of the line segment in the xy plane.
*/
  start: Cartesian2d;
/**
* Z-Index for draw order
*/
  z: number;
}
/**
* Projected line segment, with a x-y angle and a length factor.
*
* This format is usable for 2D drawing.
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
/**
* Z-Index for draw ordering
*/
  z: number;
}
/**
* A position- and size-independent description of a body pose snapshot for 2d
* rendering. An intermediate step for [`RenderableSkeleton`].
*
* Each limb has a 2D angle in the x-y plane plus a length factor to simulate
* the third dimension in a 2D projection. X grows to the right, y grows down.
* Plus, there is a z-index for the order in which segments should be drawn.
*
* This format is for exporting to other modules. JS code can easily read it
* and potentially render it.
*
* Note that the skeleton is stripped of position information, it only has
* angles of all body parts. This means it cannot be used to overlay a video.
* Use the original keypoints for such matters.
*
* TODO: I  don't think there is a good reason to expose internals of this. JS
* should only worry about final coordinates, which it gets from the
* RenderableSkeleton struct.
*/
export class Skeleton {
  free(): void;
/**
* @param {boolean} sideway
* @returns {Skeleton}
*/
  static resting(sideway: boolean): Skeleton;
/**
* @returns {Skeleton}
*/
  restingPose(): Skeleton;
/**
* @returns {string}
*/
  debugString(): string;
/**
* Compute 2d coordinates for the skeleton for rendering.
*
* The skeleton will be rendered assuming hard-coded values for body part
* proportional lengths, multiplied with the size parameter. The hip
* segment will have its center at the given position.
* @param {Cartesian2d} hip_center
* @param {number} size
* @returns {SkeletonV2}
*/
  render(hip_center: Cartesian2d, size: number): SkeletonV2;
/**
* Does the dancer face away more than they face the camera?
*/
  backwards: boolean;
/**
*/
  hip: Segment;
/**
*/
  left: SkeletonSide;
/**
*/
  right: SkeletonSide;
/**
*/
  shoulder: Segment;
/**
* Does the dancer look more to the side han they face the camera?
*/
  sideway: boolean;
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
  foot: Segment;
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
export class SkeletonSideV2 {
  free(): void;
/**
*/
  arm: RenderableSegment;
/**
*/
  foot: RenderableSegment;
/**
*/
  forearm: RenderableSegment;
/**
*/
  shin: RenderableSegment;
/**
*/
  thigh: RenderableSegment;
}
/**
* A self-sufficient description of a body position snapshot for 2.5d
* rendering.
*
* In this format, x,y,z values have been computed to fit in a specific area,
* assuming specific body part lengths. JS code can take it and directly draw
* it on a Canvas or as an SVG. The z information is an integer describing draw
* order conditions for the renderer to respect.
*/
export class SkeletonV2 {
  free(): void;
/**
* @param {SkeletonField} field
* @returns {RenderableSegment}
*/
  segment(field: SkeletonField): RenderableSegment;
/**
* Does the dancer face away more than they face the camera?
*/
  backwards: boolean;
/**
*/
  hip: RenderableSegment;
/**
*/
  left: SkeletonSideV2;
/**
*/
  right: SkeletonSideV2;
/**
*/
  shoulder: RenderableSegment;
/**
* Does the dancer look more to the side han they face the camera?
*/
  sideway: boolean;
}
/**
*/
export class SkeletonWrapper {
  free(): void;
/**
* @returns {PoseWrapper}
*/
  pose(): PoseWrapper;
/**
* @returns {Skeleton}
*/
  skeleton(): Skeleton;
/**
* @returns {Skeleton}
*/
  set(): Skeleton;
}
/**
*/
export class Skeletons {
  free(): void;
/**
*/
  front: Skeleton;
/**
*/
  side: Skeleton;
}
/**
*/
export class StepFileWrapper {
  free(): void;
/**
*/
  constructor();
/**
* @param {string} text
* @returns {StepFileWrapper}
*/
  static fromRon(text: string): StepFileWrapper;
/**
* @returns {(StepWrapper)[]}
*/
  steps(): (StepWrapper)[];
/**
* @param {StepWrapper} new_step
*/
  addStep(new_step: StepWrapper): void;
/**
* @param {StepWrapper} new_step
*/
  overwriteStep(new_step: StepWrapper): void;
/**
* @param {string} id
*/
  removeStep(id: string): void;
/**
* @returns {string}
*/
  buildRon(): string;
}
/**
* Used in the editor to add and edit poses of a step.
*/
export class StepPositionBuilder {
  free(): void;
/**
* @param {PoseWrapper} pose
*/
  constructor(pose: PoseWrapper);
/**
* @returns {PoseWrapper}
*/
  pose(): PoseWrapper;
/**
* @param {number} height
*/
  setJumpHeight(height: number): void;
/**
* @param {Orientation} orientation
*/
  setOrientation(orientation: Orientation): void;
/**
*/
  readonly jumpHeight: number | undefined;
/**
*/
  readonly orientation: Orientation;
}
/**
*/
export class StepWrapper {
  free(): void;
/**
* @param {string} id
* @param {string} name
* @param {string} source
*/
  constructor(id: string, name: string, source: string);
/**
* @param {number} beat
* @returns {Skeleton}
*/
  skeleton(beat: number): Skeleton;
/**
* How much the body position deviates from the origin.
* @param {number} beat
* @returns {Cartesian2d}
*/
  bodyShift(beat: number): Cartesian2d;
/**
* Applies a rotation (in degree) and returns the resulting skelton.
* @param {number} beat
* @param {number} rotation
* @returns {Skeleton}
*/
  rotatedSkeleton(beat: number, rotation: number): Skeleton;
/**
* @param {number} beat
* @returns {number | undefined}
*/
  jumpHeight(beat: number): number | undefined;
/**
* Look up poses from the global collection, do not use for courses that
* require a custom collection.
* @returns {(PoseWrapper)[]}
*/
  poses(): (PoseWrapper)[];
/**
* Positions with poses from the global collection, do not use for courses
* that require a custom collection.
* @returns {(StepPositionBuilder)[]}
*/
  positions(): (StepPositionBuilder)[];
/**
* Add poses from the global collection, do not use for courses that
* require a custom collection.
* @param {StepPositionBuilder} position
*/
  addPosition(position: StepPositionBuilder): void;
/**
* @param {number} index
*/
  removePosition(index: number): void;
/**
* @param {number} index
* @param {StepPositionBuilder} position
*/
  insertPosition(index: number, position: StepPositionBuilder): void;
/**
* The number of beats the step takes for one repetition.
*/
  readonly beats: number;
/**
* The unique identifier for the step.
*/
  readonly id: string;
/**
* The descriptive name for the step. The same name is used for variations
* of the same step.
*/
  readonly name: string;
/**
* Description identifier for the translated text which describes how the
* variation is different from the original.
*
* For example: "left-first" can be used for all steps which are the same
* as the original but instead of starting with the right foot, it starts
* with the left foot first. The app shows a translated text like "Left Leg First".
*/
  readonly variation: string;
}
/**
* A Tracker gathers skeletons over time and passes it on to a DanceDetector.
*/
export class Tracker {
  free(): void;
/**
* @param {number} timestamp
* @returns {ExportedFrame}
*/
  exportFrame(timestamp: number): ExportedFrame;
/**
* @returns {string}
*/
  exportKeypoints(): string;
/**
* Create a tracker for all known steps.
*/
  constructor();
/**
* Track one specific step, by name, including its variations (with the same name).
* @param {string} step_name
* @returns {Tracker}
*/
  static StepTracker(step_name: string): Tracker;
/**
* Track one specific step, by ID, excluding its variations (with the same name).
*
* This is not intended for general dance detection but rather for a
* specific training session without much regard for timing etc.
* @param {string} step_id
* @returns {Tracker}
*/
  static UniqueStepTracker(step_id: string): Tracker;
/**
*/
  finishTracking(): void;
/**
*/
  clear(): void;
/**
* Insert keypoints of a new frame for tracking.
*
* This is the main method to insert data into the tracker.
* @param {Keypoints} keypoints
* @param {number} timestamp
* @returns {Skeletons}
*/
  addKeypoints(keypoints: Keypoints, timestamp: number): Skeletons;
/**
* @param {number} bpm
*/
  setBpm(bpm: number): void;
/**
* @param {number} first_beat
*/
  alignBeat(first_beat: number): void;
/**
* @param {boolean} yes
*/
  enforceBeat(yes: boolean): void;
/**
* @param {number} error_threshold
*/
  setErrorThreshold(error_threshold: number): void;
/**
* Goes over all data and detects the best fitting dance.
*
* There is no re-use or consistency between calls. It always starts at 0
* and computes the global best fit.
*
* Use [`Tracker::run_detection`] for incremental detection.
* @returns {DetectionResult}
*/
  detectDance(): DetectionResult;
/**
* @returns {DetectionResult}
*/
  runDetection(): DetectionResult;
/**
* @returns {PoseHint}
*/
  poseHint(): PoseHint;
/**
* @returns {PoseApproximation | undefined}
*/
  currentPoseError(): PoseApproximation | undefined;
/**
* @param {number | undefined} [now]
* @returns {number}
*/
  nextHalfBeat(now?: number): number;
/**
* @returns {AudioEffect | undefined}
*/
  nextAudioEffect(): AudioEffect | undefined;
/**
* Return a skeleton for a pose.
* @param {string} id
* @returns {Skeleton | undefined}
*/
  poseSkeleton(id: string): Skeleton | undefined;
/**
* Return a skeleton that's expected now.
*
* Only implemented to work properly for trackers of unique steps.
*
* (experimenting with live instructor, I probably want to change this when cleaning up the impl)
* @returns {Skeleton}
*/
  expectedPoseSkeleton(): Skeleton;
/**
* @param {number} t
* @returns {number}
*/
  beat(t: number): number;
/**
* @param {number} beat
* @returns {Skeleton}
*/
  poseSkeletonAtBeat(beat: number): Skeleton;
/**
* @returns {Cartesian2d}
*/
  expectedPoseBodyShift(): Cartesian2d;
/**
* @param {number} beat
* @returns {Cartesian2d}
*/
  poseBodyShiftAtBeat(beat: number): Cartesian2d;
/**
* @returns {number}
*/
  numDetectedPoses(): number;
/**
* @param {number} timestamp
* @returns {Cartesian3d}
*/
  hipPosition(timestamp: number): Cartesian3d;
/**
* Fit frames in a time interval against all poses and return the best fit.
*
* This API is exported mostly for debugging. To extract fitted dances, use
* `detect_dance` instead.
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
* @returns {Skeleton | undefined}
*/
  skeletonAt(timestamp: number): Skeleton | undefined;
/**
* @param {number} timestamp
* @returns {SkeletonWrapper | undefined}
*/
  skeletonWrapperAt(timestamp: number): SkeletonWrapper | undefined;
/**
* The original keypoints rendered as skeleton, at the given time frame.
* @param {number} timestamp
* @param {number} width
* @param {number} height
* @returns {SkeletonV2 | undefined}
*/
  renderedKeypointsAt(timestamp: number, width: number, height: number): SkeletonV2 | undefined;
/**
*/
  readonly detectionState: ReadableDetectionState;
/**
*/
  readonly halfSpeed: boolean;
/**
*/
  readonly lastDetection: DetectionResult;
/**
*/
  readonly timeBetweenPoses: number;
/**
*/
  readonly trackedBeats: number | undefined;
}
/**
*/
export class ZError {
  free(): void;
/**
*/
  error: number;
/**
*/
  readonly name: string;
/**
*/
  quadrant_error: boolean;
}
/**
*/
export class ZWrongOrderError {
  free(): void;
/**
*/
  readonly expected: string;
}
