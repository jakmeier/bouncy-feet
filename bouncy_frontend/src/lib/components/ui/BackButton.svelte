<script>
  import Arrow from './Arrow.svelte';

  let { color = undefined } = $props();

  function goBack() {
    window.history.back();
  }
</script>

<button onclick={goBack}>
  <div class="arrow">
    <Arrow {color} />
  </div>
</button>

<style>
  button {
    min-width: unset;
    width: 1rem;
    height: 2rem;
    padding: 0 10px;
    background-color: transparent;
    display: grid;
    justify-content: center;
    align-content: center;
    margin: auto;
  }
  .arrow {
    max-width: 2rem;
    max-height: 1.2rem;
    rotate: 90deg;
  }
</style>
