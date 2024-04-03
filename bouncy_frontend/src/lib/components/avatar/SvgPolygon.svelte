<script>
  import { getContext, onDestroy, onMount } from 'svelte';

  /** @type{{ x: number; y: number; }[]} */
  export let points;
  /**
   * @type {{ lineWidth: any; color: any; fill: any; linecap: any; }}
   */
  export let style;
  /** @type {string} */
  export let id;

  const svg = getContext('svg');
  $: svg.setPolygon(id, { points, style });

  onMount(() => svg.setPolygon(id, { points, style }));
  onDestroy(() => svg.removePolygon(id));
</script>
