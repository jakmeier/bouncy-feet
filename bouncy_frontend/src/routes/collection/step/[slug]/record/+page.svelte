<script>
  import { getContext, onDestroy, onMount, setContext, tick } from 'svelte';
  import { t } from '$lib/i18n';
  import { Tracker } from '$lib/instructor/bouncy_instructor';
  import { stepsByName } from '$lib/instructor/bouncy_instructor';
  import { page } from '$app/stores';
  import LiveRecording from '$lib/components/record/LiveRecording.svelte';
  import DanceStats from '../../../../profile/DanceStats.svelte';
  import VideoReview from '$lib/components/review/VideoReview.svelte';
  import Header from '$lib/components/ui/Header.svelte';
  import { hideNavigation } from '$lib/stores/UiState';
  import LiveRecordingSettings from '$lib/components/record/LiveRecordingSettings.svelte';

  const stepName = $page.params.slug;
  const instructorStep = stepsByName(stepName)[0];
  const tracker = Tracker.UniqueStepTracker(instructorStep.id);
  setContext('tracker', { tracker });

  const userCtx = getContext('user');

  /** @type {undefined | string} */
  let reviewVideoSrc;
  let isModelOn = false;
  /** @type {number | undefined} */
  let recordingStart = undefined;
  /** @type {number | undefined} */
  let recordingEnd = undefined;
  let enableLiveAvatar = true;
  let enableInstructorAvatar = true;

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
    detectedSteps = tracker.detectNextPose().steps();
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
{#if $hideNavigation}
  <div class="title">{stepName}</div>
{:else}
  <Header title="{$t('record.train-dance-prefix')} {stepName}" />
{/if}

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
      {enableLiveAvatar}
      {enableInstructorAvatar}
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
      <button class="light" on:click={reset}>
        <span class="material-symbols-outlined"> videocam </span>
        <p>{$t('record.start-button')}</p>
      </button>
      <!-- <a href={reviewVideoSrc} download>
        <button>
          <span class="material-symbols-outlined"> download </span>
          <p>{$t('record.download')}</p>
        </button>
      </a> -->
    {/if}
  </div>
  {#if isModelOn}
    <LiveRecordingSettings bind:enableLiveAvatar bind:enableInstructorAvatar />
  {/if}
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

  div.title {
    margin: auto;
    height: 25px;
    width: 100%;
    padding: 3px;
    background-color: var(--theme-neutral-light);
    border-radius: 2px;
    text-align: center;
    font-size: 23px;
    overflow: hidden auto;
  }

  @media (max-width: 360px) {
    /* Two buttons must fit next to each other, including margin */
    button {
      width: 120px;
    }
  }
</style>
