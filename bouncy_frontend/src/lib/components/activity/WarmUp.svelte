<script>
  import LightBackground from '$lib/components/ui/sections/LightBackground.svelte';
  import {
    DetectionResult,
    StepWrapper,
    Tracker,
  } from '$lib/instructor/bouncy_instructor';
  import { DetectionState } from '$lib/instructor/bouncy_instructor_bg';
  import ActivityReview from './ActivityReview.svelte';
  import LiveActivity from './LiveActivity.svelte';
  import WarmUpPreview from './WarmUpPreview.svelte';
  import { registerTracker } from '$lib/stores/Beat';
  import { onDestroy } from 'svelte';

  /**
   * @typedef {Object} Props
   * @property {StepWrapper} step
   * @property {string} videoUrl
   * @property {string} description
   * @property {boolean} audioControl
   */

  /** @type {Props} */
  let { step, videoUrl: previewVideoUrl, description, audioControl } = $props();

  let previewDone = $state(false);

  /** @type {Tracker | undefined} */
  let tracker = $state(Tracker.UniqueStepTracker(step.id));
  /** @type {import('svelte/store').Readable<DetectionState> | null} */
  let trackingState = $derived(tracker ? tracker.detectionState : null);
  $effect(() => registerTracker(tracker));

  let recordingStart = $state(0);
  let recordingEnd = $state(0);
  let detection = $state();
  let recordedVideoUrl = $state();
  /**
   * @param {DetectionResult} newDetection
   * @param {number} newRecordingStart
   * @param {number} newRecordingEnd
   * @param {string} videoUrl
   */
  function onRecordingStopped(
    newDetection,
    newRecordingStart,
    newRecordingEnd,
    videoUrl
  ) {
    detection = newDetection;
    recordingStart = newRecordingStart;
    recordingEnd = newRecordingEnd;
    recordedVideoUrl = videoUrl;
  }

  function onRestart() {
    previewDone = false;
    URL.revokeObjectURL(recordedVideoUrl);
    recordedVideoUrl = null;
    detection = null;
    tracker?.clear();
  }

  function onBack() {
    window.history.back();
  }

  onDestroy(() => {
    if (recordedVideoUrl) {
      URL.revokeObjectURL(recordedVideoUrl);
    }
  });
</script>

<LightBackground />

{#if !previewDone}
  <WarmUpPreview
    {audioControl}
    {description}
    videoUrl={previewVideoUrl}
    {step}
    onDone={() => (previewDone = true)}
  />
{:else if $trackingState !== DetectionState.TrackingDone}
  <!-- TODO: warmup should be with video instead of avatar -->
  <LiveActivity
    onDone={onRecordingStopped}
    videoOpacity={1.0}
    enableLiveAvatar={false}
    enableInstructorAvatar={true}
  ></LiveActivity>
{:else}
  <ActivityReview
    {detection}
    {recordingStart}
    {recordingEnd}
    {onBack}
    {onRestart}
    videoUrl={recordedVideoUrl}
  ></ActivityReview>
{/if}

<style>
</style>
