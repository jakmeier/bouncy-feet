<script>
  import { getContext } from 'svelte';
  import SvgAvatar from '$lib/components/avatar/SvgAvatar.svelte';
  import Svg from '$lib/components/avatar/Svg.svelte';

  /**
   * @typedef {Object} Props
   * @property {any} step
   * @property {any} scrollableWidth
   * @property {any} scrollOffset
   * @property {any} reviewStart
   * @property {any} reviewEnd
   * @property {number} [avatarSize]
   */

  /** @type {Props} */
  let {
    step,
    scrollableWidth,
    scrollOffset,
    reviewStart,
    reviewEnd,
    avatarSize = 60,
  } = $props();

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
  title={step.name}
  style="left: {scrollOffset +
    timeToPixel(Number(step.start) - reviewStart)}px; width: {timeToPixel(
    Number(step.end) - Number(step.start)
  ) + avatarSize}px"
>
  <div
    class="pose-group"
    class:passive={step.name.includes('Idle')}
    class:good={step.error < 0.1}
  >
    {#each step.poses as pose}
      <div
        class="pose"
        style="left: {timeToPixel(
          Number(pose.timestamp) - Number(step.start)
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
  </div>
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
    display: grid;
    grid-template-areas:
      'poses'
      'name';
  }
  .pose-group {
    grid-area: poses;
    width: 100%;
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
    grid-area: name;
  }
  .pose {
    position: absolute;
    height: 60px;
  }
</style>
