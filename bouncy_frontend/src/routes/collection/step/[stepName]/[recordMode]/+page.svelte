<script>
  import { getContext, onMount, setContext, tick } from 'svelte';
  import { t } from '$lib/i18n';
  import { Tracker } from '$lib/instructor/bouncy_instructor';
  import { stepsByName } from '$lib/instructor/bouncy_instructor';
  import { page } from '$app/stores';
  import LiveRecording from '$lib/components/record/LiveRecording.svelte';
  import VideoReview from '$lib/components/review/VideoReview.svelte';
  import Header from '$lib/components/ui/Header.svelte';
  import { hideNavigation } from '$lib/stores/UiState';
  import LiveRecordingSettings from '$lib/components/record/LiveRecordingSettings.svelte';
  import Popup from '$lib/components/ui/Popup.svelte';
  import SessionReward from '$lib/components/SessionReward.svelte';
  import { registerTracker, setBpm, setHalfSpeed } from '$lib/stores/Beat';
  import Button from '$lib/components/ui/Button.svelte';

  const stepName = $page.params.stepName;
  const instructorStep = stepsByName(stepName)[0];
  // 'learn' | 'train'
  const mode = $page.params.recordMode;
  const isLearnMode = mode === 'learn';
  const isTrainMode = mode === 'train';

  const tracker = Tracker.UniqueStepTracker(instructorStep.id);
  registerTracker(tracker);
  setBpm(120);
  setHalfSpeed(isLearnMode);

  const userCtx = getContext('user');

  /** @type {undefined | string} */
  let reviewVideoSrc;
  let isModelOn = false;
  let showReview = false;
  let showSummary = false;
  /** @type {import("svelte/store").Writable<boolean>} */
  let showLearnModeHint;
  /** @type {import("svelte/store").Writable<boolean>} */
  let showTrainModeHint;
  /** @type {number | undefined} */
  let recordingStart = undefined;
  /** @type {number | undefined} */
  let recordingEnd = undefined;
  let enableLiveAvatar = isLearnMode;
  let enableInstructorAvatar = true;
  let videoOpacity = isTrainMode ? 0.25 : 0.0;

  /** @type {import("$lib/instructor/bouncy_instructor").DetectedStep[]} */
  let detectedSteps = [];
  /** @type {DanceSessionResult?} */
  let sessionResult;

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
    if (isLearnMode) {
      // Reuse all previous detections and show exactly that in the review.
      detectedSteps = tracker.runDetection().steps();
    } else if (isTrainMode) {
      // FIXME: this below doesn't work as expected, needs more testing
      // In train mode, we want to find the best match after the fact, rather
      // than the greedy live-search.
      // detectedSteps = tracker
      // .detectDance()
      // .steps()
      // .filter((step) => {
      //   step.error <= 0.5;
      // });
      detectedSteps = tracker.runDetection().steps();
    }
    showSummary = true;
    sessionResult = userCtx.computeDanceStats(detectedSteps);
    if (sessionResult) {
      setTimeout(() => {
        userCtx.addDanceToStats(sessionResult);
      }, 1000);
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
    showReview = false;
    showSummary = false;
    // wait for LiveRecording component to be mounted again
    await tick();
    await turnOnRecording();
  }

  function openReview() {
    showReview = true;
  }

  function closeReview() {
    showReview = false;
  }

  function goBackToStep() {
    window.history.back();
  }

  onMount(() => {
    if (isLearnMode) {
      showLearnModeHint.set(true);
      showLearnModeHint.subscribe((hintShown) => {
        if (!$showLearnModeHint && isLearnMode && !isModelOn) {
          turnOnRecording();
        }
      });
    }

    if (isTrainMode) {
      showTrainModeHint.set(true);
      showTrainModeHint.subscribe((hintShown) => {
        if (!$showTrainModeHint && isTrainMode && !isModelOn) {
          turnOnRecording();
        }
      });
    }
  });
</script>

<!-- TODO: translate danceName -->
{#if $hideNavigation}
  <div class="title">{stepName}</div>
{:else if isLearnMode}
  <Header title="{$t('record.learn-dance-prefix')} {stepName}" />
{:else}
  <Header title="{$t('record.train-dance-prefix')} {stepName}" />
{/if}

<div id="outer">
  {#if showReview}
    {#if reviewVideoSrc !== undefined && recordingStart !== undefined && recordingEnd !== undefined}
      <VideoReview
        {reviewVideoSrc}
        {detectedSteps}
        {recordingStart}
        {recordingEnd}
      ></VideoReview>
      <div>
        <a href={reviewVideoSrc} download>
          <Button class="light" symbol="download" text="record.download" />
        </a>
        <Button
          class="light"
          on:click={closeReview}
          symbol="arrow_back"
          text="record.back-button"
        />
      </div>
    {:else}
      Could not show review, something failed.
    {/if}
  {:else if showSummary}
    <div>
      <SessionReward data={sessionResult} step={instructorStep}></SessionReward>
    </div>
    <div class="buttons">
      <Button
        class="light"
        on:click={openReview}
        symbol="tv"
        text="record.review-button"
      />
      <Button
        class="light"
        on:click={reset}
        symbol="videocam"
        text="record.reset-button"
      />
      <Button
        class="light"
        on:click={goBackToStep}
        symbol="arrow_back"
        text="record.back-button"
      />
    </div>
  {:else}
    <LiveRecording
      bind:startCamera
      bind:stopCamera
      bind:startRecording
      bind:endRecording
      bind:recordingStart
      bind:recordingEnd
      {videoOpacity}
      {enableLiveAvatar}
      {enableInstructorAvatar}
    ></LiveRecording>
  {/if}

  {#if isModelOn}
    <div>
      <Button
        on:click={stopCameraAndRecording}
        symbol="camera"
        text="record.stop-button"
      />
    </div>
    <LiveRecordingSettings
      bind:enableLiveAvatar
      bind:enableInstructorAvatar
      bind:videoOpacity
    />
  {/if}
  <p style="width: 100px; height: 50px;"></p>
</div>

<Popup
  title="record.learn-hint-title"
  bind:isOpen={showLearnModeHint}
  showOkButton
>
  <div>{$t('record.learn-hint')}</div>
  <div>{$t('record.general-hint')}</div>
  <div>{$t('record.no-upload-hint')}</div>
</Popup>

<Popup
  title="record.train-hint-title"
  bind:isOpen={showTrainModeHint}
  showOkButton
>
  <div>{$t('record.train-hint')}</div>
  <div>{$t('record.general-hint')}</div>
  <div>{$t('record.no-upload-hint')}</div>
</Popup>

<style>
  #outer {
    margin: auto;
    display: grid;
    justify-items: center;
  }

  .buttons {
    display: grid;
    grid-template-columns: auto auto auto;
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
</style>
