<script>
  import { base } from '$app/paths';

  export let white = false;
</script>

<div class="brand-footer">
  <img
    class="logo"
    src="{base}/icons/bouncyfeet{white ? '_white' : ''}.svg"
    alt="Bouncy Feet Logo"
  />
</div>

<style>
  .brand-footer {
    margin: 0 -0.75rem;
    height: 2rem;
  }
  .brand-footer img {
    max-height: min(10vw, 50px);
    min-height: 1rem;
    max-width: unset;
  }
</style>
