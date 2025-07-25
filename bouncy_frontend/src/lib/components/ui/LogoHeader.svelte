<script>
  import { base } from '$app/paths';
  import { receivePersonalityTitle } from '$lib/stores/Crossfade.svelte';
  import Arrow from './svg/Arrow.svelte';
  import { fadingOut } from '$lib/stores/UiState.svelte';

  /**
   * @typedef {Object} Props
   * @property {string} [title]
   * @property {boolean} [white]
   * @property {boolean} [black]
   * @property {boolean} [accent]
   * @property {boolean} [mainColor]
   * @property {boolean} [transparent]
   * @property {boolean} [backButton]
   * @property {()=>void} [onBack]
   */

  /** @type {Props} */
  let {
    white = false,
    black = false,
    accent = false,
    mainColor = false,
    transparent,
    title = '',
    backButton = false,
    onBack = () => {
      window.history.back();
    },
  } = $props();

  let bgColor = $derived(
    white
      ? 'var(--theme-neutral-light)'
      : black
        ? 'var(--theme-neutral-dark)'
        : accent
          ? 'var(--theme-accent)'
          : mainColor
            ? 'var(--theme-main-alt)'
            : transparent
              ? 'transparent'
              : 'var(--theme-neutral-black)'
  );

  let imgUrl = $derived(
    white || accent || mainColor
      ? `${base}/icons/logo.svg`
      : `${base}/icons/icon_tight_on_transparent.png`
  );
</script>

<header style="background-color: {bgColor};">
  {#if backButton}
    <div class="arrow-wrapper" onclick={onBack}>
      <div class="arrow">
        <Arrow />
      </div>
    </div>
  {:else}
    <img class="logo" src={imgUrl} alt="Bouncy Feet Logo" />
  {/if}
  {#if !fadingOut.text}
    <h1
      class="title"
      in:receivePersonalityTitle={{
        key: 'pageTitle',
      }}
    >
      {title}
    </h1>
  {/if}
</header>

<style>
  header {
    display: flex;
    flex-direction: column;
    margin-top: 1.5rem;
  }

  header img.logo {
    height: 3rem;
    align-self: baseline;
  }

  .arrow-wrapper {
    height: 2rem;
    display: flex;
    align-items: flex-start;
    justify-content: flex-start;
  }

  .arrow {
    max-width: 2rem;
    max-height: 1.2rem;
    transform: rotate(90deg) translateY(-100%);
    transform-origin: top left;
  }
</style>
