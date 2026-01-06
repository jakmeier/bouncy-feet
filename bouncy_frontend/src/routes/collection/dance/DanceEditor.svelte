<script>
  import { run } from 'svelte/legacy';

  import { t } from '$lib/i18n.js';
  import DanceAnimation from '../../DanceAnimation.svelte';
  import { getContext } from 'svelte';
  import Popup from '$lib/components/ui/Popup.svelte';
  import DanceEditorSteps from './DanceEditorSteps.svelte';
  import { goto } from '$app/navigation';
  import { dynamicCounter } from '$lib/timer';
  import LogoHeader from '$lib/components/ui/header/LogoHeader.svelte';

  /**
   * @typedef {Object} Props
   * @property {import("bouncy_instructor").StepWrapper[]} availableSteps
   * @property {import('bouncy_instructor').DanceBuilder} danceBuilder
   */

  /** @type {Props} */
  let { availableSteps, danceBuilder = $bindable() } = $props();
  /** @type {string} */
  let danceName = $state(danceBuilder.danceInfo().id);

  let bpm = $state(100);
  // step time is a half-beat
  let stepTime = $derived(30_000 / bpm);
  let animationTime = $derived(stepTime * 0.85);

  const beatCounter = dynamicCounter(-1, 1, stepTime);
  run(() => {
    beatCounter.setDelay(stepTime);
  });

  const danceSize = 250;

  const localCollection = getContext('localCollection');

  /** @type {import('svelte/store').Readable<import('bouncy_instructor').DanceWrapper[]>} */
  const localDances = localCollection.dances;

  let dancePreview = $derived(danceBuilder.danceInfo());
  /** @type {import('svelte/store').Writable<boolean>} */
  let pickNamePopupActive = $state();
  /** @type {import('svelte/store').Writable<boolean>} */
  let savingMethodPopupActive = $state();

  function openSavePopup() {
    const exists = $localDances.find((dance) => dance.id === danceName);
    if (exists) {
      promptToPickSavingMethod();
    } else {
      promptToPickName();
    }
  }

  function promptToPickSavingMethod() {
    $savingMethodPopupActive = true;
    $pickNamePopupActive = false;
  }

  function promptToPickName() {
    $savingMethodPopupActive = false;
    $pickNamePopupActive = true;
  }

  function save() {
    try {
      danceBuilder.setId(danceName);
      $pickNamePopupActive = false;
      localCollection.addDanceBuilder(danceBuilder);
      goto(`/collection/dance/${danceName}`, { replaceState: true });
    } catch (e) {
      // TODO: display error to user
      // possible cause: name already exists
      console.error(e);
    }
  }

  function overwrite() {
    try {
      danceBuilder.setId(danceName);
      $savingMethodPopupActive = false;
      localCollection.overwriteDanceBuilder(danceBuilder);
      window.history.back();
    } catch (e) {
      // TODO: display error to user
      // possible cause: name does not exist
      console.error(e);
    }
  }
</script>

<LogoHeader
  title={danceName}
  button="save"
  onAction={openSavePopup}
  backButton
/>

<div class="page">
  <div
    class="dance"
    style="max-width: {danceSize}px; max-height: {danceSize}px"
  >
    {#if dancePreview.length() > 0}
      <DanceAnimation dance={dancePreview} {animationTime} beat={beatCounter} />
    {/if}
  </div>

  <label class="config">
    <div>{$t('editor.speed')}</div>
    <input type="range" bind:value={bpm} min="15" max="200" class="range" />
  </label>

  <DanceEditorSteps
    {availableSteps}
    {animationTime}
    {stepTime}
    {beatCounter}
    bind:danceBuilder
  />
</div>

<Popup bind:isOpen={savingMethodPopupActive} title="editor.saving-prompt">
  <button onclick={overwrite}>{$t('editor.overwrite-existing-button')}</button>
  <button onclick={promptToPickName}>{$t('editor.save-copy-button')}</button>
</Popup>

<Popup
  bind:isOpen={pickNamePopupActive}
  title="editor.new-dance.pick-name-prompt"
>
  <label for="name">
    {$t('editor.new-dance.name-label')}
  </label>
  <input id="name" name="name" bind:value={danceName} />
  <button onclick={save}>{$t('editor.new-dance.save-button-label')}</button>
</Popup>

<style>
  .page {
    overflow: hidden;
  }
  .dance {
    margin: auto;
  }
  button {
    width: 50%;
  }
  .config {
    display: grid;
    grid-template-columns: auto;
    /* margin: auto; */
    padding: 10px;
  }
  .config input {
    font-size: unset;
  }
</style>
