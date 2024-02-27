<script>
  import { getContext } from 'svelte';
  import PoseError from './PoseError.svelte';

  /** @type {HTMLVideoElement} */
  export let reviewVideoElement;

  /** @type {number | undefined} */
  export let recordingStart;

  /**
   * @type {import("$lib/instructor/bouncy_instructor").PoseApproximation[]}
   */
  let poseErrors = [];

  function computePoseErrors() {
    const ms = reviewVideoElement.currentTime * 1000;
    const reviewTimestamp = ms + recordingStart;
    poseErrors = tracker.allPoseErrors(reviewTimestamp);
    console.log('pose errors are', poseErrors);
  }

  const tracker = getContext('tracker').tracker;
</script>

<div class="dev-area">
    <h2>Dev Area</h2>
    <button on:click={computePoseErrors}>
        <span class="material-symbols-outlined"> unfold_more_double </span>
        <p>Print Frame Details</p>
    </button>
    
    {#if poseErrors}
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