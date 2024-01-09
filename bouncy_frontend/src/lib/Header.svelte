<script>
  import BackButton from './BackButton.svelte';

  /** @type {string} */
  export let title;
</script>

<header>
  <BackButton />
  <span>{title}</span>
</header>

<style>
  header {
    display: grid;
    grid-template-columns: auto 1fr;
    height: 50px;
  }

  span {
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
</style>
