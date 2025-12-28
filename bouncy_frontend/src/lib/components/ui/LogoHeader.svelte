<script>
  import { base } from '$app/paths';
  import { receivePersonalityTitle } from '$lib/stores/Crossfade.svelte';
  import Arrow from './svg/Arrow.svelte';
  import { fadingOut } from '$lib/stores/UiState.svelte';
  import Plus from './svg/Plus.svelte';
  import UnstyledButton from './UnstyledButton.svelte';
  import { goto } from '$app/navigation';
  import { onDestroy, onMount } from 'svelte';
  import { fade, fly } from 'svelte/transition';

  /**
   * @typedef {Object} Props
   * @property {string} [title]
   * @property {boolean} [white]
   * @property {boolean} [black]
   * @property {boolean} [accent]
   * @property {boolean} [mainColor]
   * @property {boolean} [transparent]
   * @property {boolean} [backButton]
   * @property {boolean} [homeLink]
   * @property {null|string} [button]
   * @property {null|string} [secondButton]
   * @property {()=>void} [onBack]
   * @property {()=>void} [onAction] -- create a plus icon on the top right and trigger this function when selected
   * @property {()=>void} [onSecondAction]
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
    homeLink = false,
    button = null,
    secondButton = null,
    onBack = () => {
      window.history.back();
    },
    onAction,
    onSecondAction,
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

  let oddAnimation = $state(true);
  /** @type {number | undefined} */
  let interval;

  onMount(() => {
    interval = setInterval(() => (oddAnimation = !oddAnimation), 5000);
  });
  onDestroy(() => {
    if (interval) {
      clearInterval(interval);
    }
  });
</script>

<header style="background-color: {bgColor};">
  <div class="buttons-wrapper">
    {#if backButton}
      <UnstyledButton onClick={onBack}>
        <div class="arrow">
          <Arrow />
        </div>
      </UnstyledButton>
    {:else if homeLink}
      <UnstyledButton
        onClick={() => {
          goto('/');
        }}
      >
        <div class="animated-logo">
          {#if oddAnimation}
            <img
              class="logo"
              src={imgUrl}
              alt="Bouncy Feet Logo"
              in:fly={{ x: -120 }}
              out:fade={{ delay: 2000 }}
            />
          {:else}
            <div
              class="arrow"
              in:fade={{ duration: 1000, delay: 2000 }}
              out:fade
            >
              <Arrow />
            </div>
          {/if}
        </div>
      </UnstyledButton>
    {:else}
      <img class="logo" src={imgUrl} alt="Bouncy Feet Logo" />
    {/if}
    <div class="right-button-group">
      {#if onSecondAction}
        <UnstyledButton onClick={onSecondAction}>
          <div class="action">
            {#if secondButton}
              <span class="material-symbols-outlined button" translate="no">
                {secondButton}
              </span>
            {:else}
              <Plus />
            {/if}
          </div>
        </UnstyledButton>
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

  .right-button-group {
    height: 2rem;
    display: flex;
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
    margin: 0 1rem;
  }

  .button {
    height: 2rem;
    display: flex;
    align-items: center;
  }

  .action span {
    font-size: 3rem;
  }

  .animated-logo {
    position: relative;
  }
  .animated-logo * {
    position: absolute;
  }
</style>
