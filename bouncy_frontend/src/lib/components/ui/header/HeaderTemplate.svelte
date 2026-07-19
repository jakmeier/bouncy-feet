<script>
  import { pageTransition, publishCoachDest } from '$lib/stores/UiState.svelte';
  import FormattedText from '../FormattedText.svelte';

  /**
   * @typedef {Object} Props
   * @property {string} [title]
   * @property {string} bgColor
   * @property {string} accentColor
   * @property {import('svelte').Snippet} topLeft
   * @property {import('svelte').Snippet} topRight
   */

  /** @type {Props} */
  let { title = '', bgColor, accentColor, topLeft, topRight } = $props();
  let titleNode = $state();

  $effect(() => {
    publishCoachDest(titleNode);
  });
</script>

<header style="background-color: {bgColor};">
  <div class="buttons-wrapper">
    {@render topLeft()}

    <div class="right-button-group">
      {@render topRight()}
    </div>
  </div>
  <h1
    class="title"
    class:transition-target={pageTransition.kind === 'coach' &&
      pageTransition.phase === 'settle'}
    bind:this={titleNode}
  >
    <FormattedText text={title} color={accentColor}></FormattedText>
  </h1>
</header>

<style>
  header {
    display: flex;
    flex-direction: column;
    margin-top: 1.5rem;
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
    justify-content: right;
  }

  /* Keep it measurable while its a clone replaces it. */
  .transition-target {
    opacity: 0;
  }
</style>
