<script>
  import { Tween, tweened } from 'svelte/motion';
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
  let animatedCx = new Tween(circle.cx, $animation);
  const animatedCy = animationCtx
    ? animationCtx.tweenedJump(circle.cy)
    : tweened(circle.cy, $animation);
  let animatedR = new Tween(circle.r, $animation);

  $effect(() => {
    animatedCx.set(circle.cx, animation);
    animatedCy.set(circle.cy, animation);
    animatedR.set(circle.r, animation);
  });
</script>

<circle
  cx={animatedCx.current}
  cy={$animatedCy}
  r={animatedR.current}
  stroke={circle.stroke}
  stroke-width={circle.strokeWidth}
  fill={circle.fill}
/>
