<script>
  import { getContext, onDestroy, onMount } from 'svelte';

  /**
   * @typedef {Object} Props
   * @property {any} points
   * @property {Style} style
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
