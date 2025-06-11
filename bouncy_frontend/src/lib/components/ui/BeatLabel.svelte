<script>
  /**
   * @typedef {Object} Props
   * @property {boolean} [marked]
   * @property {string} [baseColor]
   * @property {string} [baseBgColor]
   * @property {string} [markedColor]
   * @property {string} [markedBgColor]
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let {
    children,
    marked,
    baseBgColor = 'var(--theme-neutral-dark)',
    baseColor = 'var(--theme-neutral-white)',
    markedBgColor = 'var(--theme-accent)',
    markedColor = 'var(--theme-neutral-white)',
  } = $props();
</script>

<div
  class="beat-label"
  class:marked
  style="background-color: {marked
    ? markedBgColor
    : baseBgColor}; color: {marked ? markedColor : baseColor}"
>
  {@render children?.()}
</div>

<style>
  .beat-label {
    display: grid;
    align-content: center;
    justify-content: center;
    margin: auto;
    padding: 0.5rem;
    min-width: 1rem;
    height: 1rem;
    font-size: var(--font-normal);
    border-radius: 2rem;
    width: fit-content;
  }

  .marked {
    font-weight: 800;
  }
</style>
