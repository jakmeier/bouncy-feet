<script>
  import Footer from '$lib/components/ui/Footer.svelte';
  import LogoHeader from '$lib/components/ui/LogoHeader.svelte';
  import Background from './sections/Background.svelte';

  /**
   * @typedef {Object} Props
   * @property {import('svelte').Snippet} [children]
   * @property {string} [title]
   * @property {boolean} [white]
   * @property {boolean} [gray]
   * @property {boolean} [accent]
   * @property {boolean} [mainColor]
   */
  /** @type {Props} */
  let { children, title, white, gray, accent, mainColor } = $props();
</script>

{#if mainColor}
  <Background bgColor="var(--theme-main)" color="var(--theme-neutral-black)"
  ></Background>
{:else if white}
  <Background
    bgColor="var(--theme-neutral-light)"
    color="var(--theme-neutral-black)"
  ></Background>
{/if}

<div class="wrapper">
  <div>
    <LogoHeader {white} {gray} {accent} {mainColor} {title} />
  </div>

  <div class="content">
    {@render children?.()}
  </div>

  <div class="footer">
    <Footer white={!(white || accent || mainColor)} />
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
