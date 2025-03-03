<script>
  import Button from '$lib/components/ui/Button.svelte';

  let { showResults = $bindable() } = $props();
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
    height: 62dvh;
    justify-content: center;
    align-items: center;
  }
</style>
