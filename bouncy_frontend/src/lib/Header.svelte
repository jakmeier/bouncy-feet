<script>
  import BackButton from './BackButton.svelte';

  /** @type {string} */
  export let title;
  /** @type {null|string} */
  export let button = null;
</script>

<header>
  <BackButton />
  <span class="title">{title}</span>
  {#if button !== null}
    <button on:click>
      <span class="material-symbols-outlined button"> {button} </span>
    </button>
  {/if}
</header>

<style>
  header {
    display: grid;
    grid-template-columns: auto 1fr auto;
    height: 50px;
  }

  span.title {
    margin: auto;
    height: 30px;
    width: calc(100% - 25px);
    padding: 10px;
    background-color: var(--theme-neutral-light);
    border-radius: 2px;
    text-align: center;
    font-size: 28px;
    overflow: hidden auto;
  }

  button {
    width: 50px;
    height: 50px;
    padding: auto;
    background-color: var(--theme-neutral-light);
  }
</style>
