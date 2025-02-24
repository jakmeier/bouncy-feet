<script>
  import { getContext, onDestroy, onMount, untrack } from 'svelte';

  /**
   * @typedef {Object} Props
   * @property {any} points
   * @property {{ lineWidth: any; color: any; fill: any; linecap: any; }} style
   * @property {string} id
   */

  /** @type {Props} */
  let { points, style, id } = $props();

  const svg = getContext('svg');
  $effect(() => {
    svg.setPolygon(id, { points, style });
  });

  onMount(() => svg.setPolygon(id, { points, style }));
  onDestroy(() => svg.removePolygon(id));
</script>
