<script>
  import { quadInOut, quadOut } from 'svelte/easing';
  import { Tween } from 'svelte/motion';

  /**
   * @typedef {Object} Props
   * @property {"left" | "center" | "right" } position
   * @property {import('svelte').Snippet} [children]
   * @property {number} index
   */

  /** @type {Props} */
  let { children, position, index } = $props();
  const animationTime = 400;
  let width = $state(400);

  let z = $state(index);
  let x = new Tween(0, {
    delay: 0,
    duration: animationTime,
    easing: quadOut,
  });
  let y = new Tween(0, {
    delay: 0,
    duration: animationTime / 2,
    easing: quadInOut,
  });
  let size = new Tween(0.3, {
    delay: 0,
    duration: animationTime,
    easing: quadInOut,
  });
  $effect(() => {
    switch (position) {
      case 'left':
        x.set(-80);
        y.set(0);
        size.set(0.3);
        z = 5000 + index;
        break;
      case 'center':
        x.set(0);
        size.set(1);
        z = 10000;
        y.set(-15);
        setTimeout(() => y.set(0), animationTime / 2);
        break;
      case 'right':
        x.set(80);
        y.set(0);
        size.set(0.3);
        z = 5000 - index;
        break;
    }
  });
</script>

<div
  class="box"
  class:left={position === 'left'}
  class:right={position === 'right'}
  bind:clientWidth={width}
  style="transform: translate(calc({x.current}% - {width /
    2}px), {y.current}%) scale({size.current}); z-index: {z}"
>
  {@render children?.()}
</div>

<style>
  .box {
    position: relative;
    top: 0;
    left: 50%;
    width: 100%;
    max-width: 400px;
    overflow: hidden;
    flex: auto 1 0;
  }

  .left,
  .right {
    position: absolute;
  }
</style>
