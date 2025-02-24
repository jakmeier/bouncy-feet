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
    width: 15px;
    height: 30px;
    padding: 0 10px;
    background-color: transparent;
  }
  .arrow {
    margin: auto;
    max-width: 15px;
    max-height: 30px;
    transform: scaleX(-1);
  }
</style>
