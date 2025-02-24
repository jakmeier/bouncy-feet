<script>
  import { getContext, onDestroy, onMount, untrack } from 'svelte';

  /**
   * @typedef {Object} Props
   * @property {any} line
   * @property {any} id
   */

  /** @type {Props} */
  let { line, id } = $props();

  const svg = getContext('svg');

  $effect(() => {
    svg.setLine(id, line);
  });

  onMount(() => svg.setLine(id, line));
  onDestroy(() => svg.removeLine(id));
</script>
