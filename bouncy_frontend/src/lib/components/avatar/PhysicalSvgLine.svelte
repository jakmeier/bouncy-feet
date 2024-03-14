<script>
  import { tweened } from 'svelte/motion';
  import { getContext } from 'svelte';
  import { writable } from 'svelte/store';

  /** @type{Point} */
  export let start;
  /** @type{Point} */
  export let end;
  /** @type{number} animationTime in ms */

  /** @type{Style} */
  export let style;

  const animationCtx = getContext('animation');
  const animation = animationCtx ? animationCtx.animation : writable(0);

  // use svelte/motion.tweened for smoothly changing x,y values
  const startX = tweened(start.x, $animation);
  const startY = tweened(start.y, $animation);
  const endX = tweened(end.x, $animation);
  const endY = tweened(end.y, $animation);

  // listen to prop changes and then update tweens
  function updatePosition() {
    startX.set(start.x, $animation);
    startY.set(start.y, $animation);
    endX.set(end.x, $animation);
    endY.set(end.y, $animation);
  }
  $: start, end, updatePosition();
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
