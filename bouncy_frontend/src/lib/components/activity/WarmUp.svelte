<script>
  import LightBackground from '$lib/components/ui/sections/LightBackground.svelte';
  import {
    DetectionResult,
    StepWrapper,
    Tracker,
  } from '$lib/instructor/bouncy_instructor';
  import LiveActivity from './LiveActivity.svelte';
  import WarmUpPreview from './WarmUpPreview.svelte';
  import { registerTracker } from '$lib/stores/Beat';
  import StandardPage from '../ui/StandardPage.svelte';
  import WarmupReview from './WarmupReview.svelte';

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

  /** @type {Tracker} */
  let tracker = Tracker.WarmUp([step.name], 120);
  $effect(() => registerTracker(tracker));
  let trackingDone = $state(false);

  let detection = $state();
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
    URL.revokeObjectURL(videoUrl);
    trackingDone = true;
  }

  function onContinue() {
    // todo
  }
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
{:else if !trackingDone}
  <!-- TODO: warmup should be with video instead of avatar -->
  <LiveActivity
    onDone={onRecordingStopped}
    videoOpacity={1.0}
    enableLiveAvatar={false}
    enableInstructorAvatar={true}
  ></LiveActivity>
{:else if detection !== undefined}
  <WarmupReview {detection} {onContinue}></WarmupReview>
{:else}
  <StandardPage white><h3>bug: missing detection</h3></StandardPage>
{/if}
