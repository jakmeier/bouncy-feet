/* tslint:disable */
/* eslint-disable */
export function init(random_seed: number, lang: string): void;
export function loadPoseFile(url: string): Promise<void>;
export function loadPoseString(data: string): void;
export function loadDanceString(data: string): void;
export function loadStepFile(url: string, source: string): Promise<void>;
export function loadDanceFile(url: string): Promise<void>;
export function loadStepString(data: string, source: string): void;
export function parseCourseString(data: string, lang: string): Course;
export function poses(): PoseWrapper[];
export function steps(): StepWrapper[];
export function stepsBySource(source: string): StepWrapper[];
export function stepById(id: string, flipped: boolean): StepWrapper | undefined;
export function stepsByName(step_name: string): StepWrapper[];
export function dances(): DanceWrapper[];
export function danceBuilderFromDance(dance_id: string): DanceBuilder;
export function addLocalPoses(poses: PoseWrapper[]): void;
export function loadLocalSteps(steps: StepWrapper[]): void;
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
  /**
   * Nothing to track, hence nothing to detect.
   */
  NoTrackingTarget = 7,
}
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
   * The instructor is showing the next moving.
   */
  InstructorDemo = 5,
  /**
   * No longer tracking but the results of the previous tracking are
   * available.
   */
  TrackingDone = 6,
}
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
 * Best guess for what the dancer needs to change to fit the pose.
 */
export enum PoseHint {
  DontKnow = 0,
  LeftRight = 1,
  ZOrder = 2,
  WrongDirection = 3,
}
export enum SkeletonLimb {
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
export enum SkeletonPoint {
  LeftShoulder = 0,
  LeftElbow = 1,
  LeftWrist = 2,
  LeftHip = 3,
  LeftKnee = 4,
  LeftAnkle = 5,
  LeftHeel = 6,
  LeftToes = 7,
  RightShoulder = 8,
  RightElbow = 9,
  RightWrist = 10,
  RightHip = 11,
  RightKnee = 12,
  RightAnkle = 13,
  RightHeel = 14,
  RightToes = 15,
}
/**
 * Hint to the UI, which information should be shown to the user during the
 * current section.
 */
export enum TeacherView {
  /**
   * Show the instructor, no need to show the user camera.
   */
  InstructorOnly = 1,
  /**
   * The user camera should be shown with a tracked avatar.
   */
  UserCameraWithTracking = 2,
  /**
   * Show nothing
   */
  Off = 3,
}

import type { Readable } from "svelte/store";

type ReadableDetectionState = Readable<DetectionState>;


export class AudioEffect {
  private constructor();
  free(): void;
  /**
   * When the sound should be played, could be in the future.
   */
  timestamp: number;
  readonly soundId: string;
}
export class Cartesian2d {
  free(): void;
  constructor(x: number, y: number);
  add(other: Cartesian2d): Cartesian2d;
  x: number;
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
export class Course {
  private constructor();
  free(): void;
  featuredStep(): StepWrapper | undefined;
  tracker(lesson_index: number): Tracker | undefined;
  /**
   * WIP: Create a training session for the given course. At the moment, it
   * is hard coded to give something for testing.
   */
  trainingTracker(): Tracker;
  readonly id: string;
  readonly name: string;
  readonly explanation: string;
  readonly lessons: Lesson[];
}
export class DanceBuilder {
  free(): void;
  constructor(id: string);
  length(): number;
  setId(id: string): void;
  addStep(step_id: string): void;
  removeStep(pos: number): string;
  insertStep(pos: number, step_id: string): void;
  setOrientation(pos: number, flipped: boolean): void;
  isFlipped(pos: number): boolean;
  clear(): void;
  danceInfo(): DanceWrapper;
}
/**
 * Contains all information about a dance to be detected and has an interface
 * to be used by a Tracker to match tracked skeletons to it.
 */
export class DanceDetector {
  private constructor();
  free(): void;
}
export class DanceFileBuilder {
  free(): void;
  constructor();
  static fromRon(text: string): DanceFileBuilder;
  addDance(dance_builder: DanceBuilder): void;
  overwriteDance(dance_builder: DanceBuilder): void;
  removeDance(id: string): void;
  buildRon(): string;
  buildPrettyRon(): string;
  dances(): DanceWrapper[];
  danceBuilder(dance_id: string): DanceBuilder;
}
export class DanceWrapper {
  private constructor();
  free(): void;
  length(): number;
  steps(): StepWrapper[];
  skeleton(beat: number): Skeleton | undefined;
  /**
   * How much the body position deviates from the origin.
   */
  bodyShift(beat: number): Cartesian2d;
  /**
   * The unique identifier for the dance.
   */
  readonly id: string;
  /**
   * The number of subbeats the dance takes for one repetition.
   */
  readonly subbeats: number;
}
/**
 * A step detected on a video feed, ready for JS code to render.
 */
export class DetectedStep {
  private constructor();
  free(): void;
  start: number;
  end: number;
  error: number;
  readonly name: string;
  readonly poses: PoseApproximation[];
  readonly bpm: number;
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
  constructor();
  steps(): DetectedStep[];
  poseHint(): PoseHint;
  poseError(): PoseApproximation | undefined;
  /**
   * If the newest detection was negative, this fields contains information
   * about the reason.
   */
  get failureReason(): DetectionFailureReason | undefined;
  /**
   * If the newest detection was negative, this fields contains information
   * about the reason.
   */
  set failureReason(value: DetectionFailureReason | null | undefined);
  poseMatches: number;
  poseMisses: number;
}
/**
 * Information of a recorded frame in RON format.
 *
 * Can be useful for creating new poses, new keypoint inputs for tests, or just
 * for general debugging.
 */
export class ExportedFrame {
  private constructor();
  free(): void;
  readonly pose: string;
  readonly keypoints: string;
}
export class Keypoints {
  free(): void;
  constructor(left: KeypointsSide, right: KeypointsSide, fully_visible: boolean);
  left: KeypointsSide;
  right: KeypointsSide;
  fullyVisible: boolean;
}
export class KeypointsSide {
  free(): void;
  constructor(shoulder: Cartesian3d, hip: Cartesian3d, knee: Cartesian3d, ankle: Cartesian3d, heel: Cartesian3d, toes: Cartesian3d, elbow: Cartesian3d, wrist: Cartesian3d);
  shoulder: Cartesian3d;
  hip: Cartesian3d;
  knee: Cartesian3d;
  ankle: Cartesian3d;
  heel: Cartesian3d;
  toes: Cartesian3d;
  elbow: Cartesian3d;
  wrist: Cartesian3d;
}
export class Lesson {
  private constructor();
  free(): void;
  readonly name: string;
  readonly explanation: string;
  readonly video: string;
  readonly parts: LessonPart[];
  readonly iconUrl: string;
}
export class LessonPart {
  private constructor();
  free(): void;
  readonly stepName: string;
  readonly explanation: string;
  readonly step: StepWrapper;
  readonly bpms: Uint16Array;
}
/**
 * Self-describing error score for a specific limb
 */
export class LimbError {
  private constructor();
  free(): void;
  render(skeleton: SkeletonV2): RenderableSegment;
  error: number;
  weight: number;
  readonly name: string;
}
/**
 * The result of fitting keypoints to poses.
 */
export class PoseApproximation {
  private constructor();
  free(): void;
  /**
   * List all limbs, order by how well they fit, best fit first.
   */
  limbErrors(): LimbError[];
  zErrors(): ZError[];
  zOrderErrors(): ZWrongOrderError[];
  /**
   * List the `n` limbs with the highest error contribution to the pose error.
   */
  worstLimbs(n: number): LimbError[];
  debugString(): string;
  /**
   * Total error between 0.0 and 1.0.
   */
  error: number;
  /**
   * Timestamp for which Keypoints were added
   */
  timestamp: number;
  readonly id: string;
  readonly name: string;
}
export class PoseFileWrapper {
  free(): void;
  constructor();
  static fromRon(text: string): PoseFileWrapper;
  poses(): PoseWrapper[];
  addPose(new_pose: PoseWrapper): void;
  overwritePose(new_pose: PoseWrapper): void;
  removePose(id: string): void;
  buildRon(): string;
  buildPrettyRon(): string;
}
export class PoseWrapper {
  private constructor();
  free(): void;
  skeleton(): Skeleton;
  sideSkeleton(): Skeleton;
  id(): string;
  name(lang: string): string;
  setName(name: string, lang: string): void;
  setAngle(field: SkeletonLimb, degree: number): void;
  /**
   * Angle in degree
   */
  getAngle(field: SkeletonLimb): number;
  setZ(field: SkeletonPoint, z: number): void;
  getZ(field: SkeletonPoint): number;
  setWeight(field: SkeletonLimb, weight: number): void;
  /**
   * Weight of limb in pose detection
   */
  getWeight(field: SkeletonLimb): number;
  setDirection(direction: PoseDirection): void;
  readonly direction: PoseDirection;
  xShift: number;
  yShift: number;
  turnShoulder: number;
  turnHip: number;
}
/**
 * Projected line segment with two coordinates and a Z index.
 *
 * This format is perfect for 2D drawing.
 */
export class RenderableSegment {
  private constructor();
  free(): void;
  /**
   * Start of the line segment in the xy plane.
   */
  start: Cartesian2d;
  /**
   * End of the line segment in the xy plane.
   */
  end: Cartesian2d;
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
  private constructor();
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
  private constructor();
  free(): void;
  /**
   * Compute 2d coordinates for the skeleton for rendering.
   *
   * The skeleton will be rendered assuming hard-coded values for body part
   * proportional lengths, multiplied with the size parameter. The hip
   * segment will have its center at the given position.
   */
  render(hip_center: Cartesian2d, size: number): SkeletonV2;
  static resting(sideway: boolean): Skeleton;
  restingPose(): Skeleton;
  debugString(): string;
  left: SkeletonSide;
  right: SkeletonSide;
  hip: Segment;
  shoulder: Segment;
  /**
   * Does the dancer look more to the side han they face the camera?
   */
  sideway: boolean;
  /**
   * Does the dancer face away more than they face the camera?
   */
  backwards: boolean;
}
export class SkeletonSide {
  private constructor();
  free(): void;
  thigh: Segment;
  shin: Segment;
  arm: Segment;
  forearm: Segment;
  foot: Segment;
}
export class SkeletonSideV2 {
  private constructor();
  free(): void;
  thigh: RenderableSegment;
  shin: RenderableSegment;
  arm: RenderableSegment;
  forearm: RenderableSegment;
  foot: RenderableSegment;
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
  private constructor();
  free(): void;
  segment(field: SkeletonLimb): RenderableSegment;
  left: SkeletonSideV2;
  right: SkeletonSideV2;
  hip: RenderableSegment;
  shoulder: RenderableSegment;
  /**
   * Does the dancer look more to the side han they face the camera?
   */
  sideway: boolean;
  /**
   * Does the dancer face away more than they face the camera?
   */
  backwards: boolean;
}
export class SkeletonWrapper {
  private constructor();
  free(): void;
  pose(): PoseWrapper;
  skeleton(): Skeleton;
  set(): Skeleton;
}
export class Skeletons {
  private constructor();
  free(): void;
  front: Skeleton;
  side: Skeleton;
}
export class StepFileWrapper {
  free(): void;
  constructor();
  static fromRon(text: string): StepFileWrapper;
  steps(): StepWrapper[];
  addStep(new_step: StepWrapper): void;
  overwriteStep(new_step: StepWrapper): void;
  removeStep(id: string): void;
  buildRon(): string;
  buildPrettyRon(): string;
}
/**
 * Used in the editor to add and edit poses of a step.
 */
export class StepPositionBuilder {
  free(): void;
  constructor(pose: PoseWrapper);
  pose(): PoseWrapper;
  setJumpHeight(height: number): void;
  setOrientation(orientation: Orientation): void;
  readonly jumpHeight: number | undefined;
  readonly orientation: Orientation;
}
export class StepWrapper {
  free(): void;
  constructor(id: string, name: string, source: string);
  skeleton(beat: number): Skeleton;
  /**
   * How much the body position deviates from the origin.
   */
  bodyShift(beat: number): Cartesian2d;
  /**
   * Applies a rotation (in degree) and returns the resulting skelton.
   */
  rotatedSkeleton(beat: number, rotation: number): Skeleton;
  jumpHeight(beat: number): number | undefined;
  /**
   * Look up poses from the global collection, do not use for courses that
   * require a custom collection.
   */
  poses(): PoseWrapper[];
  /**
   * Positions with poses from the global collection, do not use for courses
   * that require a custom collection.
   */
  positions(): StepPositionBuilder[];
  /**
   * Add poses from the global collection, do not use for courses that
   * require a custom collection.
   */
  addPosition(position: StepPositionBuilder): void;
  removePosition(index: number): void;
  insertPosition(index: number, position: StepPositionBuilder): void;
  /**
   * The unique identifier for the step.
   */
  readonly id: string;
  /**
   * The descriptive name for the step. The same name is used for variations
   * of the same step.
   */
  name: string;
  /**
   * Description identifier for the translated text which describes how the
   * variation is different from the original.
   *
   * For example: "left-first" can be used for all steps which are the same
   * as the original but instead of starting with the right foot, it starts
   * with the left foot first. The app shows a translated text like "Left Leg First".
   */
  readonly variation: string;
  /**
   * The number of subbeats the step takes for one repetition.
   */
  readonly subbeats: number;
}
export class TextEffect {
  private constructor();
  free(): void;
  /**
   * When the text should be displayed, could be in the future.
   */
  timestamp: number;
  /**
   * How long to show the text, in ms
   */
  duration: number;
  readonly text: string;
}
/**
 * A Tracker gathers skeletons over time and passes it on to a DanceDetector.
 */
export class Tracker {
  free(): void;
  /**
   * Create a tracker for all known steps.
   */
  constructor();
  /**
   * Track one specific step, by name, including its variations (with the same name).
   */
  static StepTracker(step_name: string): Tracker;
  /**
   * Track one specific step, by ID, excluding its variations (with the same name).
   *
   * This is not intended for general dance detection but rather for a
   * specific training session without much regard for timing etc.
   */
  static UniqueStepTracker(step_id: string): Tracker;
  /**
   * Mix a warmup with the given steps, by name.
   *
   */
  static WarmUp(step_names: string[], num_beats: number): Tracker;
  finishTracking(): void;
  clear(): void;
  /**
   * Insert keypoints of a new frame for tracking.
   *
   * This is the main method to insert data into the tracker.
   */
  addKeypoints(keypoints: Keypoints, timestamp: number): Skeletons;
  setBpm(bpm: number): void;
  alignBeat(first_beat: number): void;
  enforceBeat(yes: boolean): void;
  setErrorThreshold(error_threshold: number): void;
  /**
   * Goes over all data and detects the best fitting dance.
   *
   * There is no re-use or consistency between calls. It always starts at 0
   * and computes the global best fit.
   *
   * Use [`Tracker::run_detection`] for incremental detection.
   */
  detectDance(): DetectionResult;
  runDetection(): DetectionResult;
  poseHint(): PoseHint;
  currentPoseError(): PoseApproximation | undefined;
  currentView(t: number): TeacherView;
  nextSubbeat(now?: number | null): number;
  nextAudioEffect(): AudioEffect | undefined;
  nextTextEffect(after: number): TextEffect | undefined;
  /**
   * Return a skeleton for a pose.
   */
  poseSkeleton(id: string): Skeleton | undefined;
  /**
   * Return a skeleton that's expected now.
   *
   * Only implemented to work properly for trackers of unique steps.
   *
   * (experimenting with live instructor, I probably want to change this when cleaning up the impl)
   */
  expectedPoseSkeleton(): Skeleton;
  subbeat(t: number): number;
  poseSkeletonAtSubbeat(subbeat: number): Skeleton;
  expectedPoseBodyShift(): Cartesian2d;
  poseBodyShiftAtSubbeat(beat: number): Cartesian2d;
  numDetectedPoses(): number;
  hipPosition(timestamp: number): Cartesian3d;
  /**
   * Fit frames in a time interval against all poses and return the best fit.
   *
   * This API is exported mostly for debugging. To extract fitted dances, use
   * `detect_dance` instead.
   */
  bestFitPose(start: number, end: number): PoseApproximation | undefined;
  /**
   * Fit a single frame against all poses and return all errors
   */
  allPoseErrors(timestamp: number): PoseApproximation[];
  skeletonAt(timestamp: number): Skeleton | undefined;
  skeletonWrapperAt(timestamp: number): SkeletonWrapper | undefined;
  /**
   * The original keypoints rendered as skeleton, at the given time frame.
   */
  renderedKeypointsAt(timestamp: number, width: number, height: number): SkeletonV2 | undefined;
  devSetState(state: DetectionState, timestamp: number): void;
  exportFrame(timestamp: number): ExportedFrame;
  exportKeypoints(): string;
  readonly detectionState: ReadableDetectionState;
  readonly trackedSubbeats: number;
  readonly timeBetweenPoses: number;
  readonly lastDetection: DetectionResult;
}
export class ZError {
  private constructor();
  free(): void;
  error: number;
  quadrant_error: boolean;
  readonly name: string;
}
export class ZWrongOrderError {
  private constructor();
  free(): void;
  readonly expected: string;
}
