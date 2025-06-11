<script>
  import {
    PoseApproximation,
    Skeleton,
  } from '$lib/instructor/bouncy_instructor';
  import InstructorAvatar from '../avatar/InstructorAvatar.svelte';
  import { LEFT_RIGHT_COLORING } from '$lib/constants';
  import { base } from '$app/paths';
  import { dev } from '$lib/stores/FeatureSelection';
  import BeatLabel from '../ui/BeatLabel.svelte';

  /**
   * @typedef {Object} Props
   * @property {PoseApproximation} pose
   * @property {Skeleton} skeleton
   * @property {string} [beatLabel]
   * @property {number} [threshold]
   */

  /** @type {Props} */
  let { pose, skeleton, beatLabel = '0', threshold = 0.075 } = $props();
  let passed = $derived(pose.error < threshold);
</script>

<div class="pose" class:failed-pose={pose.error > threshold}>
  <BeatLabel>
    {beatLabel}
  </BeatLabel>

  <div class="skeleton">
    <div class="avatar-container">
      {#if skeleton}
        <InstructorAvatar
          avatarSize={1.0}
          {skeleton}
          instructorStyle={LEFT_RIGHT_COLORING}
        />
      {/if}
    </div>
  </div>

  <div class="result">
    {#if passed}
      <img src="{base}/img/symbols/bf_check.svg" alt="passed" />
      {#if $dev}
        {pose.error.toFixed(2)}
      {/if}
    {:else}
      <img src="{base}/img/symbols/bf_cross.svg" alt="F" />
      {#if $dev}
        {pose.error.toFixed(2)}
      {/if}
    {/if}
  </div>
</div>

<style>
  .pose {
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
  .result {
    width: 1.5rem;
    margin: auto;
  }
  .result img {
    width: 100%;
  }
</style>
