<script>
  // Display what the LiveRecording component tells to show.
  //
  // All the data for display should be provided by properties, so that this
  // components is a pure UI component.
  //
  // One simple rule to keep in mind to preserve this: Don't access tracker from
  // within this component.

  import { base } from '$app/paths';
  import { BLACK_COLORING, LEFT_RIGHT_COLORING_LIGHT } from '$lib/constants';
  import { t } from '$lib/i18n';
  import { timeBetweenMoves } from '$lib/stores/Beat';
  import {
    Cartesian2d,
    LimbError,
    Skeleton,
    TeacherView,
  } from '$lib/instructor/bouncy_instructor';
  import Avatar from '$lib/components/avatar/Avatar.svelte';
  import Camera from '$lib/components/record/Camera.svelte';
  import Canvas from '$lib/components/Canvas.svelte';
  import FullScreenArea from '../ui/FullScreenArea.svelte';
  import InstructorAvatar from '../avatar/InstructorAvatar.svelte';
  import ProgressBar from './ProgressBar.svelte';
  import { PoseHint } from '$lib/instructor/bouncy_instructor_bg';

  /**
   * @typedef {Object} Props
   * @property {Skeleton} [instructorSkeleton]
   * @property {string} effectText
   * @property {Cartesian2d} lastSuccessSkeletonOrigin
   * @property {number} lastDetectedSkeletonSize
   * @property {number} progress
   * @property {Cartesian2d} instructorSkeletonBodyShift
   * @property {boolean} lastPoseWasCorrect
   * @property {import("@mediapipe/tasks-vision").NormalizedLandmark[]} userLandmarks Where on camera the user has been detected.
   * @property {LimbError[]} [markedLimbs]
   * @property {()=>void} [onStop] - Function is called when the recording is stopped by the user with the stop button.
   */

  /** @type {Props} */
  let {
    instructorSkeleton,
    effectText,
    lastSuccessSkeletonOrigin,
    lastDetectedSkeletonSize,
    markedLimbs,
    progress,
    instructorSkeletonBodyShift,
    lastPoseWasCorrect,
    userLandmarks,
    onStop,
  } = $props();

  let animationTime = $derived(Math.min($timeBetweenMoves / 3, 300));

  //   Flags to control the UI
  let enableLiveAvatar = $state(false);
  let enableInstructorAvatar = $state(false);
  let showOverlay = $state(true);
  let beforeRecording = $state(true);
  let recordingOn = $state(false);
  let videoOpacity = $state(0.0);
  let view = $state(TeacherView.Off);
  let poseHint = $state();
  let avatarStyle = $state(selectStyle(PoseHint.DontKnow));

  /** Where should the instructor origin be display. It can be on top of the
   * detection, or it can be fixed on screen, depending on the view. */
  let instructorOrigin = $derived(
    view === TeacherView.InstructorOnly
      ? new Cartesian2d(0.0, 0.0)
      : lastSuccessSkeletonOrigin
  );
  let instructorSkeletonSize = $derived(
    view === TeacherView.InstructorOnly
      ? 2.0
      : Math.min(lastDetectedSkeletonSize, 4.0)
  );

  // bindings to DOM
  /** @type {Camera} */
  let camera;
  /** @type {HTMLVideoElement | undefined } */
  let camVideoElement = $state();
  let outerWidth = $state();
  let cameraOn = $state(false);

  export function isCameraOn() {
    return camera;
  }

  export function cameraVideoElement() {
    return camVideoElement;
  }

  function stop() {
    onStop?.();
  }

  export async function startCamera() {
    await camera.startCamera();
  }
  export async function startRecording() {
    await camera.startRecording();
    recordingOn = true;

    beforeRecording = false;
    showOverlay = false;
  }
  export async function endRecording() {
    camera.stopCamera();
    return await camera.endRecording();
  }
  export function stopCamera() {
    camera.stopCamera();
  }

  /**
   * @param {TeacherView} newView
   */
  export function updateView(newView) {
    view = newView;
    switch (view) {
      case TeacherView.Off:
        videoOpacity = 0.0;
        enableLiveAvatar = false;
        enableInstructorAvatar = false;
        break;

      case TeacherView.UserCameraWithTracking:
        videoOpacity = 1.0;
        enableLiveAvatar = true;
        enableInstructorAvatar = false;
        break;

      case TeacherView.InstructorOnly:
        videoOpacity = 0.0;
        enableLiveAvatar = false;
        enableInstructorAvatar = true;
        break;

      case TeacherView.InstructorAndCamera:
        videoOpacity = 1.0;
        enableLiveAvatar = false;
        enableInstructorAvatar = true;
        break;

      default:
        console.warn('Unexpected TeacherView', view);
    }
  }

  /** @param {PoseHint} newPoseHint  */
  export function updatePoseHint(newPoseHint) {
    poseHint = newPoseHint;
    avatarStyle = selectStyle(poseHint);
  }

  /**
   * @param {PoseHint} inputHint
   */
  function selectStyle(inputHint) {
    switch (inputHint) {
      case PoseHint.LeftRight:
        return LEFT_RIGHT_COLORING_LIGHT;
      case PoseHint.ZOrder:
        return BLACK_COLORING;
      default:
        return BLACK_COLORING;
    }
  }
</script>

<FullScreenArea>
  <div class="camera" bind:clientWidth={outerWidth}>
    <Camera
      bind:opacity={videoOpacity}
      bind:videoElement={camVideoElement}
      bind:cameraOn
      bind:this={camera}
    />

    {#if enableInstructorAvatar && instructorSkeleton && recordingOn}
      <div class="avatar-container">
        <InstructorAvatar
          width={camVideoElement.clientWidth}
          height={camVideoElement.clientHeight}
          avatarSize={instructorSkeletonSize}
          skeleton={instructorSkeleton}
          bodyShift={instructorSkeletonBodyShift}
          origin={instructorOrigin}
          instructorStyle={LEFT_RIGHT_COLORING_LIGHT}
          {lastPoseWasCorrect}
          {animationTime}
        />
      </div>
    {/if}

    {#if enableLiveAvatar && recordingOn}
      <div
        class="avatar-container"
        style="left: {(outerWidth - camVideoElement.clientWidth) / 2}px;"
      >
        <Canvas
          width={camVideoElement.clientWidth}
          height={camVideoElement.clientHeight}
        >
          <Avatar
            landmarks={userLandmarks}
            width={camVideoElement.clientWidth}
            height={camVideoElement.clientHeight}
            style={avatarStyle}
            torsoLineWidth={5}
            {markedLimbs}
          ></Avatar>
        </Canvas>
      </div>
    {/if}

    <div class="ui">
      <!-- TODO: add this dev tooling again (ideally without mirroring it :P) -->
      <!-- {#if true}
        <LiveRecordingSettings
          bind:enableLiveAvatar
          bind:enableInstructorAvatar
          bind:videoOpacity
        />
      {/if} -->
      {#if recordingOn}
        <ProgressBar {progress}></ProgressBar>
        <button class="symbol" onclick={stop}>
          <img src="{base}/img/symbols/bf_stop.svg" alt="stop" />
        </button>
      {/if}
    </div>
  </div>

  {#if showOverlay}
    <div class="overlay">
      <div class="frame">
        <div class="corner-marked2">
          <div class="corner-marked">
            {#if beforeRecording}
              <div class="overlay-text">
                {$t('courses.lesson.exercise-start-description')}
              </div>
            {/if}
            {#if effectText}
              <div class="effect-text">
                {effectText}
              </div>
            {/if}
          </div>
        </div>
      </div>
      {#if beforeRecording}
        <button class="symbol" onclick={startRecording}>
          <img src="{base}/img/symbols/bf_eye.svg" alt="start" />
          <div class="accent">Start recording</div>
        </button>
      {/if}
    </div>
  {/if}
</FullScreenArea>

<style>
  .camera {
    width: 100%;
    height: 100%;
    transform: scaleX(-1);

    position: relative;
    display: grid;
    align-items: center;
    justify-items: center;
  }
  .avatar-container {
    position: absolute;
    width: 100%;
    height: 100%;
    z-index: 0;
  }
  .ui {
    text-align: center;
    width: calc(100% - 2rem);
    position: absolute;
    bottom: 0;
  }
  button.symbol {
    margin: 1rem;
  }
  button img {
    width: 3rem;
  }
  .overlay {
    position: absolute;
    top: 0;
    height: 100dvh;
    width: 100vw;
    color: var(--theme-neutral-white);
    background-color: var(--theme-neutral-dark-transparent);
  }
  .overlay-text {
    height: 50dvh;
    display: grid;
    justify-content: center;
    align-content: center;
  }
  .effect-text {
    height: 50dvh;
    display: grid;
    justify-content: center;
    align-content: center;
    font-size: 16rem;
    color: var(--theme-main);
  }
  .frame {
    margin: 10dvh 2rem;
  }
  .overlay button {
    position: relative;
    bottom: 3rem;
  }
  .accent {
    color: var(--theme-accent);
  }
</style>
