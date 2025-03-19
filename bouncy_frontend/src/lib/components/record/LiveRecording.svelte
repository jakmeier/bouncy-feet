<script>
  import LiveRecordingScreen from './LiveRecordingScreen.svelte';

  // This is the big component at the center of anything related to a live
  // activity. This includes:
  // - Showing an instructor avatar with the moves to make
  // - Starting a camera to show the user how they move
  // - Feeding frames of the camera to the tracker for detection
  // - Switching between different views during the activity
  // - Syncing the music and audio effects to the exercise
  //
  // This makes this file fairly large, beyond what I usually see as a healthy
  // component size. Mostly just too much JS code. Thus, I make this a pure
  // controller class and move all the HTML + CSS in a separate component
  // (LiveREcordingScreen). This separates at least the logic from the display.

  import { getContext, onMount } from 'svelte';
  import { I, landmarksToKeypoints, PoseDetection } from '$lib/pose';
  import BackgroundTask from '../BackgroundTask.svelte';
  import {
    Cartesian2d,
    DanceCursor,
    DetectionResult,
    DetectionState,
    LimbError,
  } from '$lib/instructor/bouncy_instructor';
  import {
    loadSuccessSound,
    loadAudio,
    scheduleAudio,
    scheduleAudioOnChannel,
    setChannelGain,
    initAudioContext,
  } from '$lib/stores/Audio';
  import { distance2d } from '$lib/math';
  import { base } from '$app/paths';
  import { locale } from '$lib/i18n';
  import { timeBetweenMoves } from '$lib/stores/Beat';
  import {
    recordDetectionDelay,
    recordTrackSyncDelay,
    recordMediaPipeDelay,
  } from '$lib/stores/System';
  import MusicControl from './MusicControl.svelte';

  export const startCamera = async () => {
    await screen.startCamera();
  };
  export const startRecording = async () => {
    await screen.startRecording();
    recordingOn = true;
    stopped = false;
  };
  export const stopCamera = () => {
    screen.stopCamera();
  };
  let stopped = $state(true);
  export const stop = async () => {
    stopped = true;
    const blob = await screen.endRecording();
    onStop(blob, recordingStart, recordingEnd);
  };

  /**
   * @typedef {Object} Props
   * @property {boolean} [cameraOn]
   * @property {any} [onStop] - Function is called when the recording is stopped by the user with the stop
button or by a explicit `stop` call from the parent component.
The recording video blob is passed as as a parameter.
   * @property {boolean} [forceBeat] - always evaluate the pose on beat and move on to the next pose, even when
it does not match
   */

  /** @type {Props} */
  let {
    onStop = (/** @type {Blob | undefined} */ _recording) => {},
    forceBeat = false,
  } = $props();

  /** @type {LiveRecordingScreen} */
  let screen;

  let lastPoseWasCorrect = $state(true);
  let recordingOn = $state(false);

  let recordingStart = $state(0);
  let recordingEnd = $state(0);

  const poseCtx = getContext('pose');
  let tracker = getContext('tracker').tracker;
  let detectionState = tracker.detectionState;
  tracker.enforceBeat(forceBeat);
  let progress = $state(0.0);
  /** @type {DanceCursor | null} */
  let tailCursor = null;
  let firstPoseIsShown = false;

  let lastAudioHint = performance.now() - 2000;
  let audioHintDelay = 5000;
  let effectText = $state('');
  /** @type {number | undefined} */
  let clearTextTime = undefined;

  /** @type {import("bouncy_instructor").Skeleton} */
  let instructorSkeleton = $state(tracker.expectedPoseSkeleton().restingPose());
  let instructorSkeletonBodyShift = $state(tracker.expectedPoseBodyShift());
  let instructorJumpHeight = $state(1.0);

  /** @type {import("@mediapipe/tasks-vision").NormalizedLandmark[]} */
  let landmarks = $state([]);
  /** @type {PoseDetection} */
  let dataListener;

  /** When the last step was detected, where was the user detected on camera.
   * last detection -> updated on every detection
   * user skeleton -> updated on pose to not interfere with animations
   */
  let lastDetectedSkeletonSize = $state(1.0);
  let lastSuccessSkeletonOrigin = $state(new Cartesian2d(0.0, 0.0));

  let userSkeletonSize = $state(1.0);
  let userSkeletonOrigin = $state(new Cartesian2d(0.0, 0.0));

  /** @type {LimbError[]} */
  let worstLimbs = $state([]);

  let animationTime = $derived(Math.min($timeBetweenMoves / 3, 300));

  // this is called periodically in a background task
  function onFrame() {
    if (stopped) {
      return;
    }

    const teacherView = tracker.currentView(performance.now());
    screen.updateView(teacherView, $detectionState);

    if ($detectionState === DetectionState.TrackingDone) {
      stop();
    }

    if (
      !recordingOn &&
      $detectionState !== DetectionState.Init &&
      $detectionState !== DetectionState.Positioning
    ) {
      startRecording();
    }

    if (screen.isCameraOn() && dataListener) {
      processNextFrame(dataListener);
    }
    processAudioEffects();
    processTextEffects();
  }

  function processAudioEffects() {
    for (
      let audio = tracker.nextAudioEffect();
      audio !== undefined;
      audio = tracker.nextAudioEffect()
    ) {
      scheduleAudio(audio.soundId, Number(audio.timestamp));
    }
  }

  function processTextEffects() {
    if (clearTextTime !== undefined && performance.now() >= clearTextTime) {
      clearTextTime = undefined;
      effectText = '';
    }
    for (
      let textEffect = tracker.nextTextEffect(performance.now());
      textEffect !== undefined;
      textEffect = tracker.nextTextEffect(performance.now())
    ) {
      clearTextTime = textEffect.timestamp + textEffect.duration;
      effectText = textEffect.text;
    }
  }

  /** @param {PoseDetection} dataListener */
  function processNextFrame(dataListener) {
    const videoElement = screen.cameraVideoElement();
    const videoAvailable = videoElement && videoElement.videoHeight > 0;
    if (!videoAvailable) {
      return;
    }
    const start = performance.now();
    dataListener.trackFrame(videoElement);
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

        /** @type {DanceCursor} */
        let newCursor = tracker.cursor(future);
        if (!tailCursor?.isSamePose(newCursor)) {
          instructorSkeleton = tracker.poseSkeletonAt(newCursor);
          instructorSkeletonBodyShift = tracker.poseBodyShift(newCursor);
          instructorJumpHeight = tracker.jumpHeight(newCursor);
          tailCursor = newCursor;

          updateInstructorPosition();
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

  function updateInstructor() {
    instructorSkeleton = tracker.expectedPoseSkeleton();
    instructorSkeletonBodyShift = tracker.expectedPoseBodyShift();
    instructorJumpHeight = tracker.expectedJumpHeight();
    updateInstructorPosition();
  }

  function updateInstructorPosition() {
    userSkeletonSize = lastDetectedSkeletonSize;
    userSkeletonOrigin = lastSuccessSkeletonOrigin;
  }

  // this is called anytime media pipe has a frame with landmarks
  const onPoseDetection = (
    /** @type {{ landmarks: import('@mediapipe/tasks-vision').Landmark[][]; }} */ result,
    /** @type {number} */ timestamp
  ) => {
    if (recordingStart === 0) {
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
    screen.updatePoseHint(poseHint);
    const poseError = tracker.currentPoseError();
    if (poseError) {
      worstLimbs = poseError
        .worstLimbs(3)
        .filter(
          (/** @type {LimbError} */ limb) => limb.error * limb.weight > 0.2
        );
    }
  }

  onMount(async () => {
    await initAudioContext();
    dataListener = await poseCtx.newPoseDetection(onPoseDetection);
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
    stopped = false;
  });
</script>

<BackgroundTask {onFrame}></BackgroundTask>
<MusicControl />

<LiveRecordingScreen
  bind:this={screen}
  {effectText}
  {instructorJumpHeight}
  {instructorSkeleton}
  {instructorSkeletonBodyShift}
  {lastPoseWasCorrect}
  {progress}
  {userSkeletonOrigin}
  {userSkeletonSize}
  markedLimbs={worstLimbs}
  userLandmarks={landmarks}
  onStop={stop}
></LiveRecordingScreen>
