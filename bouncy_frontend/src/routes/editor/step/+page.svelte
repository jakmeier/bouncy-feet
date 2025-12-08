<script>
  import { t } from '$lib/i18n';
  import { counter } from '$lib/timer';
  import { getContext } from 'svelte';
  import Step from '../../collection/Step.svelte';
  import { goto } from '$app/navigation';
  import {
    StepFileWrapper,
    StepWrapper,
  } from '$lib/instructor/bouncy_instructor';
  import EditOrDeleteList from '$lib/components/ui/EditOrDeleteList.svelte';
  import Popup from '$lib/components/ui/Popup.svelte';
  import { writable } from 'svelte/store';
  import Button from '$lib/components/ui/Button.svelte';
  import { downloadTextFile } from '$lib/text_utils';
  import LogoHeader from '$lib/components/ui/LogoHeader.svelte';

  const localCollectionCtx = getContext('localCollection');

  /** @type {import("svelte/store").Readable<StepWrapper[]>} */
  const steps = localCollectionCtx.steps;
  const builder = localCollectionCtx.stepBuilder;

  const stepTime = 300;
  const animationTime = stepTime * 0.7;
  const i = counter(-1, 1, stepTime);
  let showSettings = $state(writable(false));

  /** @param {number} index */
  function editStep(index) {
    const stepId = $steps[index].id;
    goto(`./${stepId}`);
  }

  /** @param {number} index */
  function deleteConfirmed(index) {
    const stepId = $steps[index].id;

    try {
      localCollectionCtx.removeStep(stepId);
    } catch (error) {
      console.warn('deleting step failed', stepId, error);
      alert($t('editor.step.delete-failed'));
    }
  }

  function openSettings() {
    $showSettings = true;
  }

  function handleFileUpload(event) {
    const file = event.target.files[0];
    if (file) {
      const reader = new FileReader();
      reader.onload = (e) => {
        const ronString = e.target.result;
        importFile(ronString);
      };
      reader.readAsText(file);
    }
  }

  /** @param {string} ronString */
  function importFile(ronString) {
    try {
      const file = StepFileWrapper.fromRon(ronString);
      for (let step of file.steps()) {
        localCollectionCtx.addStep(step);
      }
    } catch (e) {
      alert(e);
    }
  }

  function exportFile() {
    const string = $builder.buildPrettyRon();
    downloadTextFile('my_steps.steps.ron', string);
  }
</script>

<LogoHeader
  title={$t('editor.step.title')}
  button="menu"
  onAction={openSettings}
  backButton
/>

<div class="centered">
  <a href="./new">
    <button class="big wide"> {$t('editor.step.new')} </button>
  </a>
</div>

<h2 class="box">{$t('editor.step.list')}</h2>

<EditOrDeleteList items={$steps} onDelete={deleteConfirmed} onEdit={editStep}>
  {#snippet itemWrapper({ item, index, selected })}
    <div class="list-item">
      <div class:bold={selected}>{item.name}</div>
      <Step
        step={item}
        poseIndex={$i}
        {animationTime}
        borderWidth={selected ? 4 : 2}
      />
    </div>
  {/snippet}

  {#snippet confirmDeleteText({ item })}
    <div>
      <p>
        {$t('editor.step.delete-confirmation0')}
        "{item.name}"
        {$t('editor.step.delete-confirmation1')}
      </p>
    </div>
  {/snippet}
</EditOrDeleteList>

<Popup title={'editor.settings'} bind:isOpen={showSettings} showOkButton>
  <Button
    class="wide"
    symbol="upgrade"
    text={'editor.step.export-all'}
    on:click={exportFile}
  />
  <Button
    class="wide"
    symbol="system_update_alt"
    text={'editor.step.import'}
    on:click={() => document.querySelector('input#ron-upload')?.click()}
  ></Button>
</Popup>

<div class="hidden">
  <input
    id="ron-upload"
    type="file"
    accept=".ron"
    onchange={handleFileUpload}
  />
</div>

<style>
  .centered {
    margin: 30px auto;
    text-align: center;
  }

  .bold {
    font-weight: 800;
    text-shadow: gray 0px 1px 2px;
  }

  .hidden {
    display: None;
  }

  .list-item {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    height: 100%;
  }
</style>
