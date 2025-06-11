<script>
  // A section is marked by a separate background color and it typically fills
  // the screen. It can have a small arrow hinting that it can be scrolled.
  //
  // Adding headers and footers is not the responsibility of a section. But they
  // can be child snippets.
  import NextSectionArrow from '../NextSectionArrow.svelte';

  /**
   * @typedef {Object} Props
   * @property {string | undefined} [bgColor]
   * @property {string | undefined} [color]
   * @property {boolean} [fillScreen]
   * @property {boolean} [arrow]
   * @property {string} [arrowText]
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let {
    bgColor = undefined,
    color = undefined,
    fillScreen = false,
    arrow = false,
    arrowText = undefined,
    children,
  } = $props();
</script>

<section
  class="section-container"
  class:fill-screen={fillScreen}
  style="background-color: {bgColor}; color: {color}; --background-color: {bgColor}; --color: {color};"
>
  {@render children?.()}

  {#if arrow}
    <NextSectionArrow text={arrowText} {color}></NextSectionArrow>
  {/if}
</section>

<style>
  .section-container {
    padding: 0.1px 1.5rem;
    margin: 0rem -1.5rem;
    /* needed for arrow */
    display: flex;
    flex-direction: column;
  }

  .fill-screen {
    min-height: 100dvh;
  }

  @media (min-width: 730px) {
    .section-container {
      border-radius: 5px;
    }
  }
</style>
