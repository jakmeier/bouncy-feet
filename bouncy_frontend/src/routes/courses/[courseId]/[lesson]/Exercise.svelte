<script>
  import Audio from '$lib/components/BeatAudio.svelte';
  import Popup from '$lib/components/ui/Popup.svelte';
  import Toggle from '$lib/components/ui/Toggle.svelte';
  import { t } from '$lib/i18n';
  import { counter } from '$lib/timer';
  import { writable } from 'svelte/store';
  import Step from '../../../collection/Step.svelte';
  import Symbol from '$lib/components/ui/Symbol.svelte';

  
  /**
   * @typedef {Object} Props
   * @property {import("$lib/instructor/bouncy_instructor").LessonPart} lessonPart
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let { lessonPart, children } = $props();

  let outerWidth = $state(300);
  /** @type {boolean} */
  let audioOn = $state();
  /** @type {import('svelte/store').Writable<boolean>} */
  let popUpIsOpen = writable(true);

  let step = $derived(lessonPart.step);
  let bpmIndex = $state(0);
  let bpm = $derived(lessonPart.bpms[bpmIndex]);
  let stepTime = $derived(30_000 / bpm);
  let animationTime = $derived(Math.min(stepTime * 0.7, 300));
  let i = $derived(counter(-1, 1, stepTime));

  let size = $derived(Math.max(outerWidth, 100));

  /** @param {number} index */
  function selectBpm(index) {
    bpmIndex = index;
  }

  /**
   * @param {KeyboardEvent} event
   * @param {number} index
   */
  function selectBpmWithKeyboard(event, index) {
    if (event.key === 'Enter' || event.key === ' ') {
      selectBpm(index);
    }
  }

  /** @param {number} index */
  function speedIcon(index) {
    switch (index) {
      case 0:
        return '🐢';
      case 1:
        return '🏃';
      case 2:
        return '🚀';
      default:
        return '⚡';
    }
  }

  function closePopUp() {
    $popUpIsOpen = false;
  }
</script>

<div bind:clientWidth={outerWidth}>
  <Step {step} poseIndex={$i} {animationTime} {size}></Step>
  <div class="controls">
    <div class="label">{$t('courses.lesson.bpm-label')}</div>
    <div
      class="bpm-selector"
      style="grid-template-columns: repeat({lessonPart.bpms.length}, 1fr);"
    >
      {#each lessonPart.bpms as bpm, index}
        <div
          class="bpm-option
          {bpmIndex === index ? 'selected' : ''}
          "
          onclick={() => selectBpm(index)}
          onkeydown={(event) => selectBpmWithKeyboard(event, index)}
          title={`BPM: ${bpm}`}
          role="radio"
          aria-checked={bpmIndex === index}
          tabindex="0"
        >
          {speedIcon(index)}
        </div>
      {/each}
    </div>
    <div class="label">{$t('courses.lesson.audio-label')}</div>
    <div class="audio-selector">
      <Toggle bind:isOn={audioOn} border></Toggle>
      {#if audioOn}
        <Symbol class="blue margin15" size={40}>volume_up</Symbol>
      {:else}
        <Symbol class="blue margin15" size={40}>volume_off</Symbol>
      {/if}
    </div>
  </div>

  <Audio {bpm} bind:isOn={audioOn}></Audio>

  <Popup isOpen={popUpIsOpen} title={$t('courses.lesson.exercise-popup-title')}>
    <div>
      {$t('courses.lesson.exercise-start-description')}
    </div>
    <button onclick={closePopUp}
      >{$t('courses.lesson.own-music-button')}</button
    >
    <button
      onclick={() => {
        audioOn = true;
        closePopUp();
      }}>{$t('courses.lesson.play-beat-button')}</button
    >
    {@render children?.()}
  </Popup>
</div>

<style>
  .controls {
    display: grid;
    grid-template-columns: 1fr 2fr;
    margin: 10px auto;
    max-width: 300px;
    background-color: var(--theme-neutral-light);
    border-radius: 10px;
    padding: 10px;
  }

  .label {
    line-height: 60px;
  }

  .bpm-selector,
  .audio-selector {
    height: 50px;
    margin-top: 10px;
    display: flex;
    align-items: center;
    justify-content: flex-end;
  }

  .bpm-option {
    font-size: var(--font-large);
    margin: 0 10px;
    cursor: pointer;
    transition: transform 0.2s ease-in-out;
    opacity: 0.6;
    border-radius: 50%;
    padding: 5px;
    background-color: var(--theme-main-light);
    border: solid 1px var(--theme-main);
    transform: scale(0.8);
  }

  .bpm-option.selected {
    transform: scale(1);
    opacity: 1;
    background-color: var(--theme-main);
  }

  .bpm-option:not(.selected):hover {
    transform: scale(1.2);
  }
</style>
