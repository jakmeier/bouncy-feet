<script>
  import Camera from './Camera.svelte';
  import { landmarksToKeypoints } from '$lib/pose';
  import SvgAvatar from '$lib/avatar/SvgAvatar.svelte';
  import { getContext, onDestroy, onMount, setContext, tick } from 'svelte';
  import Area from './Area.svelte';
  import { t } from '$lib/i18n';
  import { Tracker } from '$lib/instructor/bouncy_instructor';
  import Banner from './Banner.svelte';
  import DanceStats from '../profile/DanceStats.svelte';
  import Settings from './Settings.svelte';

  const poseCtx = getContext('pose');
  const userCtx = getContext('user');

  /** @type {HTMLVideoElement} */
  let cameraVideoElement;
  /** @type {HTMLVideoElement} */
  let reviewVideoElement;
  /** @type {Camera} */
  let camera;
  /** @type {undefined | import("$lib/instructor/bouncy_instructor").Skeleton} */
  /** @type {undefined | string} */
  let reviewVideoSrc;
  /** @type {import("$lib/instructor/bouncy_instructor").Skeleton | undefined} */
  let skeleton;
  let isModelOn = false;
  let cameraOn = false;
  let showCamera = false;
  let recordingStarted = false;
  /** @type {number | undefined} */
  let recordingStart = undefined;
  let recordingEnd = undefined;
  /** @type {{ trackFrame: (arg0: HTMLVideoElement) => void; }} */
  let dataListener;
  let stop = false;
  /** @type {number} */
  let lastSeek = 0;

  /** @type {import("$lib/instructor/bouncy_instructor").DetectedStep[]} */
  let detectedSteps = [];

  let reviewStatsNumSteps = 0;
  let reviewStatsSeconds = 0;

  const tracker = new Tracker();
  setContext('tracker', {
    tracker,
  });

  function loop() {
    if (isModelOn && dataListener) {
      const start = performance.now();
      dataListener.trackFrame(cameraVideoElement);
      const t = performance.now() - start;
      if (t > 50) {
        console.debug(`trackFrame took ${t}ms`);
      }
      detectedSteps = tracker.detectDance();
      const t2 = performance.now() - start;
      if (t2 - t > 30) {
        console.debug(`detectDance took ${t2 - t}ms`);
      }
    }
    const now = new Date().getTime();
    // for now, don't limit time between seeks
    if (recordingStarted && !isModelOn && lastSeek + 0 < now) {
      onSeek();
      lastSeek = now;
    }
    requestAnimationFrame(loop);
  }

  async function startCamera() {
    showCamera = true;
    await tick();
    if (!cameraOn) {
      await camera.startCamera();
      isModelOn = true;
    }
  }

  async function startRecording() {
    await startCamera();
    camera.startRecording();
    tracker.clear();
    recordingStart = undefined;
    recordingStarted = true;
  }

  function stopCamera() {
    camera.stopCamera();
    isModelOn = false;
  }

  async function stopCameraAndRecording() {
    stopCamera();
    detectedSteps = tracker.detectDance();
    const result = userCtx.addDanceToStats(detectedSteps);
    if (result) {
      reviewStatsNumSteps = result.numSteps;
      reviewStatsSeconds = result.duration;
    }
    const videoBlob = await camera.endRecording();

    if (videoBlob) {
      reviewVideoSrc = URL.createObjectURL(videoBlob);
    }
  }

  function reset() {
    recordingStarted = false;
    reviewVideoSrc = undefined;
    skeleton = undefined;
    recordingStart = undefined;
    recordingEnd = undefined;
  }

  async function onSeek() {
    if (recordingStarted && !isModelOn && reviewVideoElement) {
      const ms = reviewVideoElement.currentTime * 1000;
      const reviewTimestamp = ms + recordingStart;
      skeleton = tracker.skeletonAt(reviewTimestamp);
      const cursor =
        (reviewTimestamp - recordingStart) / (recordingEnd - recordingStart);
      setCursor(cursor);
    }
  }

  /**
   * Manually called by child banner. Due to cyclic reactivity, it seems easier
   * than using reactive statements (but maybe I just don't know how to use them
   * properly in such cases)
   * @param {number} cursor
   */
  function seekVideoToCursor(cursor) {
    if (reviewVideoElement && reviewVideoElement.paused) {
      reviewVideoElement.currentTime =
        (cursor * (recordingEnd - recordingStart)) / 1000;
    }
  }
  // the other direction, to be manually called by parent
  /**
   * @type {(cursor: number) => void}
   */
  let setCursor;

  onMount(async () => {
    dataListener = await poseCtx.newPoseDetection((result, timestamp) => {
      if (recordingStart === undefined) {
        recordingStart = timestamp;
      }
      if (result.landmarks && result.landmarks.length >= 1) {
        const kp = landmarksToKeypoints(result.landmarks[0]);
        const skeletons = tracker.addKeypoints(kp, timestamp);
        skeleton = skeletons.front;
        recordingEnd = timestamp;
        if (setCursor) {
          setCursor(1.0);
        }
      }
    });

    if (!stop) {
      loop();
    }
  });

  onDestroy(() => {
    stop = true;
  });
</script>

<div id="outer">
  {#if !showCamera}
    <h1>{$t('record.title')}</h1>
    <p>{$t('record.description')}</p>
    <div style="margin: 30px">
      <Settings {tracker}></Settings>
    </div>
  {:else}
    {#if reviewVideoSrc}
      <!-- svelte-ignore a11y-media-has-caption -->
      <video
        bind:this={reviewVideoElement}
        on:seeked={onSeek}
        src={reviewVideoSrc}
        playsinline
        controls
        style="margin-top: 10px;"
      ></video>
    {:else}
      <Area width="{282}px" height="{376}px">
        <Camera
          width={282}
          height={376}
          bind:videoElement={cameraVideoElement}
          bind:cameraOn
          bind:this={camera}
        />
      </Area>
    {/if}

    <Area width="{280}px" height="{280}px">
      <svg viewBox="0 0 280 280">
        <SvgAvatar width={280} height={280} {skeleton} />
      </svg>
    </Area>
  {/if}

  {#if recordingStarted}
    <Banner
      steps={detectedSteps}
      bind:setCursor
      reviewStart={recordingStart || 0}
      reviewEnd={recordingEnd || 1}
      onScroll={seekVideoToCursor}
    ></Banner>
  {/if}

  <div>
    {#if !showCamera}
      <button on:click={startRecording}>
        <span class="material-symbols-outlined"> videocam </span>
        <p>{$t('record.start-button')}</p>
      </button>
    {/if}

    {#if showCamera}
      {#if !recordingStarted}
        <button on:click={startRecording}>
          <span class="material-symbols-outlined">
            radio_button_unchecked
          </span>
          <p>{$t('record.record-button')}</p>
        </button>
      {:else if isModelOn}
        <button on:click={stopCameraAndRecording}>
          <span class="material-symbols-outlined"> camera </span>
          <p>{$t('record.stop-record')}</p>
        </button>
      {:else}
        <DanceStats
          numSteps={reviewStatsNumSteps}
          seconds={reviewStatsSeconds}
        />
        <button on:click={reset}>
          <span class="material-symbols-outlined"> done </span>
          <p>{$t('record.done-button')}</p>
        </button>
        <a href={reviewVideoSrc} download>
          <button>
            <span class="material-symbols-outlined"> download </span>
            <p>{$t('record.download')}</p>
          </button>
        </a>
      {/if}
    {/if}
  </div>
  <p style="width: 100px; height: 50px;"></p>
</div>

<style>
  #outer {
    margin: auto;
    display: grid;
    justify-items: center;
  }

  button {
    width: 152px;
    height: 80px;
    margin: 10px;
  }
  button span {
    font-size: 42px;
  }

  @media (max-width: 360px) {
    /* Two buttons must fit next to each other, including margin */
    button {
      width: 120px;
    }
  }
</style>
