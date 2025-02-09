<script>
  import { t } from '$lib/i18n';
  import {
    bpm,
    setBeatStart,
    setBpm,
    beatStart,
    beatDuration,
  } from '$lib/stores/Beat';
  import Button from '../ui/Button.svelte';
  import Header from '../ui/Header.svelte';
  import Toggle from '../ui/Toggle.svelte';
  import BeatVisualizer from './BeatVisualizer.svelte';

  export let counter = -1;
  export let bpmSelected = false;
  export let useFixedBpm = false;

  let start = performance.now();
  let lastTap = performance.now();

  const fixedBpmOptions = [80, 100, 120, 132];
  const timeout = 2000;

  $: bpmSelected = counter > 0 || useFixedBpm;
  $: displayedBpm = bpmSelected ? $bpm : '?';

  function reset() {
    counter = -1;
    setBeatStart(performance.now());
    useFixedBpm = false;
  }

  function tap() {
    const now = performance.now();
    if (lastTap + timeout < now) {
      counter = -1;
    }
    lastTap = now;
    setBeatStart(lastTap);

    if (useFixedBpm) {
      useFixedBpm = false;
      counter = -1;
    }
    if (counter === -1) {
      start = lastTap;
    }
    counter += 1;

    if (counter > 2 && !useFixedBpm) {
      setBpm(Math.round((counter * 60_000) / (lastTap - start)));
    }
  }

  /** @param {number} bpm */
  function setFixedBpm(bpm) {
    useFixedBpm = true;
    setBpm(bpm);
    setBeatStart(performance.now());
  }
</script>

<Header title={$t('record.prepare-title')} backButton></Header>

<div class="outer">
  <div class="visualizer" on:pointerdown={tap}>
    <BeatVisualizer
      size={200}
      accentColor={!useFixedBpm}
      start={$beatStart}
      timeBetweenBeats={$beatDuration}
    >
      {useFixedBpm
        ? $t('record.bpm-fixed-button')
        : $t('record.bpm-tap-button')}
    </BeatVisualizer>
  </div>

  <div class="bpm-container">
    <div class="bpm">
      {displayedBpm}
    </div>
    <div>
      {$t('record.estimated-bpm-label')}
    </div>
    {#if !useFixedBpm}
      <Button
        class="reset"
        on:click={reset}
        symbolClass="thin"
        symbolSize={28}
        symbol="cancel"
      ></Button>
    {/if}
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

  <div class="bpm-source">
    <div>{$t('record.bpm-source-toggle-label')}</div>
    <Toggle bind:isOn={useFixedBpm}></Toggle>
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
    margin: 10px auto 30px;
  }
  .bpm {
    font-size: var(--font-large);
  }
  .visualizer {
    padding-top: 20px;
  }
  .fixed-bpm-options {
    display: flex;
    justify-content: center;
  }
  .fixed-bpm-option {
    padding: 5px;
    margin: 5px;
    font-size: var(--font-normal);
    cursor: pointer;
    background-color: var(--theme-main);
    color: var(--theme-neutral-white);
    border-radius: 4px;
  }
  .bpm-source {
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 10px;
    gap: 20px;
  }
</style>
