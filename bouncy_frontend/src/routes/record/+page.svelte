<script>
  import LiveRecording from '$lib/components/record/LiveRecording.svelte';
  import VideoReview from '$lib/components/review/VideoReview.svelte';
  import { getContext, setContext, tick } from 'svelte';
  import { t } from '$lib/i18n';
  import { Tracker } from '$lib/instructor/bouncy_instructor';
  import DanceStats from '../profile/DanceStats.svelte';
  import Settings from '$lib/components/record/Settings.svelte';
  import AllPoseErrors from '$lib/components/dev/AllPoseErrors.svelte';
  import { dev } from '$app/environment';

  const userCtx = getContext('user');

  /** @type {HTMLVideoElement} */
  let reviewVideoElement;
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
  /** @type {number | undefined} */
  let recordingEnd = undefined;

  /** @type {import("$lib/instructor/bouncy_instructor").DetectedStep[]} */
  let detectedSteps = [];

  let reviewStatsNumSteps = 0;
  let reviewStatsSeconds = 0;

  const tracker = new Tracker();
  setContext('tracker', {
    tracker,
  });

  const camera = {
    startCamera: async () => {},
    stopCamera: async () => {},
    startRecording: async () => {},
    endRecording: async () => {
      return undefined;
    },
  };

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

</script>

<div id="outer">
  {#if !showCamera}
    <h1>{$t('record.title')}</h1>
    <p>{$t('record.description')}</p>
    <p>{$t('record.description1')}</p>
    <div style="margin: 30px 0px">
      <Settings {tracker}></Settings>
    </div>
  {:else if reviewVideoSrc !== undefined && recordingStart !== undefined && recordingEnd !== undefined}
    <VideoReview
      {reviewVideoSrc}
      {detectedSteps}
      {recordingStart}
      {recordingEnd}
    ></VideoReview>
  {:else}
    <LiveRecording
      bind:startCamera={camera.startCamera}
      bind:stopCamera={camera.stopCamera}
      bind:startRecording={camera.startRecording}
      bind:endRecording={camera.endRecording}
      bind:recordingStart
      bind:recordingEnd
    ></LiveRecording>
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

{#if dev && showCamera && recordingStarted && !isModelOn}
  <AllPoseErrors {reviewVideoElement} {recordingStart}></AllPoseErrors>
{/if}

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
