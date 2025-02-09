<script>
  import Arrow from './Arrow.svelte';

  export let color = undefined;

  function goBack() {
    window.history.back();
  }
</script>

<button on:click={goBack}>
  <div class="arrow">
    <Arrow {color} />
  </div>
</button>

<style>
  button {
    width: 45px;
    height: 50px;
    padding: 0 10px;
    background-color: transparent;
  }
  .arrow {
    margin: auto;
    max-width: 50px;
    max-height: 50px;
    transform: scaleX(-1);
  }
</style>
