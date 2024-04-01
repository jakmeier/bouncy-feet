<script>
  import { tweened } from 'svelte/motion';
  import { getContext } from 'svelte';
  import { writable } from 'svelte/store';

  /** @type{number} */
  export let cx;
  /** @type{number} */
  export let cy;
  /** @type{number} */
  export let r;
  /** @type{string} */
  export let fill;

  const animationCtx = getContext('animation');
  const animation = animationCtx
    ? animationCtx.animation
    : writable({ duration: 0 });

  // use svelte/motion.tweened for smoothly changing x,y values
  const cxStore = tweened(cx);
  const cyStore = tweened(cy);
  const rStore = tweened(r);

  $: cx, updateX();
  $: cy, updateY();
  $: r, updateR();

  // somehow this indirection is necessary for reactivity to kick in as desired
  function updateX() {
    cxStore.set(cx, $animation);
  }

  function updateY() {
    cyStore.set(cy, $animation);
  }

  function updateR() {
    rStore.set(r, $animation);
  }
</script>

<circle cx={$cxStore} cy={$cyStore} r={$rStore} {fill} />
