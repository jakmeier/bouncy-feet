<script>
  import { base } from '$app/paths';

  export let bgColor = 'var(--theme-neutral-black)';
</script>

<div class="title" style="background-color: {bgColor};">
  <img
    class="logo"
    src="{base}/icons/icon_tight_on_black.svg"
    alt="Bouncy Feet Logo"
  />
</div>

<style>
  .title {
    position: relative;
    padding: 1rem 0;

    /* color: var(--theme-neutral-white); */
    z-index: 1;
    height: calc(4rem + 5px);
  }
  .title img.logo {
    height: 3rem;
  }
</style>
