<script>
  /**
   * @typedef {Object} Props
   * @property {string} [color]
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let { children, color = 'var(--theme-main)' } = $props();
</script>

<div class="outer" style="--corner-color: {color}">
  <div class="corner-marked2">
    <div class="corner-marked">
      {@render children?.()}
    </div>
  </div>
</div>

<style>
  .outer {
    position: relative;
    display: flex;
    width: 100%;
    height: 100%;
  }

  .corner-marked {
    position: relative;
    padding: 0.25rem;
    width: 100%;
    height: 100%;
  }

  .corner-marked2 {
    position: relative;
    margin: 0;
    padding: 0.125rem;
  }

  .corner-marked2::before,
  .corner-marked2::after,
  .corner-marked::before,
  .corner-marked::after {
    content: '';
    position: absolute;
    width: min(3em, 25%);
    height: min(3em, 25%);
    padding: 0;
    margin: 0;
    border: 0.125rem solid var(--corner-color);
    border-radius: 2px;
  }

  .corner-marked::before {
    top: 0.125rem;
    left: 0.125rem;
    border-right: none;
    border-bottom: none;
  }

  .corner-marked::after {
    bottom: 0.125rem;
    right: 0.125rem;
    border-left: none;
    border-top: none;
  }

  .corner-marked2::before {
    top: 0.25rem;
    right: -0.25rem;
    border-left: none;
    border-bottom: none;
  }

  .corner-marked2::after {
    bottom: -0.25rem;
    left: 0.25rem;
    border-right: none;
    border-top: none;
  }
</style>
