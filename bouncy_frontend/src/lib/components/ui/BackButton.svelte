<script>
  function goBack() {
    window.history.back();
  }
</script>

<button on:click={goBack}>
  <span class="material-symbols-outlined" translate="no"> chevron_left </span>
</button>

<style>
  button {
    width: 50px;
    height: 50px;
    padding: auto;
    background-color: var(--theme-neutral-light);
    color: var(--theme-neutral-dark);
  }
  button span {
    font-size: var(--font-large);
  }
</style>
