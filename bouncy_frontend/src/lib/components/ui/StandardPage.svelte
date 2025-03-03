<script>
  import Footer from '$lib/components/ui/Footer.svelte';
  import LogoHeader from '$lib/components/ui/LogoHeader.svelte';

  /**
   * @typedef {Object} Props
   * @property {import('svelte').Snippet} [children]
   * @property {string} title
   * @property {boolean} white
   */
  /** @type {Props} */
  let { children, title, white } = $props();
</script>

<div class="wrapper">
  <div>
    <LogoHeader {white} {title} />
  </div>

  {@render children?.()}

  <div class="footer">
    <Footer white={!white} />
  </div>
</div>

<style>
  .wrapper {
    position: relative;
    height: 100dvh;
    margin: 0;
  }

  .footer {
    position: absolute;
    bottom: 1rem;
  }
</style>
