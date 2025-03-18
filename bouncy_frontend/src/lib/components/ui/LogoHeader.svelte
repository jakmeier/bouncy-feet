<script>
  import { base } from '$app/paths';

  /**
   * @typedef {Object} Props
   * @property {string} [title]
   * @property {boolean} [white]
   * @property {boolean} [gray]
   * @property {boolean} [accent]
   * @property {boolean} [mainColor]
   */

  /** @type {Props} */
  let {
    white = false,
    gray = false,
    accent = false,
    mainColor = false,
    title = '',
  } = $props();

  let bgColor = $derived(
    white
      ? 'var(--theme-neutral-light)'
      : gray
        ? 'var(--theme-neutral-dark)'
        : accent
          ? 'var(--theme-accent)'
          : mainColor
            ? 'var(--theme-main)'
            : 'var(--theme-neutral-black)'
  );

  let imgUrl = $derived(
    white || accent || mainColor
      ? `${base}/icons/logo.svg`
      : `${base}/icons/icon_tight_on_transparent.png`
  );
</script>

<header style="background-color: {bgColor};">
  <img class="logo" src={imgUrl} alt="Bouncy Feet Logo" />
  <h1 class="title">{title}</h1>
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
</style>
