<script>
  // TODO: Make this UI less ugly, ideally come up with a clever idea how to display it nicely
  import {
    PoseApproximation,
    Tracker,
  } from '$lib/instructor/bouncy_instructor';
  import { getContext } from 'svelte';
  import InstructorAvatar from '../avatar/InstructorAvatar.svelte';
  import { LEFT_RIGHT_COLORING } from '$lib/constants';
  import { base } from '$app/paths';

  /**
   * @typedef {Object} Props
   * @property {PoseApproximation} pose
   * @property {string} [beatLabel]
   * @property {number} [threshold]
   */

  /** @type {Props} */
  let { pose, beatLabel = '0', threshold = 0.05 } = $props();
  let passed = $derived(pose.error < threshold);

  /** @type {number} bound to client width of bar */
  let width = $state(10);

  /** @type {{tracker: Tracker}} */
  let { tracker } = getContext('tracker');
  let instructorSkeleton = tracker.poseSkeleton(pose.id);
</script>

<div class="pose" class:failed-pose={pose.error > threshold}>
  <div class="beat-label">
    {beatLabel}
  </div>

  <div class="skeleton">
    <div class="avatar-container">
      {#if instructorSkeleton}
        <InstructorAvatar
          {width}
          height={width}
          avatarSize={1.0}
          skeleton={instructorSkeleton}
          instructorStyle={LEFT_RIGHT_COLORING}
        />
      {/if}
    </div>
  </div>

  <div class="result">
    {#if passed}
      <img src="{base}/img/symbols/bf_check.svg" alt="passed" />
    {:else}
      <img src="{base}/img/symbols/bf_cross.svg" alt="F" />
    {/if}
  </div>
</div>

<style>
  .pose {
    height: 230px;
    padding: 0 10px;
    margin: 10px;
  }
  .pose:active {
    box-shadow: none;
  }

  .avatar-container {
    position: relative;
    margin: -20px 0;
    width: 100px;
    height: 100%;
  }
  .skeleton {
    height: 150px;
  }
  .beat-label {
    display: grid;
    align-content: center;
    justify-content: center;
    margin: auto;
    padding: 0.5rem;
    min-width: 1rem;
    height: 1rem;
    font-size: var(--font-normal);
    color: var(--theme-neutral-white);
    background-color: var(--theme-neutral-dark);
    border-radius: 2rem;
    width: fit-content;
  }
  .result {
    width: 1.5rem;
    margin: auto;
  }
  .result img {
    width: 100%;
  }
</style>
