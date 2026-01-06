<script>
  import Arrow from '../svg/Arrow.svelte';
  import Plus from '../svg/Plus.svelte';
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
   * @property {null|string} [button]
   * @property {()=>void} [onBack]
   * @property {()=>void} onAction
   */

  /** @type {Props} */
  let {
    white = false,
    black = false,
    accent = false,
    mainColor = false,
    transparent,
    title = '',
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
</script>

<HeaderTemplate {bgColor} {title}>
  {#snippet topLeft()}
    <UnstyledButton onClick={onBack}>
      <div class="arrow">
        <Arrow />
      </div>
    </UnstyledButton>
  {/snippet}

  {#snippet topRight()}
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
  {/snippet}
</HeaderTemplate>

<style>
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
</style>
