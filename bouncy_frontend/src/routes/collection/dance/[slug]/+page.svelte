<script>
  import { page } from '$app/stores';
  import { t } from '$lib/i18n.js';
  import Header from '$lib/components/ui/Header.svelte';
  import DanceAnimation from '../../../DanceAnimation.svelte';
  import Step from '../../Step.svelte';
  import { counter } from '$lib/timer';
  import { getContext } from 'svelte';
  import Popup from '$lib/components/ui/Popup.svelte';
  import { goto } from '$app/navigation';
  import { derived } from 'svelte/store';

  /** @type {import('./$types').PageData} */
  export let data;

  const bpm = 240;
  const stepTime = 60_000 / bpm;
  const animationTime = stepTime * 0.85;
  const danceSize = 250;
  const stepSize = 100;

  const localCollection = getContext('localCollection');
  const localDances = localCollection.dances;

  const id = derived(page, ($page) => $page.params.slug);
  const isSelected = (
    /** @type {import('$lib/instructor/bouncy_instructor').DanceInfo} */ dance
  ) => dance.id === $id;
  $: dance =
    data.officialDances.find(isSelected) || $localDances.find(isSelected);
  $: steps = dance?.steps() || [];
  $: isStatic = data.officialDances.find(isSelected) !== undefined;
  const beatCounter = counter(-1, 1, stepTime);

  /** @type {import('svelte/store').Writable<boolean>} */
  let optionsPopupActive;

  function edit() {
    $optionsPopupActive = false;
    goto('./edit');
  }

  function maybeDelete() {
    if (confirm($t('editor.delete-dance-confirmation'))) {
      localCollection.removeDance($id);
      $optionsPopupActive = false;
      window.history.back();
    }
  }
</script>

<!-- TODO: translate -->
<Header
  title={$id}
  button="edit"
  on:click={() => optionsPopupActive.set(true)}
/>

<div class="page">
  <div
    class="dance"
    style="max-width: {danceSize}px; max-height: {danceSize}px"
  >
    {#if dance}
      <DanceAnimation {dance} />
    {/if}
  </div>

  <div class="steps-container">
    {#each steps as step}
      <div class="step">
        <a href={`../../step/${step.name}`}>
          <Step
            {step}
            poseIndex={$beatCounter}
            {animationTime}
            size={stepSize}
          />
          <p style="width: {stepSize}px">{step.name}</p>
        </a>
      </div>
    {/each}
  </div>
</div>

<Popup bind:isOpen={optionsPopupActive} title="editor.edit-dance-context-menu">
  <button class="light" on:click={edit}>{$t('editor.edit-button')}</button>
  <!-- <button class="light" on:click={copyAndEdit}
    >{$t('editor.edit-copy-button')}</button
  > -->
  {#if !isStatic}
    <button class="danger" on:click={maybeDelete}
      >{$t('editor.delete-dance-button')}</button
    >
  {/if}
</Popup>

<style>
  .page {
    overflow: hidden;
  }
  .dance {
    margin: auto;
  }
  .steps-container {
    display: flex;
    overflow: auto hidden;
  }
  .step {
    margin: 2px;
  }
  .step p {
    background-color: var(--theme-neutral-light);
    border-radius: 5px;
    text-align: center;
    margin: 0;
    padding: 2px;
  }
  .step a {
    font-weight: 400;
  }
</style>
