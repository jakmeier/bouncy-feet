<script>
  import { getContext, onDestroy, onMount, untrack } from 'svelte';

  /**
   * @typedef {Object} Props
   * @property {any} path
   * @property {any} id
   */

  /** @type {Props} */
  let { path, id } = $props();

  const svg = getContext('svg');

  $effect(() => {
    svg.setPath(id, path);
    untrack(() => {
      // (optimization): can I avoid calling update on every line or path?
      // svg.update();
    });
  });

  onMount(() => svg.setPath(id, path));
  onDestroy(() => svg.removePath(id));
</script>
