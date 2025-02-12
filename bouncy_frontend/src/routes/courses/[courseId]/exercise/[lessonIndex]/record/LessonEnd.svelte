<script>
  import Button from '$lib/components/ui/Button.svelte';

  export let showResults;
</script>

<div class="outer">
  <Button
    class="big"
    on:click={() => (showResults = true)}
    text="record.results-button"
    symbol="trophy"
  ></Button>
</div>

<style>
  .outer {
    display: grid;
    height: 62vh;
    justify-content: center;
    align-items: center;
  }
</style>
