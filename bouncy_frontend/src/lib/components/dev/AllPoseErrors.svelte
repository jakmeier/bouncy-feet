<script>
  import { getContext } from 'svelte';
  import PoseError from './PoseError.svelte';
  import Symbol from '../ui/Symbol.svelte';

  /** @type {HTMLVideoElement} */
  export let reviewVideoElement;

  /** @type {number | undefined} */
  export let recordingStart;

  /**
   * @type {import("$lib/instructor/bouncy_instructor").PoseApproximation[]}
   */
  let poseErrors = [];
  /**  @type {number} */
  let poseErrorTimestamp = 0.0;

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
  <button on:click={computePoseErrors}>
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
