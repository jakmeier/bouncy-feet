<script>
  import { run } from 'svelte/legacy';

  import { getContext, onDestroy, onMount } from 'svelte';
  
  
  /**
   * @typedef {Object} Props
   * @property {any} path
   * @property {any} id
   */

  /** @type {Props} */
  let { path, id } = $props();

  const svg = getContext('svg');

  // (optimization): can I avoid calling update on every line or path?
  run(() => {
    svg.setPath(id, path), svg.update();
  });

  onMount(() => svg.setPath(id, path));
  onDestroy(() => svg.removePath(id));
</script>
