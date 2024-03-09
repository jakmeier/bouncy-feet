<script>
  import { getContext, onDestroy, onMount, setContext, tick } from 'svelte';
  import { t } from '$lib/i18n';
  import { Tracker } from '$lib/instructor/bouncy_instructor';
  import { page } from '$app/stores';
  import LiveRecording from '$lib/components/record/LiveRecording.svelte';
  import DanceStats from '../../../../profile/DanceStats.svelte';
  import VideoReview from '$lib/components/review/VideoReview.svelte';
  import Header from '$lib/components/ui/Header.svelte';

  const danceName = $page.params.slug;
  const tracker = Tracker.StepTracker(danceName);
  setContext('tracker', { tracker });

  const userCtx = getContext('user');

  /** @type {undefined | string} */
  let reviewVideoSrc;
  let isModelOn = false;
  /** @type {number | undefined} */
  let recordingStart = undefined;
  /** @type {number | undefined} */
  let recordingEnd = undefined;

  /** @type {import("$lib/instructor/bouncy_instructor").DetectedStep[]} */
  let detectedSteps = [];

  let reviewStatsNumSteps = 0;
  let reviewStatsSeconds = 0;

  /** @type {() => any}*/
  let startCamera;
  /** @type {() => any}*/
  let startRecording;
  /** @type {() => any}*/
  let stopCamera;
  /** @type {() => any}*/
  let endRecording;

  async function turnOnRecording() {
    await startCamera();
    await startRecording();
    tracker.clear();
    isModelOn = true;
    recordingStart = undefined;
  }

  function stop() {
    stopCamera();
    isModelOn = false;
  }

  async function stopCameraAndRecording() {
    stop();
    detectedSteps = tracker.detectDance();
    const result = userCtx.addDanceToStats(detectedSteps);
    if (result) {
      reviewStatsNumSteps = result.numSteps;
      reviewStatsSeconds = result.duration;
    }
    const videoBlob = await endRecording();

    if (videoBlob) {
      reviewVideoSrc = URL.createObjectURL(videoBlob);
    } else {
      console.warn('ended recording and did not get video blob', videoBlob);
    }
  }

  async function reset() {
    reviewVideoSrc = undefined;
    recordingStart = undefined;
    recordingEnd = undefined;
    // wait for LiveRecording component to be mounted again
    await tick();
    await turnOnRecording();
  }

  onMount(() => {
    turnOnRecording();
  });
</script>

<!-- TODO: translate danceName -->
<Header title="{$t('record.train-dance-prefix')} {danceName}" />

<div id="outer">
  {#if reviewVideoSrc !== undefined && recordingStart !== undefined && recordingEnd !== undefined}
    <VideoReview
      {reviewVideoSrc}
      {detectedSteps}
      {recordingStart}
      {recordingEnd}
    ></VideoReview>
  {:else}
    <LiveRecording
      bind:startCamera
      bind:stopCamera
      bind:startRecording
      bind:endRecording
      bind:recordingStart
      bind:recordingEnd
    ></LiveRecording>
  {/if}

  <div>
    {#if isModelOn}
      <button on:click={stopCameraAndRecording}>
        <span class="material-symbols-outlined"> camera </span>
        <p>{$t('record.stop-record')}</p>
      </button>
    {:else}
      <DanceStats numSteps={reviewStatsNumSteps} seconds={reviewStatsSeconds} />
      <button on:click={reset}>
        <span class="material-symbols-outlined"> done </span>
        <p>{$t('record.done-button')}</p>
      </button>
      <!-- <a href={reviewVideoSrc} download>
        <button>
          <span class="material-symbols-outlined"> download </span>
          <p>{$t('record.download')}</p>
        </button>
      </a> -->
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
