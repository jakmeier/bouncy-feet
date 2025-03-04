<script>
  import Footer from '$lib/components/ui/Footer.svelte';
  import LogoHeader from '$lib/components/ui/LogoHeader.svelte';

  /**
   * @typedef {Object} Props
   * @property {import('svelte').Snippet} [children]
   * @property {string} [title]
   * @property {boolean} [white]
   * @property {boolean} [gray]
   */
  /** @type {Props} */
  let { children, title, white, gray } = $props();
</script>

<div class="wrapper">
  <div>
    <LogoHeader {white} {gray} {title} />
  </div>

  <div class="content">
    {@render children?.()}
  </div>

  <div class="footer">
    <Footer white={!white} />
  </div>
</div>

<style>
  .wrapper {
    display: flex;
    flex-direction: column;
    min-height: 100dvh;
    margin: 0;
  }

  .content {
    flex-grow: 1;
  }

  .footer {
    margin-top: 3rem;
    margin-bottom: 1rem;
  }
</style>
