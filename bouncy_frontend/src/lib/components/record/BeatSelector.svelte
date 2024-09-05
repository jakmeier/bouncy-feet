<script>
  import { t } from '$lib/i18n';
  import Header from '../ui/Header.svelte';
  import BeatVisualizer from './BeatVisualizer.svelte';

  export let counter = -1;
  let start = Date.now();
  export let lastTap = Date.now();
  export let bpm = 0;

  $: if (counter > 0) bpm = Math.round((counter * 60_000) / (lastTap - start));
  $: displayedBpm = counter > 0 ? bpm : '?';

  function reset() {
    counter = -1;
    lastTap = Date.now();
  }

  function tap() {
    lastTap = Date.now();
    if (counter === -1) {
      start = lastTap;
    }
    counter += 1;
  }
</script>

<Header title={$t('record.prepare-title')} backButton></Header>

<div class="outer">
  <div class="bpm-container">
    <div class="bpm">
      {displayedBpm}
    </div>
    <div>
      {$t('record.estimated-bpm-label')}
    </div>
    <button class="reset" on:click={reset}>
      <span class="material-symbols-outlined button"> cancel </span>
    </button>
  </div>

  <div class="visualizer" on:pointerdown={tap}>
    <BeatVisualizer size={200}>
      {$t('record.bpm-tap-button')}
    </BeatVisualizer>
  </div>
</div>

<style>
  .bpm-container {
    display: grid;
    text-align: center;
    justify-content: center;
    justify-items: center;
    width: 200px;
    height: 160px;
    border-radius: 20px;
    margin: 10px auto;
  }
  .bpm {
    font-size: 50px;
    margin-top: 20px;
  }
  .reset {
    width: 80px;
    height: 30px;
    background-color: var(--theme-accent-light);
    color: var(--theme-neutral-dark);
    text-align: center;
    padding: 0;
  }
  span {
    line-height: 30px;
    font-size: 28px;
  }
  .visualizer {
    padding: 20px;
  }
</style>
