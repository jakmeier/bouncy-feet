<script>
  import { base } from '$app/paths';
  import { receivePersonalityTitle } from '$lib/stores/Crossfade.svelte';
  import Arrow from './svg/Arrow.svelte';
  import { fadingOut } from '$lib/stores/UiState.svelte';
  import Plus from './svg/Plus.svelte';
  import UnstyledButton from './UnstyledButton.svelte';

  /**
   * @typedef {Object} Props
   * @property {string} [title]
   * @property {boolean} [white]
   * @property {boolean} [black]
   * @property {boolean} [accent]
   * @property {boolean} [mainColor]
   * @property {boolean} [transparent]
   * @property {boolean} [backButton]
   * @property {null|string} [button]
   * @property {()=>void} [onBack]
   * @property {()=>void} [onAction] -- create a plus icon on the top right and trigger this function when selected
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
    button = null,
    onBack = () => {
      window.history.back();
    },
    onAction,
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
  <div class="buttons-wrapper">
    {#if backButton}
      <UnstyledButton onClick={onBack}>
        <div class="arrow">
          <Arrow />
        </div>
      </UnstyledButton>
    {:else}
      <img class="logo" src={imgUrl} alt="Bouncy Feet Logo" />
    {/if}
    {#if onAction}
      <UnstyledButton onClick={onAction}>
        <div class="action">
          {#if button}
            <span class="material-symbols-outlined button" translate="no">
              {button}
            </span>
          {:else}
            <Plus />
          {/if}
        </div>
      </UnstyledButton>
    {/if}
  </div>
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

  .buttons-wrapper {
    height: 2rem;
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
  }

  .arrow {
    max-width: 2rem;
    max-height: 1.2rem;
    transform: rotate(90deg) translateY(-100%);
    transform-origin: top left;
  }

  .action {
    max-width: 2rem;
    max-height: 2rem;
  }

  .action span {
    font-size: 2rem;
  }
</style>
