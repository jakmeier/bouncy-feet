<script>
  import { run } from 'svelte/legacy';

  import { getContext, onDestroy, onMount } from 'svelte';
  
  
  /**
   * @typedef {Object} Props
   * @property {any} line
   * @property {any} id
   */

  /** @type {Props} */
  let { line, id } = $props();

  const svg = getContext('svg');

  // (optimization): can I avoid calling update on every line?
  run(() => {
    svg.setLine(id, line), svg.update();
  });

  onMount(() => svg.setLine(id, line));
  onDestroy(() => svg.removeLine(id));
</script>
