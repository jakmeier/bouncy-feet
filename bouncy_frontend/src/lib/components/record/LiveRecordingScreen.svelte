<script>
  // Display what the LiveRecording component tells to show.
  //
  // All the data for display should be provided by properties, so that this
  // components is a pure UI component.
  //
  // One simple rule to keep in mind to preserve this: Don't access tracker from
  // within this component.

  import { base } from '$app/paths';
  import {
    BLACK_COLORING,
    LEFT_RIGHT_COLORING,
    LEFT_RIGHT_COLORING_LIGHT,
    UGLY_LEFT_RIGHT_COLORING,
  } from '$lib/constants';
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
  import {
    DetectionState,
    PoseHint,
  } from '$lib/instructor/bouncy_instructor_bg';
  import { fade } from 'svelte/transition';

  /**
   * @typedef {Object} Props
   * @property {Skeleton} [instructorSkeleton]
   * @property {string} effectText
   * @property {Cartesian2d} userSkeletonOrigin Where the dancer is on camera. Don't update between poses.
   * @property {number} userSkeletonSize How large the dancer is on camera. Don't update between poses.
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
    userSkeletonOrigin,
    userSkeletonSize,
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
  let showExplanation = $state(true);
  let recordingOn = $state(false);
  let videoOpacity = $state(0.0);
  let view = $state(TeacherView.Off);
  let poseHint = $state();
  let userStyle = $state(selectStyle(PoseHint.DontKnow));

  /** Where should the instructor origin be display. It can be on top of the
   * detection, or it can be fixed on screen, depending on the view. */
  let instructorOrigin = $derived.by(() => {
    if (view === TeacherView.InstructorOnly) {
      return new Cartesian2d(0.0, 0.0);
    } else if (
      userSkeletonOrigin.x > -1 &&
      userSkeletonOrigin.x < 1 &&
      userSkeletonOrigin.y > -1 &&
      userSkeletonOrigin.y < 1
    ) {
      return userSkeletonOrigin;
    } else {
      return new Cartesian2d(0.0, 0.0);
    }
  });
  let instructorSkeletonSize = $derived(
    view === TeacherView.InstructorOnly ? 1.25 : Math.min(userSkeletonSize, 2.5)
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
   * @param {DetectionState} newDetectionState
   */
  export function updateView(newView, newDetectionState) {
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

      case TeacherView.CameraOnly:
        videoOpacity = 1.0;
        enableLiveAvatar = false;
        enableInstructorAvatar = false;
        break;

      default:
        console.warn('Unexpected TeacherView', view);
    }

    showExplanation = [
      DetectionState.Init,
      DetectionState.Positioning,
    ].includes(newDetectionState);
    showOverlay = showExplanation || !!effectText;
  }

  /** @param {PoseHint} newPoseHint  */
  export function updatePoseHint(newPoseHint) {
    poseHint = newPoseHint;
    userStyle = selectStyle(poseHint);
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
        return UGLY_LEFT_RIGHT_COLORING;
    }
  }
</script>

<FullScreenArea backgroundColor="var(--theme-neutral-black)">
  <div class="camera" bind:clientWidth={outerWidth}>
    <Camera
      bind:opacity={videoOpacity}
      bind:videoElement={camVideoElement}
      bind:cameraOn
      bind:this={camera}
    />

    {#if enableInstructorAvatar && instructorSkeleton}
      <div class="instructor">
        <InstructorAvatar
          avatarSize={instructorSkeletonSize}
          skeleton={instructorSkeleton}
          bodyShift={instructorSkeletonBodyShift}
          origin={instructorOrigin}
          instructorStyle={LEFT_RIGHT_COLORING}
          {lastPoseWasCorrect}
          {animationTime}
        />
      </div>
    {/if}

    {#if enableLiveAvatar}
      <div
        class="avatar-container"
        style="left: {(outerWidth - camVideoElement?.clientWidth) / 2}px;"
      >
        <Canvas
          width={camVideoElement?.clientWidth}
          height={camVideoElement?.clientHeight}
        >
          <Avatar
            landmarks={userLandmarks}
            width={camVideoElement?.clientWidth}
            height={camVideoElement?.clientHeight}
            style={userStyle}
            torsoLineWidth={5}
            {markedLimbs}
          ></Avatar>
        </Canvas>
      </div>
    {/if}

    <div class="progress">
      <ProgressBar {progress}></ProgressBar>
    </div>
  </div>

  <div class="overlay" class:transparent={!showOverlay} transition:fade>
    <div class="frame">
      <div class="corner-marked2">
        <div class="corner-marked">
          <div class="framed">
            {#if showExplanation}
              <div class="overlay-text">
                <div>
                  {$t('courses.lesson.exercise-start-description-0')}
                </div>
                <div>
                  {$t('courses.lesson.exercise-start-description-1')}
                </div>
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
    </div>
  </div>

  <div class="ui">
    <!-- TODO: add this dev tooling again (ideally without mirroring it :P) -->
    <!-- {#if true}
      <LiveRecordingSettings
        bind:enableLiveAvatar
        bind:enableInstructorAvatar
        bind:videoOpacity
      />
    {/if} -->
    <button class="symbol" onclick={stop}>
      <img src="{base}/img/symbols/bf_stop.svg" alt="stop" />
    </button>
  </div>
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
  .progress {
    position: absolute;
    top: 1rem;
    width: 90vw;
  }
  .instructor {
    position: absolute;
    top: 4rem;
    width: 100%;
    height: calc(100dvh - 8em);
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
  .overlay.transparent {
    background-color: transparent;
  }
  .effect-text,
  .overlay-text {
    height: 100%;
    display: grid;
    justify-content: center;
    align-content: center;
  }
  .overlay-text div {
    padding: 1rem;
  }
  .effect-text {
    font-size: 16rem;
    color: var(--theme-main);
  }
  .frame {
    margin: 10dvh 2rem;
  }
  .framed {
    height: calc(100dvh - 8em);
  }
</style>
