<!-- @migration-task Error while migrating Svelte code: Can't migrate code with afterUpdate. Please migrate by hand. -->
<script>
  /**
   * Canvas component
   *
   * Provides the "canvas" context for descendent components with the
   * `addItem(ctx2d)` function to register new canvas items.
   */
  import { afterUpdate, onMount, tick, setContext } from 'svelte';

  export let width = 100;
  export let height = 100;

  /** @type HTMLCanvasElement */
  let canvas;
  /** @type CanvasRenderingContext2D */
  let ctx;
  let items = new Set();
  let scheduled = false;

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

    afterUpdate(async () => {
      if (scheduled) return;

      scheduled = true;
      await tick();
      scheduled = false;

      draw();
    });
  }

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
