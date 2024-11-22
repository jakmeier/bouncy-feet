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
  import DanceCounts from '$lib/components/DanceCounts.svelte';

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
    /** @type {import('$lib/instructor/bouncy_instructor').DanceWrapper} */ dance
  ) => dance.id === $id;
  $: dance =
    data.officialDances.find(isSelected) || $localDances.find(isSelected);
  $: steps = dance?.steps() || [];
  $: isStatic = data.officialDances.find(isSelected) !== undefined;
  const beatCounter = counter(-1, 1, stepTime);

  /** @type {import('svelte/store').Writable<boolean>} */
  let optionsPopupActive;
  let highlightedStep = -1;

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

  function highlight(index) {
    highlightedStep = index;
  }

  function resetHighlight() {
    highlightedStep = -1;
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

  <ol class="steps-container">
    {#each steps as step, index}
      <li class="step" class:highlighted={highlightedStep === index}>
        <a
          href={`../../step/${step.name}`}
          on:mouseover={() => highlight(index)}
          on:focus={() => highlight(index)}
          on:mouseout={resetHighlight}
          on:blur={resetHighlight}
          tabindex={0}
        >
          <Step
            {step}
            poseIndex={$beatCounter}
            {animationTime}
            size={stepSize}
          />
          <p style="width: {stepSize}px">{step.name}</p>
        </a>
      </li>
    {/each}
  </ol>

  <h2 class="box">{$t('dance.counts')}</h2>
  <DanceCounts {dance} bind:highlightedStep markedPoseIndex={$beatCounter} />
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
    padding: 5px;
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
  .step.highlighted {
    font-weight: 800;
    border-radius: 5px;
    background-color: var(--theme-accent-light);
  }
</style>
