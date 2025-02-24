<script>
  import { run } from 'svelte/legacy';

  import { tweened } from 'svelte/motion';
  import { getContext } from 'svelte';
  import { writable } from 'svelte/store';

  /** @type{number} animationTime in ms */

  /**
   * @typedef {Object} Props
   * @property {any} start
   * @property {any} end
   * @property {any} style
   */

  /** @type {Props} */
  let { start, end, style } = $props();

  const animationCtx = getContext('animation');
  const animation = animationCtx
    ? animationCtx.animation
    : writable({ duration: 0 });
  const tweenedJump = animationCtx ? animationCtx.tweenedJump : writable;

  // use svelte/motion.tweened for smoothly changing x,y values
  const startX = tweened(start.x, $animation);
  const startY = tweenedJump(start.y);
  const endX = tweened(end.x, $animation);
  const endY = tweenedJump(end.y);
  const animationTimeZero = animationCtx
    ? animationCtx.animationTimeZero
    : null;

  // listen to prop changes and then update tweens
  function updatePosition() {
    startX.set(start.x, $animation);
    startY.set(start.y);
    endX.set(end.x, $animation);
    endY.set(end.y);
    if (animationTimeZero) {
      animationTimeZero.set(performance.now());
    }
  }
  $effect(() => {
    if (start || end) updatePosition();
  });
</script>

<line
  x1={$startX}
  y1={$startY}
  x2={$endX}
  y2={$endY}
  stroke-width="{style.lineWidth}px"
  stroke={style.color}
  stroke-linecap={style.linecap}
>
</line>
