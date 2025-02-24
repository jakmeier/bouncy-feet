<script>
  import { getContext, onDestroy, onMount } from 'svelte';

  /**
   * @typedef {Object} Props
   * @property {Path} path
   * @property {any} id
   */

  /** @type {Props} */
  let { path, id } = $props();

  const svg = getContext('svg');

  $effect(() => {
    svg.setPath(id, path);
  });

  onMount(() => svg.setPath(id, path));
  onDestroy(() => svg.removePath(id));
</script>
