<script>
  import { getContext } from 'svelte';
  import PoseError from './PoseError.svelte';
  import Symbol from '../ui/Symbol.svelte';

  /**
   * @typedef {Object} Props
   * @property {HTMLVideoElement} reviewVideoElement
   * @property {number | undefined} recordingStart
   */

  /** @type {Props} */
  let { reviewVideoElement, recordingStart } = $props();

  /**
   * @type {import("bouncy_instructor").PoseApproximation[]}
   */
  let poseErrors = $state([]);
  /**  @type {number} */
  let poseErrorTimestamp = $state(0.0);

  function computePoseErrors() {
    const ms = reviewVideoElement.currentTime * 1000;
    const reviewTimestamp = ms + recordingStart;
    poseErrors = tracker.allPoseErrors(reviewTimestamp);
    poseErrorTimestamp = reviewTimestamp;
  }

  const tracker = getContext('tracker').tracker;
</script>

<div class="dev-area">
  <h2>Dev Area</h2>
  <button onclick={computePoseErrors}>
    <Symbol>unfold_more_double</Symbol>
    <p>Print Frame Details</p>
  </button>

  {#if poseErrors}
    <p>timestamp: {poseErrorTimestamp}</p>
    {#each poseErrors as pose}
      <PoseError data={pose} />
    {/each}
  {/if}
</div>

<style>
  .dev-area {
    text-align: center;
    padding: 10px;
    background-color: var(--theme-neutral-light);
  }
</style>
