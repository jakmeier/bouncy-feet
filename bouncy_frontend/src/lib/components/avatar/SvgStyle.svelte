<script>
  import { setContext, untrack } from 'svelte';
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
    children,
  } = $props();

  const ctx = {
    color,
    lineWidth,
    linecap,
  };
  const reactiveCtx = writable(ctx);
  setContext('svg-style', reactiveCtx);

  $effect(() => {
    untrack(() => reactiveCtx.set({ color, lineWidth, linecap }));
  });
</script>

{@render children?.()}
