<script>
  import { quadInOut } from 'svelte/easing';
  import { Spring, Tween } from 'svelte/motion';

  /**
   * @typedef {Object} Props
   * @property {"left" | "center" | "right" } position
   */

  /** @type {Props} */
  let { children, position } = $props();
  let x = new Spring(0);
  let size = new Tween(1, {
    delay: 0,
    duration: 500,
    easing: quadInOut,
  });
  $effect(() => {
    switch (position) {
      case 'left':
        x.set(-150);
        size.set(0.3);
        break;
      case 'center':
        x.set(0);
        size.set(1);
        break;
      case 'right':
        x.set(150);
        size.set(0.3);
        break;
    }
  });
</script>

<div
  class="box"
  style="transform: translate({x.current}%, 0) scale({size.current});"
>
  {@render children?.()}
</div>

<style>
  .box {
    position: absolute;
    top: 0;
    left: 0;
  }
</style>
