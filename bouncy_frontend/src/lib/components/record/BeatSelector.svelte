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

  <div class="visualizer">
    <BeatVisualizer size={100} />
  </div>

  <div class="tap-container">
    <button class="tap" on:click={tap}>
      {$t('record.bpm-tap-button')}
    </button>
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
    border: var(--theme-neutral-dark) solid 1px;
    border-radius: 20px;
    margin: 10px auto;
  }
  .bpm {
    font-size: 50px;
    margin-top: 20px;
  }
  .tap {
    width: 200px;
    height: 150px;
    background-color: var(--theme-neutral-light);
    border: var(--theme-main) solid 3px;
    border-radius: 20px;
    box-shadow:
      0 4px 0 var(--theme-neutral-dark),
      0 5px 15px -4px var(--theme-neutral-dark);
    margin-top: 0px;
    margin-bottom: 6px;
  }
  .tap:active {
    box-shadow: none;
    margin-top: 6px;
    margin-bottom: 0px;
    transition:
      box-shadow 0.05s ease-in,
      margin-top 0.05s ease-in,
      margin-bottom 0.05s ease-in;
  }
  .reset {
    width: 80px;
    height: 30px;
    background-color: var(--theme-neutral-dark);
    color: var(--theme-neutral-white);
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
