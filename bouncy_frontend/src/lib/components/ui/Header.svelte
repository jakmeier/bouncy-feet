<script>
  import { createBubbler } from 'svelte/legacy';

  const bubble = createBubbler();
  import BackButton from './BackButton.svelte';

  
  
  /**
   * @typedef {Object} Props
   * @property {string} title
   * @property {null|string} [button]
   * @property {boolean} [backButton]
   */

  /** @type {Props} */
  let { title, button = null, backButton = true } = $props();

  /** @type {Element} */
  let titleElement = $state();
</script>

<header>
  {#if backButton}
    <BackButton />
  {:else}
    <div></div>
  {/if}
  <div class="title-container">
    <span class="title" bind:this={titleElement}>{title}</span>
  </div>
  {#if button !== null}
    <button onclick={bubble('click')}>
      <span class="material-symbols-outlined button" translate="no">
        {button}
      </span>
    </button>
  {/if}
</header>

<style>
  header {
    display: grid;
    grid-template-columns: auto 1fr auto;
    min-height: 50px;
    align-content: center;
    align-items: center;
  }

  .title-container {
    margin: auto;
    min-height: 1.15em;
    width: calc(100% - 25px);
    /* padding: 10px; */
    border-radius: 2px;
    text-align: right;
    font-size: var(--font-large);
    /* adjust to make font look centered vertically */
    line-height: 1.15em;
    padding-top: 0.15em;
  }

  span {
    display: inline-block;
    text-transform: capitalize;
  }

  button {
    width: 30px;
    height: 30px;
    padding: auto;
    background-color: var(--theme-neutral-light);
    color: var(--theme-neutral-dark);
  }
</style>
