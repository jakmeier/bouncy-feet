<script>
  import { getContext, onDestroy, onMount } from 'svelte';
  /** @type{Line} */
  export let line;
  /** @type{string} */
  export let id;

  const svg = getContext('svg');

  // (optimization): can I avoid calling update on every line?
  $: svg.setLine(id, line), svg.update();

  onMount(() => svg.setLine(id, line));
  onDestroy(() => svg.removeLine(id));
</script>
