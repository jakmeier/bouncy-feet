<script>
  // An arrow down, centered and sized for consistency.
  import Arrow from './Arrow.svelte';

  /**
   * @typedef {Object} Props
   * @property {string} [text]
   * @property {string} [color]
   */

  /** @type {Props} */
  let { text, color = 'var(--theme-neutral-black)' } = $props();
</script>

<div class="down-marker">
  <div class="down-marker-text">{text}</div>
  <div class="down-marker-arrow">
    <Arrow {color} />
  </div>
</div>

<style>
  .down-marker {
    height: 3rem;
    width: 2rem;
    /* requires parent to be flex in column direction */
    margin-top: auto;
    align-self: center; 
  }
  .down-marker-text {
    text-align: center;
  }
  .down-marker-arrow {
    margin: auto;
    max-width: 2rem;
    max-height: 2rem;
  }
</style>
