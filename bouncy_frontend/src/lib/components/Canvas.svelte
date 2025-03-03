<!-- @migration-task Error while migrating Svelte code: Can't migrate code with afterUpdate. Please migrate by hand. -->
<script>
  /**
   * Canvas component
   *
   * Provides the "canvas" context for descendent components with the
   * `addItem(ctx2d)` function to register new canvas items.
   */
  import { onMount, setContext } from 'svelte';

  /**
   * @typedef {Object} Props
   * @property {number} [width]
   * @property {number} [height]
   */

  /** @type {Props} */
  let { width = 100, height = 100 } = $props();

  /** @type HTMLCanvasElement */
  let canvas;
  /** @type CanvasRenderingContext2D */
  let ctx;
  let items = new Set();

  onMount(() => {
    ctx = canvas.getContext('2d');
  });

  setContext('canvas', {
    addItem,
  });

  function addItem(fn) {
    onMount(() => {
      items.add(fn);
      return () => items.delete(fn);
    });
  }

  $effect(() => draw());

  function draw() {
    ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
    items.forEach((fn) => {
      ctx.save();
      fn(ctx);
      ctx.restore();
    });
  }
</script>

<canvas
  bind:this={canvas}
  {width}
  {height}
  style="width: {width}px; height: {height}px;"
>
  <slot />
</canvas>
