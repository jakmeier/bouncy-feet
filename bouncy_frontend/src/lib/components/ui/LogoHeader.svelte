<script>
  import { base } from '$app/paths';

  /**
   * @typedef {Object} Props
   * @property {string} [title]
   * @property {boolean} [white]
   * @property {boolean} [gray]
   */

  /** @type {Props} */
  let { white = false, gray = false, title = '' } = $props();

  let bgColor = $derived(
    white
      ? 'var(--theme-neutral-light)'
      : gray
        ? 'var(--theme-neutral-dark)'
        : 'var(--theme-neutral-black)'
  );

  let imgUrl = $derived(
    white
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
    display: grid;
    grid-template-columns: auto 1fr auto;
    min-height: 50px;
    align-content: center;
    align-items: center;
    height: 5rem;
  }

  header img.logo {
    height: 3rem;
  }

  header h1 {
    text-align: right;
    /* adjust to make font look centered vertically */
    line-height: 1.15em;
    padding-top: 0.15em;
  }
</style>
