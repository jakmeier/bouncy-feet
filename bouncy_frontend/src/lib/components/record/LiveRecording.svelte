<script>
  import Area from '$lib/components/ui/Area.svelte';
  import Camera from '$lib/components/record/Camera.svelte';
  import Canvas from '$lib/components/Canvas.svelte';
  import Avatar from '$lib/components/avatar/Avatar.svelte';
  import { wideView } from '$lib/stores/UiState';
  import { getContext, onMount } from 'svelte';
  import { I, landmarksToKeypoints, PoseDetection } from '$lib/pose';
  import BackgroundTask from '../BackgroundTask.svelte';
  import { writable } from 'svelte/store';
  import {
    Cartesian2d,
    DetectionResult,
    DetectionState,
    LimbError,
    PoseHint,
  } from '$lib/instructor/bouncy_instructor';
  import {
    loadSuccessSound,
    loadAudio,
    scheduleAudio,
    scheduleAudioOnChannel,
    setChannelGain,
  } from '$lib/stores/Audio';
  import InstructorAvatar from '../avatar/InstructorAvatar.svelte';
  import { distance2d } from '$lib/math';
  import { BLACK_COLORING, LEFT_RIGHT_COLORING_LIGHT } from '$lib/constants';
  import { base } from '$app/paths';
  import ProgressBar from './ProgressBar.svelte';
  import { locale } from '$lib/i18n';
  import { timeBetweenMoves } from '$lib/stores/Beat';
  import {
    recordDetectionDelay,
    recordTrackSyncDelay,
    recordMediaPipeDelay,
  } from '$lib/stores/System';

  /** @type {boolean} */
  export let cameraOn = false;
  /** @type {undefined | number} */
  export let recordingStart;
  /** @type {undefined | number} */
  export let recordingEnd;

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
  /** always evaluate the pose on beat and move on to the next pose, even when
   * it does not match */
  export let forceBeat = false;
  export let videoOpacity = 0.0;
  let lastPoseWasCorrect = true;

  const poseCtx = getContext('pose');
  let tracker = getContext('tracker').tracker;
  let detectionState = tracker.detectionState;
  tracker.enforceBeat(forceBeat);
  $: animationTime = Math.min($timeBetweenMoves / 3, 300);
  // $: $hideNavigation = cameraOn;
  $: $wideView = cameraOn;
  let progress = 0.0;
  let currentBeat = -1;
  let firstPoseIsShown = false;

  let lastAudioHint = performance.now() - 2000;
  let audioHintDelay = 5000;

  /** @type {Camera} */
  let camera;
  /** @type {HTMLVideoElement} */
  let cameraVideoElement;
  /** @type {import("$lib/instructor/bouncy_instructor").Skeleton} */
  let instructorSkeleton = tracker.expectedPoseSkeleton().restingPose();
  let instructorSkeletonBodyShift = tracker.expectedPoseBodyShift();

  /** @type {import("@mediapipe/tasks-vision").NormalizedLandmark[]} */
  let landmarks = [];
  /** @type {PoseDetection} */
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

  let lastDetectedSkeletonSize = 1.0;
  $: instructorSkeletonSize = Math.min(lastDetectedSkeletonSize, 2.0);
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

  // this is called periodically in a background task
  function onFrame() {
    if (cameraOn && dataListener) {
      const start = performance.now();
      dataListener.trackFrame(cameraVideoElement);
      const t = performance.now() - start;
      if (t > 50) console.debug(`trackFrame took ${t}ms`);
      recordTrackSyncDelay(t);

      const before = tracker.numDetectedPoses();
      let detectionResult = tracker.runDetection();
      if (tracker.numDetectedPoses() > before) {
        onStepDetection(detectionResult);
        if (!forceBeat) {
          updateInstructor();
        }
        console.assert(
          instructorSkeleton,
          'tracker returned no next expected pose'
        );
      }
      if (
        $detectionState === DetectionState.LiveTracking ||
        $detectionState === DetectionState.InstructorDemo
      ) {
        if (forceBeat) {
          const future = performance.now() + animationTime;
          let newBeat = tracker.beat(future);
          if (newBeat !== currentBeat) {
            instructorSkeleton = tracker.poseSkeletonAtSubbeat(newBeat);
            instructorSkeletonBodyShift =
              tracker.poseBodyShiftAtSubbeat(newBeat);
            currentBeat = newBeat;
          }
        } else if (before === 0 && !firstPoseIsShown) {
          updateInstructor();
          firstPoseIsShown = true;
        }
      }
      displayPoseHint();

      const t2 = performance.now() - start;
      if (t2 - t > 30) {
        console.debug(`detectDance took ${t2 - t}ms`);
      }
      recordDetectionDelay(t2 - t);
    }
    for (
      let audio = tracker.nextAudioEffect();
      audio !== undefined;
      audio = tracker.nextAudioEffect()
    ) {
      scheduleAudio(audio.soundId, Number(audio.timestamp));
    }
  }

  function updateInstructor() {
    instructorSkeleton = tracker.expectedPoseSkeleton();
    instructorSkeletonBodyShift = tracker.expectedPoseBodyShift();
  }

  // this is called anytime media pipe has a frame with landmarks
  const onPoseDetection = (
    /** @type {{ landmarks: import('@mediapipe/tasks-vision').Landmark[][]; }} */ result,
    /** @type {number} */ timestamp
  ) => {
    if (recordingStart === undefined) {
      recordingStart = timestamp;
    }
    recordMediaPipeDelay(performance.now() - timestamp);
    if (result.landmarks && result.landmarks.length >= 1) {
      landmarks = result.landmarks[0];
      const kp = landmarksToKeypoints(result.landmarks[0]);
      const fullyVisible = kp.fullyVisible;
      if (fullyVisible || $detectionState !== DetectionState.Positioning) {
        tracker.addKeypoints(kp, timestamp);
        recordingEnd = timestamp;

        if (fullyVisible) {
          lastDetectedSkeletonSize =
            distance2d(landmarks[I.LEFT_SHOULDER], landmarks[I.LEFT_HIP]) * 6;
        }
      } else if (lastAudioHint + audioHintDelay < performance.now()) {
        lastAudioHint = performance.now();
        scheduleAudioOnChannel('take-position', lastAudioHint, 'audio-guide');
      }
    }
    // TODO(performance): do this less often
    onVideoResized();
  };

  /**
   * this is called anytime the tracker adds a pose to the detected step
   * @param {DetectionResult} detectionResult
   */
  function onStepDetection(detectionResult) {
    // The sound is played as fast as possible, even if that means it will be
    // between beats. I tried playing it on the next beat but that makes it
    // confusing about when an error happens.
    let soundTimestamp = 0;
    if (detectionResult.failureReason === undefined) {
      scheduleAudioOnChannel('success', soundTimestamp, 'live-feedback');
      lastPoseWasCorrect = true;
    } else {
      // scheduleAudioOnChannel('mistake', soundTimestamp, 'live-feedback');
      lastPoseWasCorrect = false;
    }
    const hip = tracker.hipPosition(recordingEnd || 0);
    lastSuccessSkeletonOrigin = new Cartesian2d(hip.x - 0.5, hip.y - 0.5);

    const target = tracker.trackedSubbeats;
    progress = Math.min(tracker.numDetectedPoses() / target, 1.0);
  }

  /** Display visual clues on the avatar to show what the correct position is. */
  function displayPoseHint() {
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
  }

  function onVideoResized() {
    $videoSrcWidth = cameraVideoElement.clientWidth;
    $videoSrcHeight = cameraVideoElement.clientHeight;
  }

  onMount(async () => {
    dataListener = await poseCtx.newPoseDetection(onPoseDetection);
    onVideoResized();
    loadSuccessSound();
    const promises = [
      loadAudio('mistake', `${base}/audio/one-shot-kick.mp3`),
      loadAudio('and', `${base}/audio/and_0.mp3`),
      loadAudio('one', `${base}/audio/one.mp3`),
      loadAudio('two', `${base}/audio/two.mp3`),
      loadAudio('three', `${base}/audio/three.mp3`),
      loadAudio('four', `${base}/audio/four.mp3`),
    ];

    if ($locale.startsWith('de')) {
      promises.push(
        loadAudio('take-position', `${base}/audio/de-take-position.mp3`)
      );
    } else {
      promises.push(
        loadAudio('take-position', `${base}/audio/en-take-position.mp3`)
      );
    }
    await Promise.allSettled(promises);
    // Low volume to not be louder than the music or beat.
    setChannelGain('live-feedback', 0.1);
    setChannelGain('audio-guide', 2.0);
  });

  $: updateView($detectionState);

  /**
   * @param {DetectionState} state
   */
  function updateView(state) {
    if (state === DetectionState.LiveTracking) {
      videoOpacity = 1.0;
      enableLiveAvatar = true;
    }
    if (state === DetectionState.InstructorDemo) {
      videoOpacity = 0.0;
      enableLiveAvatar = false;
    }
  }

  // onDestroy(() => {
  //   $hideNavigation = false;
  // });
</script>

<div bind:clientWidth={outerWidth} style="width: 100%; transform: scaleX(-1);">
  <BackgroundTask {onFrame}></BackgroundTask>

  <Area
    width="{width}px"
    height="{height}px"
    borderWidth="{borderWidth}px"
    zIndex={0}
    backgroundColor={"var(--theme-neutral-light)"}
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
          avatarSize={instructorSkeletonSize}
          skeleton={instructorSkeleton}
          bodyShift={instructorSkeletonBodyShift}
          origin={lastSuccessSkeletonOrigin}
          instructorStyle={LEFT_RIGHT_COLORING_LIGHT}
          {lastPoseWasCorrect}
          {animationTime}
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

  <ProgressBar {progress}></ProgressBar>
</div>

<style>
  .avatar-container {
    position: absolute;
    width: 100%;
    height: 100%;
    z-index: 0;
  }
</style>
