<script>
  import Section from './Section.svelte';
  /**
   * @typedef {Object} Props
   * @property {boolean} [fillScreen]
   * @property {boolean} [arrow]
   * @property {string} [arrowText]
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let { fillScreen, arrow, arrowText, children } = $props();
</script>

<Section
  bgColor={'var(--theme-neutral-dark)'}
  color={'var(--theme-main)'}
  {fillScreen}
  {arrow}
  {arrowText}>{@render children?.()}</Section
>
