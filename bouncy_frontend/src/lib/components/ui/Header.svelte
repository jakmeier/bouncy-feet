<script>
  import BackButton from './BackButton.svelte';

  /** @type {string} */
  export let title;
  /** @type {null|string} */
  export let button = null;
  export let backButton = true;

  /** @type {Element} */
  let titleElement;
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
    min-height: 50px;
    align-content: center;
    align-items: center;
  }

  .title-container {
    margin: auto;
    height: fit-content;
    width: calc(100% - 25px);
    padding: 10px;
    border-radius: 2px;
    text-align: center;
    font-size: var(--font-large);
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
