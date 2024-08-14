<script>
  import { tweened } from 'svelte/motion';
  import { getContext } from 'svelte';
  import { writable } from 'svelte/store';

  /** @type{Circle} */
  export let circle;

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

  $: circle, cxStore.set(circle.cx, $animation);
  $: circle, cyStore.set(circle.cy);
  $: circle, rStore.set(circle.r, $animation);
</script>

<circle cx={$cxStore} cy={$cyStore} r={$rStore} fill={circle.fill} />
