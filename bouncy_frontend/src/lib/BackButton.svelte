<script>
  function goBack() {
    window.history.back();
  }
</script>

<button on:click={goBack}>
  <span class="material-symbols-outlined"> chevron_left </span>
</button>

<style>
  button {
    width: 50px;
    height: 50px;
    padding: auto;
    background-color: var(--theme-neutral-light);
  }
  button span {
    font-size: 30px;
  }
</style>
