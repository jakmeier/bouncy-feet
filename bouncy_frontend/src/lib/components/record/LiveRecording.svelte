<script>
  import Camera from '$lib/components/record/Camera.svelte';
  import Canvas from '$lib/components/Canvas.svelte';
  import Avatar from '$lib/components/avatar/Avatar.svelte';
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
    initAudioContext,
  } from '$lib/stores/Audio';
  import InstructorAvatar from '../avatar/InstructorAvatar.svelte';
  import { distance2d } from '$lib/math';
  import { BLACK_COLORING, LEFT_RIGHT_COLORING_LIGHT } from '$lib/constants';
  import { base } from '$app/paths';
  import ProgressBar from './ProgressBar.svelte';
  import { locale, t } from '$lib/i18n';
  import { timeBetweenMoves } from '$lib/stores/Beat';
  import {
    recordDetectionDelay,
    recordTrackSyncDelay,
    recordMediaPipeDelay,
  } from '$lib/stores/System';
  import FullScreenArea from '../ui/FullScreenArea.svelte';
  import MusicControl from './MusicControl.svelte';
  import { TeacherView } from '$lib/instructor/bouncy_instructor';

  export const startCamera = async () => {
    await camera.startCamera();
  };
  export const startRecording = async () => {
    await camera.startRecording();
    recordingOn = true;
    stopped = false;
  };
  export const stopCamera = () => {
    camera.stopCamera();
  };
  let stopped = $state(false);
  export const stop = async () => {
    stopped = true;
    camera.stopCamera();
    const blob = await camera.endRecording();
    onStop(blob, recordingStart, recordingEnd);
  };

  /**
   * @typedef {Object} Props
   * @property {boolean} [cameraOn]
   * @property {any} [onStop] - Function is called when the recording is stopped by the user with the stop
button or by a explicit `stop` call from the parent component.
The recording video blob is passed as as a parameter.
   * @property {boolean} [enableLiveAvatar]
   * @property {boolean} [enableInstructorAvatar]
   * @property {boolean} [forceBeat] - always evaluate the pose on beat and move on to the next pose, even when
it does not match
   * @property {number} [videoOpacity]
   */

  /** @type {Props} */
  let {
    cameraOn = $bindable(false),
    onStop = (/** @type {Blob | undefined} */ _recording) => {},
    enableLiveAvatar = $bindable(false),
    enableInstructorAvatar = false,
    forceBeat = false,
    videoOpacity = $bindable(0.0),
  } = $props();
  let lastPoseWasCorrect = $state(true);
  let recordingOn = $state(false);
  let showTextEffect = $state(false);

  let recordingStart = $state(0);
  let recordingEnd = $state(0);

  const poseCtx = getContext('pose');
  let tracker = getContext('tracker').tracker;
  let detectionState = tracker.detectionState;
  tracker.enforceBeat(forceBeat);
  let progress = $state(0.0);
  let currentBeat = -1;
  let firstPoseIsShown = false;

  let lastAudioHint = performance.now() - 2000;
  let audioHintDelay = 5000;
  let effectText = $state('');
  /** @type {number | undefined} */
  let clearTextTime = undefined;

  /** @type {Camera} */
  let camera = $state();
  /** @type {HTMLVideoElement} */
  let cameraVideoElement = $state();
  /** @type {TeacherView} */
  let teacherView = $state(TeacherView.Off);
  /** @type {import("bouncy_instructor").Skeleton} */
  let instructorSkeleton = $state(tracker.expectedPoseSkeleton().restingPose());
  let instructorSkeletonBodyShift = $state(tracker.expectedPoseBodyShift());

  /** @type {import("@mediapipe/tasks-vision").NormalizedLandmark[]} */
  let landmarks = $state([]);
  /** @type {PoseDetection} */
  let dataListener;

  /** @type {number} */
  let outerWidth = $state();
  /** @type {number} */
  let outerHeight = $state();

  /** @type {import('svelte/store').Writable<number>} */
  let videoSrcWidth = writable(100);
  /** @type {import('svelte/store').Writable<number>} */
  let videoSrcHeight = writable(150);

  let lastDetectedSkeletonSize = $state(1.0);
  /** When the last step was detected, where was the center of the skeleton on camera. */
  let lastSuccessSkeletonOrigin = $state(new Cartesian2d(0.0, 0.0));
  /** Where should the instructor origin be display. It can be on top of the
   * detection, or it can be fixed on screen, depending on the view. */
  let instructorOrigin = $derived(
    teacherView === TeacherView.InstructorOnly
      ? new Cartesian2d(0.0, 0.0)
      : lastSuccessSkeletonOrigin
  );
  let instructorSkeletonSize = $derived(
    teacherView === TeacherView.InstructorOnly
      ? 2.0
      : Math.min(lastDetectedSkeletonSize, 4.0)
  );
  /** @type {LimbError[]} */
  let worstLimbs = $state([]);

  let animationTime = $derived(Math.min($timeBetweenMoves / 3, 300));

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
  let avatarStyle = $state(selectStyle(PoseHint.DontKnow));

  // this is called periodically in a background task
  function onFrame() {
    if (stopped) {
      return;
    }

    teacherView = tracker.currentView(performance.now());
    updateView(teacherView);

    if ($detectionState === DetectionState.TrackingDone) {
      stop();
    }

    if (cameraOn && dataListener && recordingOn) {
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
          let newBeat = tracker.subbeat(future);
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
    if (clearTextTime !== undefined && performance.now() >= clearTextTime) {
      clearTextTime = undefined;
      effectText = '';
      showTextEffect = false;
    }
    for (
      let textEffect = tracker.nextTextEffect(performance.now());
      textEffect !== undefined;
      textEffect = tracker.nextTextEffect(performance.now())
    ) {
      clearTextTime = textEffect.timestamp + textEffect.duration;
      effectText = textEffect.text;
      showTextEffect = true;
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
    await initAudioContext();
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

  /**
   * @param {TeacherView} view
   */
  function updateView(view) {
    if (view === TeacherView.UserCameraWithTracking) {
      videoOpacity = 1.0;
      enableLiveAvatar = true;
      enableInstructorAvatar = false;
    }
    if (view === TeacherView.InstructorOnly) {
      videoOpacity = 0.0;
      enableLiveAvatar = false;
      enableInstructorAvatar = true;
    }
    if (view === TeacherView.Off) {
      videoOpacity = 0.0;
      enableLiveAvatar = false;
      enableInstructorAvatar = false;
    }
  }
</script>

<BackgroundTask {onFrame}></BackgroundTask>
<MusicControl />

<FullScreenArea>
  <div
    class="camera"
    bind:clientWidth={outerWidth}
    bind:clientHeight={outerHeight}
  >
    <Camera
      bind:opacity={videoOpacity}
      bind:videoElement={cameraVideoElement}
      bind:cameraOn
      bind:this={camera}
    />
    {#if enableInstructorAvatar && instructorSkeleton !== null && recordingOn}
      <div class="avatar-container">
        <InstructorAvatar
          width={$videoSrcWidth}
          height={$videoSrcHeight}
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
        style="left: {(outerWidth - $videoSrcWidth) / 2}px;"
      >
        <Canvas width={$videoSrcWidth} height={$videoSrcHeight}>
          <Avatar
            {landmarks}
            width={$videoSrcWidth}
            height={$videoSrcHeight}
            style={avatarStyle}
            torsoLineWidth={5}
            markedLimbs={worstLimbs}
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
  {#if !recordingOn}
    <div class="overlay">
      <!-- <div class="overlay-logo">
        <LogoHeader white></LogoHeader>
      </div> -->
      <div class="frame">
        <div class="corner-marked2">
          <div class="corner-marked">
            <div class="overlay-text">
              {$t('courses.lesson.exercise-start-description')}
            </div>
          </div>
        </div>
      </div>
      <button class="symbol" onclick={startRecording}>
        <img src="{base}/img/symbols/bf_eye.svg" alt="start" />
        <div class="accent">Start recording</div>
      </button>
    </div>
  {/if}
  {#if showTextEffect}
    <div class="overlay">
      <div class="frame">
        <div class="corner-marked2">
          <div class="corner-marked">
            <div class="effect-text">
              {effectText}
            </div>
          </div>
        </div>
      </div>
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
