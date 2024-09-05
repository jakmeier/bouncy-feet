<script>
  import { t } from '$lib/i18n';
  import Header from '../ui/Header.svelte';
  import BeatVisualizer from './BeatVisualizer.svelte';

  export let counter = -1;
  let start = Date.now();
  export let lastTap = Date.now();
  export let bpm = 0;
  export let bpmSelected = false;
  export let useFixedBpm = false;

  const fixedBpmOptions = [80, 100, 120, 132];

  $: if (counter > 0 && !useFixedBpm)
    bpm = Math.round((counter * 60_000) / (lastTap - start));
  $: bpmSelected = (counter > 0 || useFixedBpm) && bpm > 0;
  $: displayedBpm = bpmSelected ? bpm : '?';

  function reset() {
    counter = -1;
    lastTap = Date.now();
    useFixedBpm = false;
  }

  function tap() {
    lastTap = Date.now();
    if (useFixedBpm) {
      useFixedBpm = false;
      counter = -1;
    }
    if (counter === -1) {
      start = lastTap;
    }
    counter += 1;
  }

  /** @param {number} value */
  function setFixedBpm(value) {
    useFixedBpm = true;
    bpm = value;
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

  <div class="fixed-bpm">
    <div>{$t('record.metronome-subtitle')}</div>
    <div class="fixed-bpm-options">
      {#each fixedBpmOptions as value}
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <!-- svelte-ignore a11y-no-static-element-interactions -->
        <div class="fixed-bpm-option" on:click={() => setFixedBpm(value)}>
          {value}
        </div>
      {/each}
    </div>
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
  .fixed-bpm-options {
    display: flex;
    justify-content: center;
  }
  .fixed-bpm-option {
    padding: 5px;
    margin: 5px;
    font-size: 28px;
    cursor: pointer;
    background-color: var(--theme-main);
    color: var(--theme-neutral-white);
    border-radius: 4px;
  }
</style>
