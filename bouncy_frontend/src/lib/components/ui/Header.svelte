<script>
  import { afterUpdate } from 'svelte';
  import BackButton from './BackButton.svelte';

  /** @type {string} */
  export let title;
  /** @type {null|string} */
  export let button = null;
  export let backButton = true;

  /** @type {Element} */
  let titleElement;

  function adjustFontSize() {
    let initialFontSize = 28;
    titleElement.style.fontSize = initialFontSize + 'px';
    const maxWidth =
      titleElement.parentElement.getBoundingClientRect().width - 20;
    const width = titleElement.getBoundingClientRect().width;
    const ratio = maxWidth / width;
    if (ratio < 1) {
      titleElement.style.fontSize = ratio * initialFontSize + 'px';
    }
  }

  afterUpdate(() => {
    adjustFontSize();
  });
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
    <button on:click>
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
    height: 50px;
  }

  .title-container {
    margin: auto;
    height: 30px;
    width: calc(100% - 25px);
    padding: 10px;
    background-color: var(--theme-neutral-light);
    border-radius: 2px;
    text-align: center;
    font-size: 28px;
    overflow: hidden auto;
    white-space: nowrap;
  }

  span {
    display: inline-block;
    text-transform: capitalize;
  }

  button {
    width: 50px;
    height: 50px;
    padding: auto;
    background-color: var(--theme-neutral-light);
    color: var(--theme-neutral-dark);
  }
</style>
