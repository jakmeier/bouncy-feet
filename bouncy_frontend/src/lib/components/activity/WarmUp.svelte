<script>
  import LightBackground from '$lib/components/ui/sections/LightBackground.svelte';
  import {
    DetectionResult,
    Tracker,
    VideoDef,
  } from '$lib/instructor/bouncy_instructor';
  import LiveActivity from './LiveActivity.svelte';
  import WarmUpPreview from './WarmUpPreview.svelte';
  import { registerTracker } from '$lib/stores/Beat';
  import StandardPage from '../ui/StandardPage.svelte';
  import WarmupReview from './WarmupReview.svelte';
  import { getContext, onDestroy } from 'svelte';

  /**
   * @typedef {Object} Props
   * @property {string[]} stepNames
   * @property {VideoDef} [video]
   * @property {string} description
   * @property {boolean} audioControl
   * @property {function} onDone
   * @property {()=>void} onBack
   */

  /** @type {Props} */
  let { stepNames, video, description, audioControl, onDone, onBack } =
    $props();

  /** @type {UserContextData} */
  let user = getContext('user');

  let previewDone = $state(false);

  /** @type {Tracker} */
  let tracker = Tracker.WarmUp(
    stepNames,
    // TODO: maybe let user decide duration?
    80
  );
  const unregisterTracker = registerTracker(tracker);
  let trackingDone = $state(false);

  /** @type {DetectionResult | undefined} */
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

    const fullId = stepNames.join('+');
    const limitedId = fullId.slice(0, 128);
    const sessionResult = user.submitWarmup(limitedId, detection);
    if (sessionResult) {
      setTimeout(() => {
        user.addDanceToStats(sessionResult);
      }, 3000);
    }
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
    {video}
    trackId={'105bpm_tropical_house'}
    onDone={() => (previewDone = true)}
    {onBack}
  />
{:else if !trackingDone}
  <!-- TODO: warmup should be with video instead of avatar -->
  <LiveActivity onDone={onRecordingStopped}></LiveActivity>
{:else if detection !== undefined}
  <WarmupReview {detection} {onContinue}></WarmupReview>
{:else}
  <StandardPage white><h3>bug: missing detection</h3></StandardPage>
{/if}
