<script>
  import Canvas from '$lib/Canvas.svelte';
  import { getContext } from 'svelte';
  import Avatar from './Avatar.svelte';

  /** @type{import("$lib/instructor/bouncy_instructor").DetectedStep} */
  export let step;

  /** @type{number} */
  export let totalWidth;
  /** @type{number} */
  export let reviewStart;
  /** @type{number} */
  export let reviewEnd;
  /** @type{number} */
  export let avatarSize = 60;

  const trackerCtx = getContext('tracker');

  /**
   * @param {number} t
   */
  function timeToPosition(t) {
    return timeToPixel(t - reviewStart);
  }

  /**
   * @param {number} delta
   */
  function timeToPixel(delta) {
    return (delta / (reviewEnd - reviewStart)) * totalWidth;
  }
</script>

<div
  class="step"
  style="left: {timeToPosition(step.start)}px; width: {timeToPixel(
    step.end - step.start
  ) + avatarSize}px"
>
  {#each step.poses as pose}
    <div
      class="pose"
      style="left: {timeToPixel(pose.timestamp - step.start)}px"
      title={pose.name}
    >
      <Canvas width={avatarSize} height={avatarSize}>
        <Avatar
          width={avatarSize}
          height={avatarSize}
          lineWidth={2}
          skeleton={trackerCtx.tracker.skeletonAt(pose.timestamp)}
        />
      </Canvas>
    </div>
  {/each}
</div>

<style>
  .step {
    position: absolute;
    height: 60px;
    background-color: var(--theme-neutral-light);
    border-radius: 10px;
    border: 1px solid var(--theme-main);
  }

  .pose {
    position: absolute;
    height: 60px;
  }
</style>
