<script>
  import { Tracker } from '$lib/instructor/bouncy_instructor';
  import { getContext, onMount } from 'svelte';
  import BackgroundTask from '../BackgroundTask.svelte';

  export let size = 50;
  export let accentColor = true;
  $: innerSize = size * 0.9;
  $: bigInnerSize = size * 0.95;
  $: padding = (bigInnerSize - innerSize) * 2 + 10;
  $: slotSize = size - 2 * padding;
  $: innerColor = accentColor ? '--theme-accent-light' : '--theme-neutral-gray';

  /** @type {{tracker: Tracker}} */
  const { tracker } = getContext('tracker');
  let duration = tracker.timeBetweenPoses;
  /** @type {Element} */
  let disk;

  const keyframes = [
    {
      offset: 0.0,
      width: 'var(--big-inner-size)',
      height: 'var(--big-inner-size)',
    },
    {
      offset: 0.1,
      width: 'var(--inner-size)',
      height: 'var(--inner-size)',
    },
    {
      offset: 1.0,
      width: 'var(--inner-size)',
      height: 'var(--inner-size)',
    },
  ];

  /** @type {Animation} */
  let animation;

  /** @param {number} duration */
  function replaceAnimation(duration) {
    if (animation) {
      animation.cancel();
    }
    const effect = new KeyframeEffect(disk, keyframes, {
      duration,
      easing: 'cubic-bezier(0.65, 0.05, 0.36, 1)',
      iterations: Infinity,
    });
    const now = performance.now();
    animation = new Animation(effect);
    const start = tracker.nextHalfBeat(now);
    animation.startTime = Number(start);
    animation.play();
  }

  function onFrame() {
    if (duration !== tracker.timeBetweenPoses) {
      duration = tracker.timeBetweenPoses;
      replaceAnimation(duration);
    }
  }

  onMount(() => {
    replaceAnimation(duration);
  });
</script>

<div class="container" style="--outer-size: {size}px;">
  <div class="outer container circle" style="--outer-size: {size}px;">
    <div
      bind:this={disk}
      class="inner circle"
      style="--inner-size: {innerSize}px; --big-inner-size: {bigInnerSize}px; --inner-color: var({innerColor})"
    ></div>
    <div
      class="slot"
      style="width: {slotSize}px; height: {slotSize}; padding: {padding}px"
    >
      <slot />
    </div>
  </div>
</div>

<BackgroundTask {onFrame} />

<style>
  .container {
    position: relative;
    display: grid;
    justify-content: center;
    align-content: center;
    margin: auto;
    height: var(--outer-size);
  }
  .outer {
    height: var(--outer-size);
    width: var(--outer-size);
    background-color: var(--theme-neutral-dark);
  }
  .inner {
    width: var(--inner-size);
    height: var(--inner-size);
    background-color: var(--inner-color);
  }
  .circle {
    border-radius: 50%;
  }
  .slot {
    position: absolute;
    display: grid;
    align-self: center;
  }
</style>
