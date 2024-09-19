<script>
  import Button from '$lib/components/ui/Button.svelte';
  import { onDestroy, onMount } from 'svelte';
  import { backgroundColor } from '$lib/stores/UiState';

  export let showResults;

  let swapBackgroundColor = 'var(--theme-neutral-white)';
  onMount(() => {
    swapBackgroundColor = $backgroundColor;
    $backgroundColor = 'var(--theme-neutral-light)';
  });
  onDestroy(() => {
    $backgroundColor = swapBackgroundColor;
  });
</script>

<div class="outer">
  <Button
    class="light big"
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
