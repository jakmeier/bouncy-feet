<script>
  import { Tracker } from '$lib/instructor/bouncy_instructor';
  import { getContext } from 'svelte';
  import BackgroundTask from '../BackgroundTask.svelte';

  export let size = 50;
  $: innerSize = size * 0.6;
  $: bigInnerSize = innerSize * 1.2;

  /** @type {{tracker: Tracker}} */
  const { tracker } = getContext('tracker');
  let duration = tracker.halfBeatDuration;
  let delay = 0;

  function onFrame() {
    const now = Date.now();
    if (duration !== 2 * tracker.halfBeatDuration) {
      duration = 2 * tracker.halfBeatDuration;
      const start = tracker.nextHalfBeat(BigInt(now));
      delay = Number(start) - now;
    }
  }
</script>

<div class="container" style="--outer-size: {size}px;">
  <div class="outer container circle" style="--outer-size: {size}px;">
    <div
      class="inner circle"
      style="--inner-size: {innerSize}px; --big-inner-size: {bigInnerSize}px; animation-duration: {duration}ms; animation-delay: {delay}ms;"
    ></div>
  </div>
</div>

<BackgroundTask {onFrame} />

<style>
  .container {
    display: grid;
    justify-content: center;
    align-content: center;
    margin: auto;
    height: var(--outer-size);
  }
  .outer {
    height: var(--outer-size);
    width: var(--outer-size);
    background-color: var(--theme-main);
  }
  .inner {
    width: var(--inner-size);
    height: var(--inner-size);
    animation: my_animation;
    animation-iteration-count: infinite;
    animation-timing-function: cubic-bezier(0.65, 0.05, 0.36, 1);
    background-color: var(--theme-neutral-light);
  }
  .circle {
    border-radius: 50%;
  }

  @keyframes my_animation {
    0% {
      width: var(--inner-size);
      height: var(--inner-size);
    }
    50% {
      width: var(--big-inner-size);
      height: var(--big-inner-size);
    }
    100% {
      width: var(--inner-size);
      height: var(--inner-size);
    }
  }
</style>
