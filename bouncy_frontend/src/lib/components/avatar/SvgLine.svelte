<script>
  import { getContext } from 'svelte';
  /** @type{Line} */
  export let line;
  /** @type{string} */
  export let id;

  const svg = getContext('svg');

  // (optimization): can I avoid calling update on every line?
  $: svg.setLine(id, line), svg.update();
</script>
