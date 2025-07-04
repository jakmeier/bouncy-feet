<script>
  import Section from './Section.svelte';
  /**
   * @typedef {Object} Props
   * @property {PageColoring} pageColoring
   * @property {boolean} [fillScreen]
   * @property {boolean} [arrow]
   * @property {string} [arrowText]
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let { pageColoring, fillScreen, arrow, arrowText, children } = $props();
</script>

<div style="--dance-floor: {pageColoring.danceFloorColor}">
  <Section
    bgColor={pageColoring.pageColor}
    color={pageColoring.fontColor}
    {fillScreen}
    {arrow}
    {arrowText}>{@render children?.()}</Section
  >
</div>
