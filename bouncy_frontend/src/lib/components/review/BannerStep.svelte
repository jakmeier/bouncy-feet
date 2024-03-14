<script>
  import { getContext } from 'svelte';
  import SvgAvatar from '$lib/components/avatar/SvgAvatar.svelte';
  import Svg from '$lib/components/avatar/Svg.svelte';

  /** @type{import("$lib/instructor/bouncy_instructor").DetectedStep} */
  export let step;

  /** @type{number} */
  export let scrollableWidth;
  /** @type{number} */
  export let scrollOffset;
  /** @type{number} */
  export let reviewStart;
  /** @type{number} */
  export let reviewEnd;
  /** @type{number} */
  export let avatarSize = 60;

  const trackerCtx = getContext('tracker');

  /**
   * @param {number} delta
   */
  function timeToPixel(delta) {
    return (delta / (reviewEnd - reviewStart)) * scrollableWidth;
  }
</script>

<div
  class="step"
  class:passive={step.name.includes('Idle')}
  class:good={step.error < 0.1}
  title={step.name}
  style="left: {scrollOffset +
    timeToPixel(step.start - reviewStart)}px; width: {timeToPixel(
    step.end - step.start
  ) + avatarSize}px"
>
  {#each step.poses as pose}
    <div
      class="pose"
      style="left: {timeToPixel(
        pose.timestamp - step.start
      )}px; width: {avatarSize}px; height: {avatarSize}px"
    >
      <Svg height={avatarSize} width={avatarSize}>
        <SvgAvatar
          width={avatarSize}
          height={avatarSize}
          lineWidth={2}
          skeleton={trackerCtx.tracker.skeletonAt(pose.timestamp)}
        />
      </Svg>
    </div>
  {/each}
  {#if !step.name.includes('Idle')}
    <div class="step-name">
      <!-- TODO: translation -->
      {step.name} ({((10 * (0.2 - step.error)) / 0.2).toFixed(0)})
    </div>
  {/if}
</div>

<style>
  .step {
    position: absolute;
    height: 60px;
    border-radius: 10px;
    border: 1px solid var(--theme-main);
  }
  .good {
    border-color: var(--theme-main);
    background-color: var(--theme-neutral-light);
  }
  .passive {
    border-color: var(--theme-neutral-light);
    background-color: var(--theme-neutral-white);
  }

  .step-name {
    position: absolute;
    top: 60px;
  }

  .pose {
    position: absolute;
    height: 60px;
  }
</style>
