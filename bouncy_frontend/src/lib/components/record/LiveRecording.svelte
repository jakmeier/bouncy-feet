<script>
  import Area from '$lib/components/ui/Area.svelte';
  import Camera from '$lib/components/record/Camera.svelte';
  import Canvas from '$lib/components/Canvas.svelte';
  import Avatar from '$lib/components/avatar/Avatar.svelte';
  import { hideNavigation, wideView } from '$lib/stores/UiState';
  import { getContext, onMount } from 'svelte';
  import { I, landmarksToKeypoints } from '$lib/pose';
  import BackgroundTask from '../BackgroundTask.svelte';
  import { writable } from 'svelte/store';
  import {
    Cartesian2d,
    DetectionResult,
    LimbError,
    PoseHint,
  } from '$lib/instructor/bouncy_instructor';
  import { playSuccessSound } from '$lib/stores/SoundEffects';
  import InstructorAvatar from '../avatar/InstructorAvatar.svelte';
  import { distance2d } from '$lib/math';
  import { BLACK_COLORING, LEFT_RIGHT_COLORING_LIGHT } from '$lib/constants';

  export let cameraOn = false;
  /** @type {undefined | number} */
  export let recordingStart;
  /** @type {undefined | number} */
  export let recordingEnd;
  /** @type {DetectionResult} */
  let detectionResult = new DetectionResult();

  export const startCamera = async () => {
    await camera.startCamera();
  };
  export const startRecording = async () => {
    await camera.startRecording();
  };
  export const stopCamera = () => {
    camera.stopCamera();
  };
  export const endRecording = async () => {
    return await camera.endRecording();
  };

  export let enableLiveAvatar = false;
  export let enableInstructorAvatar = false;
  export let videoOpacity = 0.0;
  export let slowInstructor = false;

  const poseCtx = getContext('pose');
  let tracker = getContext('tracker').tracker;
  $: $hideNavigation = cameraOn;
  $: $wideView = cameraOn;

  /** @type {Camera} */
  let camera;
  /** @type {HTMLVideoElement} */
  let cameraVideoElement;
  /** @type {import("$lib/instructor/bouncy_instructor").Skeleton} */
  let instructorSkeleton = tracker.expectedPoseSkeleton();
  let instructorSkeletonBodyShift = tracker.expectedPoseBodyShift();

  /** @type {import("@mediapipe/tasks-vision").NormalizedLandmark[]} */
  let landmarks = [];
  /** @type {{ trackFrame: (arg0: HTMLVideoElement) => void; }} */
  let dataListener;

  /** @type {number} */
  const borderWidth = 2;
  /** @type {number} */
  let outerWidth;
  /** @type {number} */
  $: width = outerWidth - 2 * borderWidth;
  /** @type {number} */
  $: height = (width * 4) / 3;
  /** @type {import('svelte/store').Writable<number>} */
  let videoSrcWidth = writable(100);
  /** @type {import('svelte/store').Writable<number>} */
  let videoSrcHeight = writable(150);

  let lastSuccessSkeletonSize = 1.0;
  let lastSuccessSkeletonOrigin = new Cartesian2d(0.0, 0.0);
  /** @type {LimbError[]} */
  let worstLimbs = [];

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
  let avatarStyle = selectStyle(PoseHint.DontKnow);

  function onFrame() {
    if (cameraOn && dataListener) {
      const start = performance.now();
      dataListener.trackFrame(cameraVideoElement);
      const t = performance.now() - start;
      if (t > 50) {
        console.debug(`trackFrame took ${t}ms`);
      }
      const before = tracker.numDetectedPoses();
      detectionResult = tracker.detectNextPose();

      // gather data for visual hints for the error
      const poseHint = tracker.poseHint();
      avatarStyle = selectStyle(poseHint);
      const poseError = tracker.currentPoseError();
      if (poseError) {
        worstLimbs = poseError
          .worstLimbs(3)
          .filter(
            (/** @type {LimbError} */ limb) => limb.error * limb.weight > 0.2
          );
      }

      // correct skeleton tracking
      if (tracker.numDetectedPoses() > before) {
        playSuccessSound();
        instructorSkeleton = tracker.expectedPoseSkeleton();
        instructorSkeletonBodyShift = tracker.expectedPoseBodyShift();
        console.assert(
          instructorSkeleton,
          'tracker returned no next expected pose'
        );
        lastSuccessSkeletonSize =
          distance2d(landmarks[I.LEFT_SHOULDER], landmarks[I.LEFT_HIP]) * 6;
        const hip = tracker.hipPosition(recordingEnd);
        lastSuccessSkeletonOrigin = new Cartesian2d(hip.x - 0.5, hip.y - 0.5);
      }

      // debug info for slow frames
      const t2 = performance.now() - start;
      if (t2 - t > 30) {
        console.debug(`detectDance took ${t2 - t}ms`);
      }
    }
  }

  // this is called anytime media pipe has a frame with landmarks
  const onPoseDetection = (
    /** @type {{ landmarks: import('@mediapipe/tasks-vision').Landmark[][]; }} */ result,
    /** @type {number} */ timestamp
  ) => {
    if (recordingStart === undefined) {
      recordingStart = timestamp;
    }
    if (result.landmarks && result.landmarks.length >= 1) {
      landmarks = result.landmarks[0];
      const kp = landmarksToKeypoints(result.landmarks[0]);
      tracker.addKeypoints(kp, timestamp);
      recordingEnd = timestamp;
    }
    // TODO(performance): do this less often
    onVideoResized();
  };

  function onVideoResized() {
    $videoSrcWidth = cameraVideoElement.clientWidth;
    $videoSrcHeight = cameraVideoElement.clientHeight;
  }

  onMount(async () => {
    dataListener = await poseCtx.newPoseDetection(onPoseDetection);
    onVideoResized();
  });
</script>

<div bind:clientWidth={outerWidth} style="width: 100%; transform: scaleX(-1);">
  <BackgroundTask {onFrame}></BackgroundTask>

  <Area
    width="{width}px"
    height="{height}px"
    borderWidth="{borderWidth}px"
    zIndex={0}
  >
    <Camera
      {width}
      {height}
      bind:opacity={videoOpacity}
      bind:videoElement={cameraVideoElement}
      bind:cameraOn
      bind:this={camera}
    />

    {#if enableInstructorAvatar && instructorSkeleton !== null}
      <div class="avatar-container">
        <InstructorAvatar
          width={$videoSrcWidth}
          height={$videoSrcHeight}
          avatarSize={lastSuccessSkeletonSize}
          skeleton={instructorSkeleton}
          bodyShift={instructorSkeletonBodyShift}
          origin={lastSuccessSkeletonOrigin}
          instructorStyle={LEFT_RIGHT_COLORING_LIGHT}
          {slowInstructor}
        />
      </div>
    {/if}
    {#if enableLiveAvatar}
      <div
        class="avatar-container"
        style="left: {(width - $videoSrcWidth) / 2}px;"
      >
        <Canvas width={$videoSrcWidth} height={$videoSrcHeight}>
          <Avatar
            {landmarks}
            width={$videoSrcWidth}
            height={$videoSrcHeight}
            style={avatarStyle}
            torsoLineWidth={3}
            markedLimbs={worstLimbs}
          ></Avatar>
        </Canvas>
      </div>
    {/if}
  </Area>
</div>

<style>
  .avatar-container {
    position: absolute;
    width: 100%;
    height: 100%;
    z-index: 0;
  }
</style>
