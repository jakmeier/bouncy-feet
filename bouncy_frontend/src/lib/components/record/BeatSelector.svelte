<script>
  import { run } from 'svelte/legacy';

  import { t } from '$lib/i18n';
  import {
    bpm,
    setBeatStart,
    setBpm,
    beatStart,
    beatDuration,
  } from '$lib/stores/Beat';
  import Button from '../ui/Button.svelte';
  import Toggle from '../ui/Toggle.svelte';
  import BeatVisualizer from './BeatVisualizer.svelte';
  import BackHeader from '../ui/header/BackHeader.svelte';

  /**
   * @typedef {Object} Props
   * @property {any} [counter]
   * @property {boolean} [bpmSelected]
   * @property {boolean} [useFixedBpm]
   */

  /** @type {Props} */
  let {
    counter = $bindable(-1),
    bpmSelected = $bindable(false),
    useFixedBpm = $bindable(false),
  } = $props();

  let start = performance.now();
  let lastTap = performance.now();

  const fixedBpmOptions = [80, 100, 120, 132];
  const timeout = 2000;

  run(() => {
    bpmSelected = counter > 0 || useFixedBpm;
  });
  let displayedBpm = $derived(bpmSelected ? $bpm : '?');

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

<BackHeader title={$t('record.prepare-title')} />

<div class="outer">
  <div class="visualizer" onpointerdown={tap}>
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
    <div class="place-holder">
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
  </div>

  <div class="fixed-bpm">
    <div>{$t('record.metronome-subtitle')}</div>
    <div class="fixed-bpm-options">
      {#each fixedBpmOptions as value}
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div class="fixed-bpm-option" onclick={() => setFixedBpm(value)}>
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
    min-height: 6rem;
    border-radius: 20px;
    margin: 2rem auto;
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
    flex-wrap: wrap;
    text-align: center;
    min-height: 2rem;
    padding: 0.75rem 1rem;
  }
  .fixed-bpm-option {
    padding: 1rem;
    margin: 0.5rem;
    width: 40px;
    font-size: var(--font-normal);
    cursor: pointer;
    background-color: var(--theme-main);
    color: var(--theme-neutral-dark);
    border-radius: 999px;
  }
  .bpm-source {
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 10px;
    gap: 20px;
  }
  .place-holder {
    height: 2rem;
  }
</style>
