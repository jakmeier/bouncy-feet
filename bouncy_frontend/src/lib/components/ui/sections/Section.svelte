<script>
  /**
   * @typedef {Object} Props
   * @property {string | undefined} [bgColor]
   * @property {string | undefined} [color]
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let { bgColor = undefined, color = undefined, children } = $props();
</script>

<section
  class="section-container"
  style="background-color: {bgColor}; color: {color}; --background-color: {bgColor}; --color: {color};"
>
  {@render children?.()}
</section>

<style>
  .section-container {
    padding: 0.5rem 1.5rem;
    margin: 0rem -1.5rem;
  }

  @media (min-width: 730px) {
    .section-container {
      border-radius: 5px;
    }
  }
</style>
