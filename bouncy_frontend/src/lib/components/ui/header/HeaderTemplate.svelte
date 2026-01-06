<script>
  import { receivePersonalityTitle } from '$lib/stores/Crossfade.svelte';
  import { fadingOut } from '$lib/stores/UiState.svelte';

  /**
   * @typedef {Object} Props
   * @property {string} [title]
   * @property {string} bgColor
   * @property {import('svelte').Snippet} topLeft
   * @property {import('svelte').Snippet} topRight
   */

  /** @type {Props} */
  let { title = '', bgColor, topLeft, topRight } = $props();
</script>

<header style="background-color: {bgColor};">
  <div class="buttons-wrapper">
    {@render topLeft()}

    <div class="right-button-group">
      {@render topRight()}
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
</style>
