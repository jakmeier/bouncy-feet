<script>
  import { run } from 'svelte/legacy';

  import { tweened } from 'svelte/motion';
  import { getContext } from 'svelte';
  import { writable } from 'svelte/store';

  
  /**
   * @typedef {Object} Props
   * @property {Circle} circle
   */

  /** @type {Props} */
  let { circle } = $props();

  const animationCtx = getContext('animation');
  const animation = animationCtx
    ? animationCtx.animation
    : writable({ duration: 0 });

  // use svelte/motion.tweened for smoothly changing x,y values
  const cxStore = tweened(circle.cx, $animation);
  const cyStore = animationCtx
    ? animationCtx.tweenedJump(circle.cy)
    : tweened(circle.cy, $animation);
  const rStore = tweened(circle.r, $animation);

  run(() => {
    circle, cxStore.set(circle.cx, $animation);
  });
  run(() => {
    circle, cyStore.set(circle.cy);
  });
  run(() => {
    circle, rStore.set(circle.r, $animation);
  });
</script>

<circle
  cx={$cxStore}
  cy={$cyStore}
  r={$rStore}
  stroke={circle.stroke}
  stroke-width={circle.strokeWidth}
  fill={circle.fill}
/>
