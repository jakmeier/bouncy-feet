<script>
  import Section from './Section.svelte';
  /**
   * @typedef {Object} Props
   * @property {PageColoring} pageColoring
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let { pageColoring, children } = $props();
</script>

<div style="--dance-floor: {pageColoring.danceFloorColor}">
  <Section bgColor={pageColoring.pageColor} color={pageColoring.fontColor}
    >{@render children?.()}</Section
  >
</div>
