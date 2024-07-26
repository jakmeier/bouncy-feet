<script>
  import { page } from '$app/stores';
  import { t } from '$lib/i18n.js';
  import Step from '../../Step.svelte';
  import Header from '$lib/components/ui/Header.svelte';
  import Select from 'svelte-select';
  import { dynamicCounter } from '$lib/timer';
  import { features } from '$lib/stores/FeatureSelection';
  import { browser } from '$app/environment';
  import Info from '$lib/components/ui/Info.svelte';

  /** @type {import('./$types').PageData} */
  export let data;

  const name = $page.params.stepName;
  const variations = data.lookupSteps({
    uniqueNames: false,
    stepName: name,
  });
  const selectItems = variations.map((step) => {
    return { value: step, label: $t(`step.variation.${step.variation}`) };
  });

  let degree = 0;
  let bpm = 120;
  // step time is a half-beat
  $: stepTime = 30_000 / bpm;
  $: animationTime = stepTime * 0.85;

  const beatCounter = dynamicCounter(-1, 1, stepTime);
  $: beatCounter.setDelay(stepTime);

  let selected = selectItems[0];
</script>

<Header title={name} />

<Step
  step={selected.value}
  poseIndex={$beatCounter}
  {animationTime}
  size={200}
  rotation={degree}
/>

<label>
  {$t('collection.step.speed')}
  <input type="number" bind:value={bpm} min="15" max="200" class="number" />
  <input type="range" bind:value={bpm} min="15" max="200" class="range" />
</label>

{#if $features.enableAvatarRotation}
  <label>
    {$t('collection.step.rotation')}
    <input
      type="number"
      bind:value={degree}
      min="-180"
      max="180"
      class="number"
    />
    <input
      type="range"
      bind:value={degree}
      min="-180"
      max="180"
      class="range"
    />
  </label>
{/if}

{#if selectItems.length > 1}
  <div class="label">
    {$t('collection.step.variation')}
    <Select
      bind:value={selected}
      items={selectItems}
      showChevron={true}
      clearable={false}
      searchable={false}
      --background="var(--theme-neutral-light)"
      --selected-item-color="var(--theme-neutral-dark)"
      --item-hover-bg="var(--theme-main)"
      --item-hover-color="var(--theme-neutral-light)"
      --item-active-background="var(--theme-accent)"
      --item-is-active-bg="var(--theme-neutral-white)"
      --item-is-active-color="var(--theme-neutral-dark)"
      --border="1px solid var(--theme-neutral-dark)"
      --border-hover="1.5px solid var(--theme-main)"
      --border-focused="1.5px solid var(--theme-main)"
      --margin="10px auto"
      --padding="10px"
      --font-size="20px"
    />
  </div>
{/if}

{#if $features.enableStepRecording || !browser}
  <div class="label buttons">
    <a href="./learn">
      <button class="light">
        <span class="material-symbols-outlined"> school </span>
        <p>{$t('record.learn-button')}</p>
      </button>
    </a>
    <a href="./train">
      <button class="light">
        <span class="material-symbols-outlined"> exercise </span>
        <p>{$t('record.train-button')}</p>
      </button>
    </a>
    <Info title="record.learn-button" text="record.info.learn" />
    <Info title="record.train-button" text="record.info.train" />
  </div>
{/if}

<style>
  label,
  .label {
    display: grid;
    justify-items: center;
    margin: 10px auto;
    max-width: 300px;
    background-color: var(--theme-neutral-light);
    border-radius: 10px;
    padding: 10px;
  }
  .buttons {
    grid-template-columns: auto auto;
    row-gap: 10px;
  }
  button {
    width: 120px;
  }
</style>
