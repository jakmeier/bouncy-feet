<script>
  import { run } from 'svelte/legacy';

  import { onMount } from 'svelte';

  /**
   * @typedef {Object} Props
   * @property {number} [size]
   * @property {boolean} [accentColor]
   * @property {number} start
   * @property {number} timeBetweenBeats
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let {
    size = 50,
    accentColor = true,
    start,
    timeBetweenBeats,
    children,
  } = $props();

  /** @type {Element} */
  let disk = $state();

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
    animation = new Animation(effect);
    animation.startTime = Number(start);
    animation.play();
  }

  onMount(() => {
    replaceAnimation(timeBetweenBeats);
  });
  let innerSize = $derived(size * 0.9);
  let bigInnerSize = $derived(size * 0.95);
  let padding = $derived((bigInnerSize - innerSize) * 2 + 10);
  let slotSize = $derived(size - 2 * padding);
  let innerColor = $derived(
    accentColor ? '--theme-main' : '--theme-neutral-gray'
  );
  run(() => {
    timeBetweenBeats, start, replaceAnimation(timeBetweenBeats);
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
      {@render children?.()}
    </div>
  </div>
</div>

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
    text-align: center;
  }
</style>
