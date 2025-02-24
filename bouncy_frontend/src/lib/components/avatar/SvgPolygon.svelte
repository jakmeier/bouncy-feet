<script>
  import { run } from 'svelte/legacy';

  import { getContext, onDestroy, onMount } from 'svelte';

  
  
  
  /**
   * @typedef {Object} Props
   * @property {any} points
   * @property {{ lineWidth: any; color: any; fill: any; linecap: any; }} style
   * @property {string} id
   */

  /** @type {Props} */
  let { points, style, id } = $props();

  const svg = getContext('svg');
  run(() => {
    svg.setPolygon(id, { points, style });
  });

  onMount(() => svg.setPolygon(id, { points, style }));
  onDestroy(() => svg.removePolygon(id));
</script>
