<script>
  /**
   * @typedef {Object} Props
   * @property {string} [color]
   */

  /** @type {Props} */
  let { color = 'var(--color)' } = $props();
</script>

<svg width="100mm" height="60mm" viewBox="0 0 100 60">
  <path
    style="fill:none;stroke:{color};stroke-width:10;stroke-linecap:round;stroke-linejoin:miter;stroke-dasharray:none;stroke-opacity:1"
    d="M 5,5 50,55 M 95,5 50,55"
    id="pathid"
  />
</svg>

<style>
  svg {
    max-width: inherit;
    max-height: inherit;
  }
</style>
