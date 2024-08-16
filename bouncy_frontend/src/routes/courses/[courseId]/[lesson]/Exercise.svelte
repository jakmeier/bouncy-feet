<script>
  import Audio from '$lib/components/Audio.svelte';
  import Popup from '$lib/components/ui/Popup.svelte';
  import Toggle from '$lib/components/ui/Toggle.svelte';
  import { t } from '$lib/i18n';
  import { counter } from '$lib/timer';
  import { writable } from 'svelte/store';
  import Step from '../../../collection/Step.svelte';

  /** @type {import("$lib/instructor/bouncy_instructor").LessonPart} */
  export let lessonPart;

  let outerWidth = 300;
  /** @type {boolean} */
  let audioOn;
  /** @type {import('svelte/store').Writable<boolean>} */
  let popUpIsOpen = writable(true);

  $: step = lessonPart.step;
  let bpmIndex = 0;
  $: bpm = lessonPart.bpms[bpmIndex];
  $: stepTime = 30_000 / bpm;
  $: animationTime = Math.min(stepTime * 0.7, 300);
  $: i = counter(-1, 1, stepTime);

  $: size = Math.max(outerWidth, 100);

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
          on:click={() => selectBpm(index)}
          on:keydown={(event) => selectBpmWithKeyboard(event, index)}
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
        <span class="material-symbols-outlined"> volume_up </span>
      {:else}
        <span class="material-symbols-outlined"> volume_off </span>
      {/if}
    </div>
  </div>

  <Audio {bpm} bind:isOn={audioOn}></Audio>

  <Popup isOpen={popUpIsOpen} title={$t('courses.lesson.exercise-popup-title')}>
    <div>
      {$t('courses.lesson.exercise-start-description')}
    </div>
    <button class="light" on:click={closePopUp}
      >{$t('courses.lesson.own-music-button')}</button
    >
    <button
      class="light"
      on:click={() => {
        audioOn = true;
        closePopUp();
      }}>{$t('courses.lesson.play-beat-button')}</button
    >
    <slot />
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
    font-size: 35px;
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

  .audio-selector span {
    color: var(--theme-main);
    font-size: 40px;
    margin: 15px;
  }
</style>
