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
   * Show nothing
   */
  Off = 0,
  /**
   * Show the instructor, no need to show the user camera.
   */
  InstructorOnly = 1,
  /**
   * The user camera should be shown with a tracked avatar.
   */
  UserCameraWithTracking = 2,
  /**
   * Show the camera (without tracking) and the instructor.
   */
  InstructorAndCamera = 3,
  /**
   * Show just the plain camera.
   */
  CameraOnly = 4,
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
 * A pointer to a pose inside a tracked activity or dance.
 *
 * Can be used to reference a pose independently of pace.
 * Useful also for checking if a pose changed.
 */
export class DanceCursor {
  free(): void;
  constructor();
  /**
   * Whether both cursors show to the same pose slot.
   *
   * Note: If the same pose is repeated and two cursors show to two different
   * duplicated, this function will count them as two different poses.
   */
  isSamePose(other: DanceCursor): boolean;
  isSameSubbeat(other: DanceCursor): boolean;
  /**
   * Global counter of subbeat within an activity.
   */
  subbeat: number;
  /**
   * Points to a section within an activity.
   */
  sectionIndex: number;
  /**
   * Points to a step within an activity.
   */
  stepIndex: number;
  /**
   * Points to a pose within a step.
   */
  poseIndex: number;
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
  cursor(): DanceCursor;
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
  energy: number;
  difficulty: number;
  readonly name: string;
  readonly explanation: string;
  readonly video: string;
  readonly song: string;
  readonly parts: LessonPart[];
}
export class LessonPart {
  private constructor();
  free(): void;
  readonly stepName: string;
  readonly step: StepWrapper;
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
  static resting(sideway: boolean): Skeleton;
  restingPose(): Skeleton;
  debugString(): string;
  /**
   * Compute 2d coordinates for the skeleton for rendering.
   *
   * The skeleton will be rendered assuming hard-coded values for body part
   * proportional lengths, multiplied with the size parameter. The hip
   * segment will have its center at the given position.
   */
  render(hip_center: Cartesian2d, size: number): SkeletonV2;
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
   * How long the tracked activity is in total, measured in milliseconds.
   */
  duration(): number;
  /**
   * Return a skeleton that's expected now.
   *
   * Only implemented to work properly for trackers of unique steps.
   *
   * (experimenting with live instructor, I probably want to change this when cleaning up the impl)
   */
  expectedPoseSkeleton(): Skeleton;
  expectedJumpHeight(): number;
  subbeat(t: number): number;
  /**
   * Return a cursor to a pose inside the tracker by timestamp.
   *
   * If `looped` is true, the subbeat wraps around when exceeding the tracked range.
   */
  cursor(t: number, looped: boolean): DanceCursor;
  /**
   * Return a cursor to a pose inside the tracker by beat count.
   *
   * If `looped` is true, the subbeat wraps around when exceeding the tracked range.
   */
  cursorAtSubbeat(subbeat: number, looped: boolean): DanceCursor;
  poseSkeletonAt(cursor: DanceCursor): Skeleton;
  jumpHeight(cursor: DanceCursor): number;
  expectedPoseBodyShift(): Cartesian2d;
  poseBodyShift(cursor: DanceCursor): Cartesian2d;
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

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_keypoints_free: (a: number, b: number) => void;
  readonly __wbg_get_keypoints_left: (a: number) => number;
  readonly __wbg_set_keypoints_left: (a: number, b: number) => void;
  readonly __wbg_get_keypoints_right: (a: number) => number;
  readonly __wbg_set_keypoints_right: (a: number, b: number) => void;
  readonly __wbg_get_keypoints_fullyVisible: (a: number) => number;
  readonly __wbg_set_keypoints_fullyVisible: (a: number, b: number) => void;
  readonly __wbg_keypointsside_free: (a: number, b: number) => void;
  readonly __wbg_get_keypointsside_shoulder: (a: number) => number;
  readonly __wbg_set_keypointsside_shoulder: (a: number, b: number) => void;
  readonly __wbg_get_keypointsside_hip: (a: number) => number;
  readonly __wbg_set_keypointsside_hip: (a: number, b: number) => void;
  readonly __wbg_get_keypointsside_knee: (a: number) => number;
  readonly __wbg_set_keypointsside_knee: (a: number, b: number) => void;
  readonly __wbg_get_keypointsside_ankle: (a: number) => number;
  readonly __wbg_set_keypointsside_ankle: (a: number, b: number) => void;
  readonly __wbg_get_keypointsside_heel: (a: number) => number;
  readonly __wbg_set_keypointsside_heel: (a: number, b: number) => void;
  readonly __wbg_get_keypointsside_toes: (a: number) => number;
  readonly __wbg_set_keypointsside_toes: (a: number, b: number) => void;
  readonly __wbg_get_keypointsside_elbow: (a: number) => number;
  readonly __wbg_set_keypointsside_elbow: (a: number, b: number) => void;
  readonly __wbg_get_keypointsside_wrist: (a: number) => number;
  readonly __wbg_set_keypointsside_wrist: (a: number, b: number) => void;
  readonly __wbg_cartesian3d_free: (a: number, b: number) => void;
  readonly __wbg_get_cartesian3d_x: (a: number) => number;
  readonly __wbg_set_cartesian3d_x: (a: number, b: number) => void;
  readonly __wbg_get_cartesian3d_y: (a: number) => number;
  readonly __wbg_set_cartesian3d_y: (a: number, b: number) => void;
  readonly __wbg_get_cartesian3d_z: (a: number) => number;
  readonly __wbg_set_cartesian3d_z: (a: number, b: number) => void;
  readonly keypoints_new: (a: number, b: number, c: number) => number;
  readonly keypointsside_new: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => number;
  readonly cartesian3d_new: (a: number, b: number, c: number) => number;
  readonly __wbg_detectionresult_free: (a: number, b: number) => void;
  readonly __wbg_get_detectionresult_failureReason: (a: number) => number;
  readonly __wbg_set_detectionresult_failureReason: (a: number, b: number) => void;
  readonly __wbg_get_detectionresult_poseMatches: (a: number) => number;
  readonly __wbg_set_detectionresult_poseMatches: (a: number, b: number) => void;
  readonly __wbg_get_detectionresult_poseMisses: (a: number) => number;
  readonly __wbg_set_detectionresult_poseMisses: (a: number, b: number) => void;
  readonly detectionresult_new_default: () => number;
  readonly detectionresult_steps: (a: number, b: number) => void;
  readonly detectionresult_cursor: (a: number) => number;
  readonly detectionresult_poseHint: (a: number) => number;
  readonly detectionresult_poseError: (a: number) => number;
  readonly __wbg_dancecursor_free: (a: number, b: number) => void;
  readonly __wbg_get_dancecursor_subbeat: (a: number) => number;
  readonly __wbg_set_dancecursor_subbeat: (a: number, b: number) => void;
  readonly __wbg_get_dancecursor_sectionIndex: (a: number) => number;
  readonly __wbg_set_dancecursor_sectionIndex: (a: number, b: number) => void;
  readonly __wbg_get_dancecursor_stepIndex: (a: number) => number;
  readonly __wbg_set_dancecursor_stepIndex: (a: number, b: number) => void;
  readonly __wbg_get_dancecursor_poseIndex: (a: number) => number;
  readonly __wbg_set_dancecursor_poseIndex: (a: number, b: number) => void;
  readonly dancecursor_new: () => number;
  readonly dancecursor_isSamePose: (a: number, b: number) => number;
  readonly dancecursor_isSameSubbeat: (a: number, b: number) => number;
  readonly __wbg_dancefilebuilder_free: (a: number, b: number) => void;
  readonly dancefilebuilder_new: () => number;
  readonly dancefilebuilder_fromRon: (a: number, b: number, c: number) => void;
  readonly dancefilebuilder_addDance: (a: number, b: number, c: number) => void;
  readonly dancefilebuilder_overwriteDance: (a: number, b: number, c: number) => void;
  readonly dancefilebuilder_removeDance: (a: number, b: number, c: number, d: number) => void;
  readonly dancefilebuilder_buildRon: (a: number, b: number) => void;
  readonly dancefilebuilder_buildPrettyRon: (a: number, b: number) => void;
  readonly dancefilebuilder_dances: (a: number, b: number) => void;
  readonly dancefilebuilder_danceBuilder: (a: number, b: number, c: number, d: number) => void;
  readonly __wbg_tracker_free: (a: number, b: number) => void;
  readonly __wbg_skeletons_free: (a: number, b: number) => void;
  readonly __wbg_get_skeletons_front: (a: number) => number;
  readonly __wbg_set_skeletons_front: (a: number, b: number) => void;
  readonly __wbg_get_skeletons_side: (a: number) => number;
  readonly __wbg_set_skeletons_side: (a: number, b: number) => void;
  readonly tracker_new_from_global_collection: () => number;
  readonly tracker_StepTracker: (a: number, b: number, c: number) => void;
  readonly tracker_UniqueStepTracker: (a: number, b: number, c: number) => void;
  readonly tracker_WarmUp: (a: number, b: number, c: number, d: number) => void;
  readonly tracker_finishTracking: (a: number) => void;
  readonly tracker_clear: (a: number) => void;
  readonly tracker_addKeypoints: (a: number, b: number, c: number) => number;
  readonly tracker_setBpm: (a: number, b: number) => void;
  readonly tracker_alignBeat: (a: number, b: number) => void;
  readonly tracker_enforceBeat: (a: number, b: number) => void;
  readonly tracker_setErrorThreshold: (a: number, b: number) => void;
  readonly tracker_detectDance: (a: number) => number;
  readonly tracker_runDetection: (a: number) => number;
  readonly tracker_poseHint: (a: number) => number;
  readonly tracker_currentPoseError: (a: number) => number;
  readonly tracker_currentView: (a: number, b: number) => number;
  readonly tracker_detectionState: (a: number) => number;
  readonly tracker_trackedSubbeats: (a: number) => number;
  readonly tracker_nextSubbeat: (a: number, b: number, c: number) => number;
  readonly tracker_timeBetweenPoses: (a: number) => number;
  readonly tracker_nextAudioEffect: (a: number) => number;
  readonly tracker_nextTextEffect: (a: number, b: number) => number;
  readonly tracker_duration: (a: number) => number;
  readonly tracker_expectedPoseSkeleton: (a: number) => number;
  readonly tracker_expectedJumpHeight: (a: number) => number;
  readonly tracker_subbeat: (a: number, b: number) => number;
  readonly tracker_cursor: (a: number, b: number, c: number) => number;
  readonly tracker_cursorAtSubbeat: (a: number, b: number, c: number) => number;
  readonly tracker_poseSkeletonAt: (a: number, b: number) => number;
  readonly tracker_jumpHeight: (a: number, b: number) => number;
  readonly tracker_expectedPoseBodyShift: (a: number) => number;
  readonly tracker_poseBodyShift: (a: number, b: number) => number;
  readonly tracker_lastDetection: (a: number) => number;
  readonly tracker_hipPosition: (a: number, b: number) => number;
  readonly tracker_bestFitPose: (a: number, b: number, c: number) => number;
  readonly tracker_allPoseErrors: (a: number, b: number, c: number) => void;
  readonly tracker_skeletonAt: (a: number, b: number) => number;
  readonly tracker_skeletonWrapperAt: (a: number, b: number) => number;
  readonly tracker_renderedKeypointsAt: (a: number, b: number, c: number, d: number) => number;
  readonly tracker_devSetState: (a: number, b: number, c: number) => void;
  readonly __wbg_steppositionbuilder_free: (a: number, b: number) => void;
  readonly steppositionbuilder_new: (a: number) => number;
  readonly steppositionbuilder_pose: (a: number) => number;
  readonly steppositionbuilder_jumpHeight: (a: number) => number;
  readonly steppositionbuilder_setJumpHeight: (a: number, b: number) => void;
  readonly steppositionbuilder_orientation: (a: number) => number;
  readonly steppositionbuilder_setOrientation: (a: number, b: number) => void;
  readonly __wbg_dancewrapper_free: (a: number, b: number) => void;
  readonly dancewrapper_id: (a: number, b: number) => void;
  readonly dancewrapper_length: (a: number) => number;
  readonly dancewrapper_steps: (a: number, b: number) => void;
  readonly dancewrapper_skeleton: (a: number, b: number) => number;
  readonly dancewrapper_subbeats: (a: number) => number;
  readonly dancewrapper_bodyShift: (a: number, b: number) => number;
  readonly __wbg_posefilewrapper_free: (a: number, b: number) => void;
  readonly posefilewrapper_new_empty: () => number;
  readonly posefilewrapper_fromRon: (a: number, b: number, c: number) => void;
  readonly posefilewrapper_poses: (a: number, b: number) => void;
  readonly posefilewrapper_addPose: (a: number, b: number, c: number) => void;
  readonly posefilewrapper_overwritePose: (a: number, b: number, c: number) => void;
  readonly posefilewrapper_removePose: (a: number, b: number, c: number, d: number) => void;
  readonly posefilewrapper_buildRon: (a: number, b: number) => void;
  readonly posefilewrapper_buildPrettyRon: (a: number, b: number) => void;
  readonly __wbg_skeletonwrapper_free: (a: number, b: number) => void;
  readonly skeletonwrapper_pose: (a: number) => number;
  readonly skeletonwrapper_set: (a: number) => number;
  readonly __wbg_stepfilewrapper_free: (a: number, b: number) => void;
  readonly stepfilewrapper_new_empty: () => number;
  readonly stepfilewrapper_fromRon: (a: number, b: number, c: number) => void;
  readonly stepfilewrapper_steps: (a: number, b: number) => void;
  readonly stepfilewrapper_addStep: (a: number, b: number, c: number) => void;
  readonly stepfilewrapper_overwriteStep: (a: number, b: number, c: number) => void;
  readonly stepfilewrapper_removeStep: (a: number, b: number, c: number, d: number) => void;
  readonly stepfilewrapper_buildRon: (a: number, b: number) => void;
  readonly stepfilewrapper_buildPrettyRon: (a: number, b: number) => void;
  readonly skeletonwrapper_skeleton: (a: number) => number;
  readonly __wbg_course_free: (a: number, b: number) => void;
  readonly __wbg_lesson_free: (a: number, b: number) => void;
  readonly __wbg_get_lesson_energy: (a: number) => number;
  readonly __wbg_set_lesson_energy: (a: number, b: number) => void;
  readonly __wbg_get_lesson_difficulty: (a: number) => number;
  readonly __wbg_set_lesson_difficulty: (a: number, b: number) => void;
  readonly __wbg_lessonpart_free: (a: number, b: number) => void;
  readonly course_id: (a: number, b: number) => void;
  readonly course_name: (a: number, b: number) => void;
  readonly course_explanation: (a: number, b: number) => void;
  readonly course_lessons: (a: number, b: number) => void;
  readonly course_featuredStep: (a: number) => number;
  readonly course_tracker: (a: number, b: number) => number;
  readonly course_trainingTracker: (a: number) => number;
  readonly lesson_name: (a: number, b: number) => void;
  readonly lesson_explanation: (a: number, b: number) => void;
  readonly lesson_video: (a: number, b: number) => void;
  readonly lesson_song: (a: number, b: number) => void;
  readonly lesson_parts: (a: number, b: number) => void;
  readonly lessonpart_stepName: (a: number, b: number) => void;
  readonly lessonpart_step: (a: number) => number;
  readonly __wbg_audioeffect_free: (a: number, b: number) => void;
  readonly __wbg_get_audioeffect_timestamp: (a: number) => number;
  readonly __wbg_set_audioeffect_timestamp: (a: number, b: number) => void;
  readonly __wbg_texteffect_free: (a: number, b: number) => void;
  readonly __wbg_get_texteffect_duration: (a: number) => number;
  readonly __wbg_set_texteffect_duration: (a: number, b: number) => void;
  readonly audioeffect_soundId: (a: number, b: number) => void;
  readonly texteffect_text: (a: number, b: number) => void;
  readonly __wbg_stepwrapper_free: (a: number, b: number) => void;
  readonly stepwrapper_new_empty: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly stepwrapper_id: (a: number, b: number) => void;
  readonly stepwrapper_name: (a: number, b: number) => void;
  readonly stepwrapper_set_name: (a: number, b: number, c: number) => void;
  readonly stepwrapper_skeleton: (a: number, b: number) => number;
  readonly stepwrapper_bodyShift: (a: number, b: number) => number;
  readonly stepwrapper_rotatedSkeleton: (a: number, b: number, c: number) => number;
  readonly stepwrapper_jumpHeight: (a: number, b: number) => number;
  readonly stepwrapper_variation: (a: number, b: number) => void;
  readonly stepwrapper_subbeats: (a: number) => number;
  readonly stepwrapper_poses: (a: number, b: number) => void;
  readonly stepwrapper_positions: (a: number, b: number) => void;
  readonly stepwrapper_addPosition: (a: number, b: number) => void;
  readonly stepwrapper_removePosition: (a: number, b: number, c: number) => void;
  readonly stepwrapper_insertPosition: (a: number, b: number, c: number, d: number) => void;
  readonly __wbg_get_texteffect_timestamp: (a: number) => number;
  readonly __wbg_set_texteffect_timestamp: (a: number, b: number) => void;
  readonly __wbg_dancedetector_free: (a: number, b: number) => void;
  readonly cartesian2d_new: (a: number, b: number) => number;
  readonly cartesian2d_add: (a: number, b: number) => number;
  readonly __wbg_dancebuilder_free: (a: number, b: number) => void;
  readonly dancebuilder_new: (a: number, b: number) => number;
  readonly dancebuilder_length: (a: number) => number;
  readonly dancebuilder_setId: (a: number, b: number, c: number) => void;
  readonly dancebuilder_addStep: (a: number, b: number, c: number) => void;
  readonly dancebuilder_removeStep: (a: number, b: number, c: number) => void;
  readonly dancebuilder_insertStep: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly dancebuilder_setOrientation: (a: number, b: number, c: number, d: number) => void;
  readonly dancebuilder_isFlipped: (a: number, b: number, c: number) => void;
  readonly dancebuilder_clear: (a: number) => void;
  readonly dancebuilder_danceInfo: (a: number) => number;
  readonly __wbg_skeleton_free: (a: number, b: number) => void;
  readonly __wbg_get_skeleton_left: (a: number) => number;
  readonly __wbg_set_skeleton_left: (a: number, b: number) => void;
  readonly __wbg_get_skeleton_right: (a: number) => number;
  readonly __wbg_set_skeleton_right: (a: number, b: number) => void;
  readonly __wbg_get_skeleton_hip: (a: number) => number;
  readonly __wbg_set_skeleton_hip: (a: number, b: number) => void;
  readonly __wbg_get_skeleton_shoulder: (a: number) => number;
  readonly __wbg_set_skeleton_shoulder: (a: number, b: number) => void;
  readonly __wbg_get_skeleton_sideway: (a: number) => number;
  readonly __wbg_set_skeleton_sideway: (a: number, b: number) => void;
  readonly __wbg_get_skeleton_backwards: (a: number) => number;
  readonly __wbg_set_skeleton_backwards: (a: number, b: number) => void;
  readonly __wbg_skeletonside_free: (a: number, b: number) => void;
  readonly __wbg_get_skeletonside_thigh: (a: number) => number;
  readonly __wbg_set_skeletonside_thigh: (a: number, b: number) => void;
  readonly __wbg_get_skeletonside_shin: (a: number) => number;
  readonly __wbg_set_skeletonside_shin: (a: number, b: number) => void;
  readonly __wbg_get_skeletonside_arm: (a: number) => number;
  readonly __wbg_set_skeletonside_arm: (a: number, b: number) => void;
  readonly __wbg_get_skeletonside_forearm: (a: number) => number;
  readonly __wbg_set_skeletonside_forearm: (a: number, b: number) => void;
  readonly __wbg_get_skeletonside_foot: (a: number) => number;
  readonly __wbg_set_skeletonside_foot: (a: number, b: number) => void;
  readonly __wbg_segment_free: (a: number, b: number) => void;
  readonly __wbg_get_segment_z: (a: number) => number;
  readonly __wbg_set_segment_z: (a: number, b: number) => void;
  readonly __wbg_cartesian2d_free: (a: number, b: number) => void;
  readonly __wbg_get_cartesian2d_x: (a: number) => number;
  readonly __wbg_set_cartesian2d_x: (a: number, b: number) => void;
  readonly __wbg_get_cartesian2d_y: (a: number) => number;
  readonly __wbg_set_cartesian2d_y: (a: number, b: number) => void;
  readonly skeleton_resting: (a: number) => number;
  readonly skeleton_restingPose: (a: number) => number;
  readonly skeleton_debugString: (a: number, b: number) => void;
  readonly __wbg_exportedframe_free: (a: number, b: number) => void;
  readonly tracker_exportFrame: (a: number, b: number) => number;
  readonly tracker_exportKeypoints: (a: number, b: number) => void;
  readonly exportedframe_pose: (a: number, b: number) => void;
  readonly exportedframe_keypoints: (a: number, b: number) => void;
  readonly __wbg_poseapproximation_free: (a: number, b: number) => void;
  readonly __wbg_get_poseapproximation_error: (a: number) => number;
  readonly __wbg_set_poseapproximation_error: (a: number, b: number) => void;
  readonly __wbg_get_poseapproximation_timestamp: (a: number) => number;
  readonly __wbg_set_poseapproximation_timestamp: (a: number, b: number) => void;
  readonly __wbg_limberror_free: (a: number, b: number) => void;
  readonly __wbg_get_limberror_error: (a: number) => number;
  readonly __wbg_set_limberror_error: (a: number, b: number) => void;
  readonly __wbg_get_limberror_weight: (a: number) => number;
  readonly __wbg_set_limberror_weight: (a: number, b: number) => void;
  readonly __wbg_zerror_free: (a: number, b: number) => void;
  readonly __wbg_get_zerror_quadrant_error: (a: number) => number;
  readonly __wbg_set_zerror_quadrant_error: (a: number, b: number) => void;
  readonly __wbg_zwrongordererror_free: (a: number, b: number) => void;
  readonly poseapproximation_id: (a: number, b: number) => void;
  readonly poseapproximation_name: (a: number, b: number) => void;
  readonly poseapproximation_limbErrors: (a: number, b: number) => void;
  readonly poseapproximation_zErrors: (a: number, b: number) => void;
  readonly poseapproximation_zOrderErrors: (a: number, b: number) => void;
  readonly poseapproximation_worstLimbs: (a: number, b: number, c: number) => void;
  readonly poseapproximation_debugString: (a: number, b: number) => void;
  readonly limberror_name: (a: number, b: number) => void;
  readonly limberror_render: (a: number, b: number, c: number) => void;
  readonly zerror_name: (a: number, b: number) => void;
  readonly zwrongordererror_expected: (a: number, b: number) => void;
  readonly init: (a: number, b: number, c: number, d: number) => void;
  readonly loadPoseFile: (a: number, b: number) => number;
  readonly loadPoseString: (a: number, b: number, c: number) => void;
  readonly loadDanceString: (a: number, b: number, c: number) => void;
  readonly loadStepFile: (a: number, b: number, c: number, d: number) => number;
  readonly loadDanceFile: (a: number, b: number) => number;
  readonly loadStepString: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly parseCourseString: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly poses: (a: number) => void;
  readonly steps: (a: number) => void;
  readonly stepsBySource: (a: number, b: number, c: number) => void;
  readonly stepById: (a: number, b: number, c: number) => number;
  readonly stepsByName: (a: number, b: number, c: number) => void;
  readonly dances: (a: number) => void;
  readonly danceBuilderFromDance: (a: number, b: number, c: number) => void;
  readonly addLocalPoses: (a: number, b: number) => void;
  readonly loadLocalSteps: (a: number, b: number) => void;
  readonly __wbg_get_segment_angle: (a: number) => number;
  readonly __wbg_get_segment_r: (a: number) => number;
  readonly __wbg_get_zerror_error: (a: number) => number;
  readonly __wbg_set_segment_angle: (a: number, b: number) => void;
  readonly __wbg_set_segment_r: (a: number, b: number) => void;
  readonly __wbg_set_zerror_error: (a: number, b: number) => void;
  readonly __wbg_detectedstep_free: (a: number, b: number) => void;
  readonly __wbg_get_detectedstep_start: (a: number) => number;
  readonly __wbg_set_detectedstep_start: (a: number, b: number) => void;
  readonly __wbg_get_detectedstep_end: (a: number) => number;
  readonly __wbg_set_detectedstep_end: (a: number, b: number) => void;
  readonly __wbg_get_detectedstep_error: (a: number) => number;
  readonly __wbg_set_detectedstep_error: (a: number, b: number) => void;
  readonly detectedstep_name: (a: number, b: number) => void;
  readonly detectedstep_poses: (a: number, b: number) => void;
  readonly detectedstep_bpm: (a: number) => number;
  readonly __wbg_posewrapper_free: (a: number, b: number) => void;
  readonly posewrapper_skeleton: (a: number) => number;
  readonly posewrapper_sideSkeleton: (a: number) => number;
  readonly posewrapper_id: (a: number, b: number) => void;
  readonly posewrapper_name: (a: number, b: number, c: number, d: number) => void;
  readonly posewrapper_setName: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly posewrapper_setAngle: (a: number, b: number, c: number) => void;
  readonly posewrapper_getAngle: (a: number, b: number) => number;
  readonly posewrapper_setZ: (a: number, b: number, c: number) => void;
  readonly posewrapper_getZ: (a: number, b: number) => number;
  readonly posewrapper_setWeight: (a: number, b: number, c: number) => void;
  readonly posewrapper_getWeight: (a: number, b: number) => number;
  readonly posewrapper_direction: (a: number) => number;
  readonly posewrapper_setDirection: (a: number, b: number) => void;
  readonly posewrapper_xShift: (a: number) => number;
  readonly posewrapper_set_xShift: (a: number, b: number) => void;
  readonly posewrapper_yShift: (a: number) => number;
  readonly posewrapper_set_yShift: (a: number, b: number) => void;
  readonly posewrapper_turnShoulder: (a: number) => number;
  readonly posewrapper_set_turnShoulder: (a: number, b: number) => void;
  readonly posewrapper_turnHip: (a: number) => number;
  readonly posewrapper_set_turnHip: (a: number, b: number) => void;
  readonly __wbg_skeletonv2_free: (a: number, b: number) => void;
  readonly __wbg_get_skeletonv2_left: (a: number) => number;
  readonly __wbg_set_skeletonv2_left: (a: number, b: number) => void;
  readonly __wbg_get_skeletonv2_right: (a: number) => number;
  readonly __wbg_set_skeletonv2_right: (a: number, b: number) => void;
  readonly __wbg_get_skeletonv2_hip: (a: number) => number;
  readonly __wbg_set_skeletonv2_hip: (a: number, b: number) => void;
  readonly __wbg_get_skeletonv2_shoulder: (a: number) => number;
  readonly __wbg_set_skeletonv2_shoulder: (a: number, b: number) => void;
  readonly __wbg_get_skeletonv2_sideway: (a: number) => number;
  readonly __wbg_set_skeletonv2_sideway: (a: number, b: number) => void;
  readonly __wbg_get_skeletonv2_backwards: (a: number) => number;
  readonly __wbg_set_skeletonv2_backwards: (a: number, b: number) => void;
  readonly __wbg_skeletonsidev2_free: (a: number, b: number) => void;
  readonly __wbg_get_skeletonsidev2_thigh: (a: number) => number;
  readonly __wbg_set_skeletonsidev2_thigh: (a: number, b: number) => void;
  readonly __wbg_get_skeletonsidev2_shin: (a: number) => number;
  readonly __wbg_set_skeletonsidev2_shin: (a: number, b: number) => void;
  readonly __wbg_get_skeletonsidev2_arm: (a: number) => number;
  readonly __wbg_set_skeletonsidev2_arm: (a: number, b: number) => void;
  readonly __wbg_get_skeletonsidev2_forearm: (a: number) => number;
  readonly __wbg_set_skeletonsidev2_forearm: (a: number, b: number) => void;
  readonly __wbg_get_skeletonsidev2_foot: (a: number) => number;
  readonly __wbg_set_skeletonsidev2_foot: (a: number, b: number) => void;
  readonly __wbg_renderablesegment_free: (a: number, b: number) => void;
  readonly __wbg_get_renderablesegment_start: (a: number) => number;
  readonly __wbg_set_renderablesegment_start: (a: number, b: number) => void;
  readonly __wbg_get_renderablesegment_end: (a: number) => number;
  readonly __wbg_set_renderablesegment_end: (a: number, b: number) => void;
  readonly __wbg_get_renderablesegment_z: (a: number) => number;
  readonly __wbg_set_renderablesegment_z: (a: number, b: number) => void;
  readonly skeletonv2_segment: (a: number, b: number) => number;
  readonly skeleton_render: (a: number, b: number, c: number) => number;
  readonly __wbindgen_export_0: (a: number) => void;
  readonly __wbindgen_export_1: (a: number, b: number, c: number) => void;
  readonly __wbindgen_export_2: (a: number, b: number) => number;
  readonly __wbindgen_export_3: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_4: WebAssembly.Table;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_export_5: (a: number, b: number, c: number) => number;
  readonly __wbindgen_export_6: (a: number, b: number, c: number) => void;
  readonly __wbindgen_export_7: (a: number, b: number, c: number, d: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
