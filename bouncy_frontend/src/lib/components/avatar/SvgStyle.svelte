<script>
  import { run } from 'svelte/legacy';

  import { setContext } from 'svelte';
  import { writable } from 'svelte/store';

  /**
   * @typedef {Object} Props
   * @property {string} [color]
   * @property {number} [lineWidth]
   * @property {string} [linecap]
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let {
    color = 'black',
    lineWidth = 1,
    linecap = 'round',
    children
  } = $props();

  const ctx = {
    color,
    lineWidth,
    linecap,
  };
  const reactiveCtx = writable(ctx);
  setContext('svg-style', reactiveCtx);

  run(() => {
    reactiveCtx.set({ color, lineWidth, linecap });
  });
</script>

{@render children?.()}
