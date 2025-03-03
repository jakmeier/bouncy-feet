<script>
  import { base } from '$app/paths';

  /**
   * @typedef {Object} Props
   * @property {boolean} [white]
   */

  /** @type {Props} */
  let { white = false } = $props();
</script>

<div class="brand-footer">
  <img
    class="logo"
    src="{base}/icons/bouncyfeet{white ? '_white' : '_black'}.svg"
    alt="Bouncy Feet Logo"
  />
</div>

<style>
  .brand-footer {
    height: 2rem;
  }
  .brand-footer img {
    height: 1.5rem;
    max-width: unset;
  }
</style>
