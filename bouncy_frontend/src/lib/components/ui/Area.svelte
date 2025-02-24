<script>
  /**
   * @typedef {Object} Props
   * @property {string} [width]
   * @property {string} [height]
   * @property {string} [borderRadius]
   * @property {string} [borderWidth]
   * @property {number} [zIndex]
   * @property {string} [backgroundColor]
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let {
    width = '250px',
    height = '250px',
    borderRadius = '100px',
    borderWidth = '5px',
    zIndex = 0,
    backgroundColor = 'none',
    children
  } = $props();
</script>

<div
  class="area"
  style="height: {height}; width: {width}; border-radius: {borderRadius}; border: {borderWidth} var(--theme-neutral-dark) solid; {zIndex ===
  0
    ? ''
    : 'z-index: ' + zIndex}
    background-color: {backgroundColor}"
>
  {@render children?.()}
</div>

<style>
  .area {
    position: relative;
    display: grid;
    align-items: center;
    justify-items: center;
    margin: 5px auto;
    overflow: hidden;
  }
</style>
