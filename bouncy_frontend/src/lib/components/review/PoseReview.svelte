<script>
  // TODO: Make this UI less ugly, ideally come up with a clever idea how to display it nicely
  import {
    PoseApproximation,
    Tracker,
  } from '$lib/instructor/bouncy_instructor';
  import { getContext } from 'svelte';
  import InstructorAvatar from '../avatar/InstructorAvatar.svelte';
  import { LEFT_RIGHT_COLORING_LIGHT } from '$lib/constants';
  import Symbol from '../ui/Symbol.svelte';

  /** @type {PoseApproximation} */
  export let pose;
  export let beatLabel = '0';
  /** @type {number} */
  export let threshold = 0.05;
  $: max = 3 * threshold;

  /** @type {number} */
  let barHeight = 30;
  /** @type {number} bound to client width of bar */
  let width = 10;

  /** @type {{tracker: Tracker}} */
  let { tracker } = getContext('tracker');
  let instructorSkeleton = tracker.poseSkeleton(pose.id);

  $: badWidth = (1.0 - threshold / max) * width;
  $: scoreWidth = ((max - pose.error) / max) * width;
</script>

<div class="pose" class:failed-pose={pose.error > threshold}>
  {#if pose.error <= threshold}
    <Symbol class="white">verified</Symbol>
  {:else}
    <Symbol class="white">release_alert</Symbol>
  {/if}

  <div class="skeleton">
    <div class="avatar-container">
      {#if instructorSkeleton}
        <InstructorAvatar
          {width}
          height={width}
          avatarSize={1.0}
          skeleton={instructorSkeleton}
          instructorStyle={LEFT_RIGHT_COLORING_LIGHT}
        />
      {/if}
    </div>
  </div>

  <div class="stats">
    <div
      class="background"
      bind:clientWidth={width}
      style="--h: {barHeight}px;"
    >
      <div class="score" style="width: {scoreWidth}px;"></div>
      <div class="bad" style="width: {badWidth}px;"></div>
    </div>
  </div>
  <div class="beat-label">
    {beatLabel}
  </div>
</div>

<style>
  .pose {
    height: 230px;
    background-color: var(--theme-neutral-light);
    border-radius: 5px;
    border: solid 2px #33a86d;
    padding: 0 10px;
    margin: 10px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.55);
  }
  .pose:hover {
    box-shadow: 0 4px 8px #33a86d;
  }
  .failed-pose {
    border: solid 2px #eb3b3b;
  }
  .failed-pose:hover {
    box-shadow: 0 4px 8px #eb3b3b;
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
  .background {
    border: solid 1px;
    border-radius: var(--h);
    height: var(--h);
    overflow: hidden;
  }
  .bad,
  .score {
    height: var(--h);
    position: absolute !important;
    top: 0;
    left: 0;
  }
  .bad {
    border-right: solid 3px white;
  }
  .score {
    background-color: var(--theme-main);
  }

  .beat-label {
    margin-top: 10px;
    padding: 5px;
    font-size: var(--font-normal);
    color: var(--theme-neutral-dark);
  }
</style>
