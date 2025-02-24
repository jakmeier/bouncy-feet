<script>
  import Section from './Section.svelte';
  /**
   * @typedef {Object} Props
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let { children } = $props();
</script>

<Section
  bgColor={'var(--theme-neutral-light)'}
  color={'var(--theme-neutral-dark)'}>{@render children?.()}</Section
>
