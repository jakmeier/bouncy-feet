<script>
  /**
   * @typedef {Object} Props
   * @property {string} [color]
   */

  /** @type {Props} */
  let { color = 'var(--color)' } = $props();
</script>

<svg width="100mm" height="100mm" viewBox="0 0 100 100">
  <path
    style="fill:none;stroke:{color};stroke-width:10;stroke-linecap:round;stroke-linejoin:miter;stroke-dasharray:none;stroke-opacity:1"
    d="M 5,50 H 95 M 50,5.5 V 95"
    id="path299"
  />
</svg>

<style>
  svg {
    max-width: inherit;
    max-height: inherit;
  }
</style>
