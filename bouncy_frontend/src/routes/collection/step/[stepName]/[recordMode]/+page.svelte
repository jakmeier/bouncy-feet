<script>
  import { run } from 'svelte/legacy';

  import { onMount, tick } from 'svelte';
  import { t } from '$lib/i18n';
  import { Tracker } from '$lib/instructor/bouncy_instructor';
  import { stepsByName } from '$lib/instructor/bouncy_instructor';
  import { page } from '$app/state';
  import LiveRecording from '$lib/components/record/LiveRecording.svelte';
  import VideoReview from '$lib/components/review/VideoReview.svelte';
  import { hideNavigation } from '$lib/stores/UiState.svelte';
  import Popup from '$lib/components/ui/Popup.svelte';
  import SessionReward from '$lib/components/SessionReward.svelte';
  import { bpm, registerTracker, setBpm } from '$lib/stores/Beat';
  import Button from '$lib/components/ui/Button.svelte';
  import { DetectionState } from '$lib/instructor/bouncy_instructor';
  import { getUserContext } from '$lib/stores/context';
  import BackHeader from '$lib/components/ui/header/BackHeader.svelte';
  import LoginRequiredContent from '$lib/components/profile/LoginRequiredContent.svelte';

  const stepName = page.params.stepName;
  const instructorStep = stepsByName(stepName)[0];
  // 'learn' | 'train'
  const mode = page.params.recordMode;
  const isLearnMode = mode === 'learn';
  const isTrainMode = mode === 'train';

  const tracker = Tracker.UniqueStepTracker(instructorStep.id);
  registerTracker(tracker);
  setBpm(120);
  // setHalfSpeed(isLearnMode);
  // setHalfSpeed(true);

  const userCtx = getUserContext();
  const user = $derived(userCtx.user);

  /** @type {undefined | string} */
  let reviewVideoSrc = $state();
  let isModelOn = false;
  let showReview = $state(false);
  let showSummary = $state(false);
  /** @type {import("svelte/store").Writable<boolean>} */
  let showLearnModeHint = $state();
  /** @type {import("svelte/store").Writable<boolean>} */
  let showTrainModeHint = $state();
  /** @type {number | undefined} */
  let recordingStart = $state(undefined);
  /** @type {number | undefined} */
  let recordingEnd = $state(undefined);
  let enableLiveAvatar = isLearnMode;
  let enableInstructorAvatar = true;
  let videoOpacity = isTrainMode ? 0.25 : 0.0;

  /** @type {import("bouncy_instructor").DetectedStep[]} */
  let detectedSteps = $state([]);
  /** @type {DanceSessionResult?} */
  let sessionResult = $state();

  /** @type {() => any}*/
  let startCamera = $state();
  /** @type {() => any}*/
  let startRecording = $state();
  /** @type {() => any}*/
  let stopCamera = $state();
  /** @type {() => any}*/
  let stopLiveRecording = $state();

  async function turnOnRecording() {
    await startCamera();
    await startRecording();
    tracker.clear();
    isModelOn = true;
    recordingStart = undefined;
  }

  /** @param {ApiUser} apiUser */
  async function stopCameraAndRecording(apiUser) {
    isModelOn = false;
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

    sessionResult = apiUser.submitStepTraining(stepName, $bpm, detectedSteps);
    if (sessionResult) {
      setTimeout(() => {
        apiUser.addDanceToStats(sessionResult);
      }, 1000);
    }

    stopLiveRecording();
  }

  /**
   * @param {Blob | undefined} videoBlob
   */
  async function onRecordingStopped(videoBlob) {
    tracker?.finishTracking();
    isModelOn = false;

    if (videoBlob) {
      if (reviewVideoSrc) {
        URL.revokeObjectURL(reviewVideoSrc);
      }
      reviewVideoSrc = URL.createObjectURL(videoBlob);
    } else {
      console.warn('ended recording and did not get video blob', videoBlob);
    }

    // detectionResult = tracker?.lastDetection;
    // detectedSteps = detectionResult?.steps();
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
        if (!$showLearnModeHint && !hintShown && !isModelOn) {
          turnOnRecording();
        }
      });
    }

    if (isTrainMode) {
      showTrainModeHint.set(true);
      showTrainModeHint.subscribe((hintShown) => {
        if (!$showTrainModeHint && !hintShown && !isModelOn) {
          turnOnRecording();
        }
      });
    }
  });
  let trackingState = $derived(tracker ? tracker.detectionState : null);

  // TODO(refactor): Very hacky use of $effect, as well as using unguarded
  // `userCtx.apiUser`.
  $effect(() => {
    if ($trackingState === DetectionState.TrackingDone) {
      stopCameraAndRecording(userCtx.apiUser);
    }
  });
</script>

<!-- TODO: translate danceName -->
{#if $hideNavigation}
  <div class="title">{stepName}</div>
{:else if isLearnMode}
  <BackHeader title="{$t('record.learn-dance-prefix')} {stepName}" />
{:else}
  <BackHeader title="{$t('record.train-dance-prefix')} {stepName}" />
{/if}

<LoginRequiredContent>
  {#snippet guest({ apiUser })}
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
              <Button class="wide" symbol="download" text="record.download" />
            </a>
            <Button
              class="wide"
              on:click={closeReview}
              symbol="arrow_back"
              text="record.back-button"
            />
          </div>
        {:else}
          {$t('record.no-video-for-review')}
          <Button
            class="wide"
            on:click={closeReview}
            symbol="arrow_back"
            text="record.back-button"
          />
        {/if}
      {:else if showSummary}
        <div>
          <SessionReward data={sessionResult} step={instructorStep} {user}
          ></SessionReward>
        </div>
        <div class="buttons">
          <Button
            class="wide"
            on:click={openReview}
            symbol="tv"
            text="record.review-button"
          />
          <Button
            class="wide"
            on:click={reset}
            symbol="videocam"
            text="record.reset-button"
          />
          <Button
            class="wide"
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
          bind:stop={stopLiveRecording}
          bind:recordingStart
          bind:recordingEnd
          onStop={onRecordingStopped}
          {videoOpacity}
          {enableLiveAvatar}
          {enableInstructorAvatar}
        ></LiveRecording>
      {/if}
    </div>
  {/snippet}
</LoginRequiredContent>

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
    grid-template-columns: auto;
    width: 90%;
    gap: 1rem;
  }

  div.title {
    margin: auto;
    height: 25px;
    width: 100%;
    padding: 3px;
    background-color: var(--theme-neutral-light);
    border-radius: 2px;
    text-align: center;
    font-size: var(--font-normal);
    overflow: hidden auto;
  }
</style>
