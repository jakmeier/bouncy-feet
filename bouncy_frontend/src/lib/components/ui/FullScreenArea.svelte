<script>
  //! Show content on the full available screen, or, if it would exceed that
  //! maximum configured area, on a limited area on top of the other content.

  /**
   * @typedef {Object} Props
   * @property {number} [maxWidth] - ! Usually this means, full-screen on mobile only.
   * @property {number} [maxHeight]
   * @property {string} [backgroundColor]
   * @property {string} [overlayColor]
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let {
    maxWidth = 600,
    maxHeight = 900,
    backgroundColor = 'none',
    overlayColor = 'none',
    children,
  } = $props();
</script>

<div class="overlay" style="background-color: {overlayColor}">
  <div
    class="area"
    style="height: min(100dvh, {maxHeight}px); width: min(100vw, {maxWidth}px); background-color: {backgroundColor};"
  >
    {@render children?.()}
  </div>
</div>

<style>
  .overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 9999;
  }
  .area {
    position: relative;
    display: grid;
    align-items: center;
    justify-items: center;
    margin: auto;
    overflow: hidden;
  }
</style>
