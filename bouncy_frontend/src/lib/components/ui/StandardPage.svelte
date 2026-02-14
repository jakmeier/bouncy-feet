<script>
  // A simple one-view pager with header and footer.
  //
  // Pages that need multiple sections should not use this component and rather
  // add header and footers manually.
  import Footer from '$lib/components/ui/Footer.svelte';
  import LogoHeader from '$lib/components/ui/header/LogoHeader.svelte';
  import Background from './sections/Background.svelte';

  /**
   * @typedef {Object} Props
   * @property {import('svelte').Snippet} [children]
   * @property {string} [title]
   * @property {boolean} [white]
   * @property {boolean} [black]
   * @property {boolean} [accent]
   * @property {boolean} [mainColor]
   * @property {boolean} [backButton]
   */
  /** @type {Props} */
  let { children, title, white, black, accent, mainColor, backButton } =
    $props();
</script>

{#if mainColor}
  <Background bgColor="var(--theme-main-alt)" color="var(--theme-neutral-black)"
  ></Background>
{:else if white}
  <Background
    bgColor="var(--theme-neutral-light)"
    color="var(--theme-neutral-black)"
  ></Background>
{:else if black}
  <Background
    bgColor="var(--theme-neutral-black)"
    color="var(--theme-neutral-white)"
  ></Background>
{/if}

<div class="wrapper">
  <div>
    <LogoHeader {white} {black} {accent} {mainColor} {title} {backButton} />
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
</style>
