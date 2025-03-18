<script>
  import LightBackground from '$lib/components/ui/sections/LightBackground.svelte';
  import { DetectionResult, Tracker } from '$lib/instructor/bouncy_instructor';
  import LiveActivity from './LiveActivity.svelte';
  import WarmUpPreview from './WarmUpPreview.svelte';
  import { registerTracker } from '$lib/stores/Beat';
  import StandardPage from '../ui/StandardPage.svelte';
  import WarmupReview from './WarmupReview.svelte';
  import { onDestroy } from 'svelte';

  /**
   * @typedef {Object} Props
   * @property {string[]} stepNames
   * @property {string} videoUrl
   * @property {string} description
   * @property {boolean} audioControl
   * @property {function} onDone
   */

  /** @type {Props} */
  let {
    stepNames,
    videoUrl: previewVideoUrl,
    description,
    audioControl,
    onDone,
  } = $props();

  let previewDone = $state(false);

  /** @type {Tracker} */
  let tracker = Tracker.WarmUp(
    stepNames,
    // TODO: maybe let user decide duration?
    80
  );
  const unregisterTracker = registerTracker(tracker);
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
    onDone();
  }

  onDestroy(unregisterTracker);
</script>

<LightBackground />

{#if !previewDone}
  <WarmUpPreview
    {audioControl}
    {description}
    videoUrl={previewVideoUrl}
    trackId={'105bpm_tropical_house'}
    onDone={() => (previewDone = true)}
  />
{:else if !trackingDone}
  <!-- TODO: warmup should be with video instead of avatar -->
  <LiveActivity onDone={onRecordingStopped}></LiveActivity>
{:else if detection !== undefined}
  <WarmupReview {detection} {onContinue}></WarmupReview>
{:else}
  <StandardPage white><h3>bug: missing detection</h3></StandardPage>
{/if}
