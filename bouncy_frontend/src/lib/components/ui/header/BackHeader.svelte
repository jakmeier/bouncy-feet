<script>
  import Arrow from '../svg/Arrow.svelte';
  import UnstyledButton from '../UnstyledButton.svelte';
  import HeaderTemplate from './HeaderTemplate.svelte';

  /**
   * @typedef {Object} Props
   * @property {string} [title]
   * @property {boolean} [white]
   * @property {boolean} [black]
   * @property {boolean} [accent]
   * @property {boolean} [mainColor]
   * @property {boolean} [transparent]
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
</script>

<HeaderTemplate {bgColor} {title}>
  {#snippet topLeft()}
    <UnstyledButton onClick={onBack}>
      <div class="arrow">
        <Arrow />
      </div>
    </UnstyledButton>
  {/snippet}

  {#snippet topRight()}{/snippet}
</HeaderTemplate>

<style>
  .arrow {
    max-width: 2rem;
    max-height: 1.2rem;
    transform: rotate(90deg) translateY(-100%);
    transform-origin: top left;
  }
</style>
