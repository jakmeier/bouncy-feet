<script>
  import { getContext, onDestroy, onMount } from 'svelte';
  /** @type{Path} */
  export let path;
  /** @type{string} */
  export let id;

  const svg = getContext('svg');

  // (optimization): can I avoid calling update on every line or path?
  $: svg.setPath(id, path), svg.update();

  onMount(() => svg.setPath(id, path));
  onDestroy(() => svg.removePath(id));
</script>
