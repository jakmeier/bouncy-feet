<script>
  import LiveRecording from '$lib/components/record/LiveRecording.svelte';
  import VideoReview from '$lib/components/review/VideoReview.svelte';
  import { getContext, setContext, tick } from 'svelte';
  import { t } from '$lib/i18n';
  import { Tracker } from 'bouncy_instructor';
  import DanceStats from '../profile/DanceStats.svelte';
  import Settings from '$lib/components/record/Settings.svelte';
  import AllPoseErrors from '$lib/components/dev/AllPoseErrors.svelte';
  import { dev } from '$lib/stores/FeatureSelection.js';
  import { registerTracker } from '$lib/stores/Beat';
  import Button from '$lib/components/ui/Button.svelte';
  import LightBackground from '$lib/components/ui/sections/LightBackground.svelte';

  const userCtx = getContext('user');

  /** @type {HTMLVideoElement} */
  let reviewVideoElement;
  /** @type {undefined | import("bouncy_instructor").Skeleton} */
  /** @type {undefined | string} */
  let reviewVideoSrc = $state();
  /** @type {import("bouncy_instructor").Skeleton | undefined} */
  let skeleton;
  let isModelOn = $state(false);
  let cameraOn = false;
  let showCamera = $state(false);
  let recordingStarted = $state(false);
  /** @type {number | undefined} */
  let recordingStart = $state(undefined);
  /** @type {number | undefined} */
  let recordingEnd = $state(undefined);

  /** @type {import("bouncy_instructor").DetectedStep[]} */
  let detectedSteps = $state([]);

  let reviewStatsNumSteps = $state(0);
  let reviewStatsSeconds = $state(0);

  const tracker = new Tracker();
  registerTracker(tracker);

  const camera = $state({
    startCamera: async () => {},
    stopCamera: async () => {},
    startRecording: async () => {},
    endRecording: async () => {
      return undefined;
    },
  });

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
    detectedSteps = tracker.detectDance().steps();
    const result = userCtx.computeDanceStats(detectedSteps);
    userCtx.addDanceToStats(result);
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

<LightBackground />

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
      bind:stop={camera.endRecording}
      bind:recordingStart
      bind:recordingEnd
    ></LiveRecording>
  {/if}

  <div>
    {#if !showCamera}
      <Button
        on:click={startRecording}
        symbol="videocam"
        text="record.start-button"
      />
    {/if}

    {#if showCamera}
      {#if !recordingStarted}
        <Button
          on:click={startRecording}
          symbol="radio_button_unchecked"
          text="record.record-button"
        />
      {:else if isModelOn}
        <Button
          on:click={stopCameraAndRecording}
          symbol="camera"
          text="record.stop-record"
        />
      {:else}
        <DanceStats
          numSteps={reviewStatsNumSteps}
          seconds={reviewStatsSeconds}
        />
        <Button on:click={reset} symbol="done" text="record.done-button" />

        <a href={reviewVideoSrc} download>
          <Button symbol="download" text="record.download" />
        </a>
      {/if}
    {/if}
  </div>
  <p style="width: 100px; height: 50px;"></p>
</div>

{#if $dev && showCamera && recordingStarted && !isModelOn}
  <AllPoseErrors {reviewVideoElement} {recordingStart}></AllPoseErrors>
{/if}

<style>
  #outer {
    margin: auto;
    display: grid;
    justify-items: center;
  }
</style>
