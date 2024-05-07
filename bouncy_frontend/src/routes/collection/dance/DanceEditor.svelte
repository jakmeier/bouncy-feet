<script>
  import { t } from '$lib/i18n.js';
  import Header from '$lib/components/ui/Header.svelte';
  import DanceAnimation from '../../DanceAnimation.svelte';
  import { getContext } from 'svelte';
  import Popup from '$lib/components/ui/Popup.svelte';
  import DanceEditorSteps from './DanceEditorSteps.svelte';
  import { goto } from '$app/navigation';

  /** @type {import("$lib/instructor/bouncy_instructor").StepInfo[]} */
  export let availableSteps;
  /** @type {import('$lib/instructor/bouncy_instructor').DanceBuilder} */
  export let danceBuilder;
  /** @type {string} */
  let danceName = danceBuilder.danceInfo().id;

  const danceSize = 250;

  const localCollection = getContext('localCollection');

  /** @type {import('svelte/store').Readable<import('$lib/instructor/bouncy_instructor').DanceInfo[]>} */
  const localDances = localCollection.dances;

  $: dancePreview = danceBuilder.danceInfo();
  /** @type {import('svelte/store').Writable<boolean>} */
  let pickNamePopupActive;
  /** @type {import('svelte/store').Writable<boolean>} */
  let savingMethodPopupActive;

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
      goto(`../${danceName}`, { replaceState: true });
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

<Header title={danceName} button="save" on:click={openSavePopup} />

<div class="page">
  <div
    class="dance"
    style="max-width: {danceSize}px; max-height: {danceSize}px"
  >
    {#if dancePreview.length() > 0}
      <DanceAnimation dance={dancePreview} />
    {/if}
  </div>

  <DanceEditorSteps {availableSteps} bind:danceBuilder />
</div>

<Popup bind:isOpen={savingMethodPopupActive} title="editor.saving-prompt">
  <button class="light" on:click={overwrite}
    >{$t('editor.overwrite-existing-button')}</button
  >
  <button class="light" on:click={promptToPickName}
    >{$t('editor.save-copy-button')}</button
  >
</Popup>

<Popup
  bind:isOpen={pickNamePopupActive}
  title="editor.new-dance.pick-name-prompt"
>
  <label for="name">
    {$t('editor.new-dance.name-label')}
  </label>
  <input id="name" name="name" bind:value={danceName} />
  <button class="light" on:click={save}
    >{$t('editor.new-dance.save-button-label')}</button
  >
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
</style>
