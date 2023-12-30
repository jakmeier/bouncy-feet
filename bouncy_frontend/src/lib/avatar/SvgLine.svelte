<script>
  import { tweened } from 'svelte/motion';
  import { quadIn, quadOut, quadInOut } from 'svelte/easing';

  /** @type{{ x: number; y: number; }} */
  export let start;
  /** @type{{ x: number; y: number; }} */
  export let end;
  /** @type{number} animationTime in ms */
  export let animationTime;

  // code architecture note: It's a bit weird that SvgLine defines the easing
  // function. I think an animation context would make more sense. But I'm
  // keeping it simple for now. I like quadIn for the avatar step animation,
  // which is the only place this is currently in use.
  const animation = {
    duration: () => animationTime,
    easing: quadIn,
  };
  // use svelte/motion.tweened for smoothly changing x,y values
  const startX = tweened(start.x, animation);
  const startY = tweened(start.y, animation);
  const endX = tweened(end.x, animation);
  const endY = tweened(end.y, animation);

  // listen to prop changes and then update tweens
  function updatePosition() {
    $startX = start.x;
    $startY = start.y;
    $endX = end.x;
    $endY = end.y;
  }
  $: start, end, updatePosition();
</script>

<line x1={$startX} y1={$startY} x2={$endX} y2={$endY}> </line>
