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
  import { getContext } from 'svelte';
  import Symbol from '$lib/components/ui/Symbol.svelte';
  import Button from '$lib/components/ui/Button.svelte';

  /** @type {import('./$types').PageData} */
  export let data;

  const user = getContext('user').store;
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

  let thickness = 10;
  let headRadiusParam = 1;
  let headHeightParam = 1;
  let rightColor = '#a9eb66';
  let leftColor = '#4c6633';
  let headColor = '#4c6633';
</script>

<Header title={name} />

<Step
  step={selected.value}
  poseIndex={$beatCounter}
  {animationTime}
  size={200}
  rotation={degree}
  lineWidth={thickness}
  headRadius={100 * 0.1 * headRadiusParam}
  headHeight={headHeightParam}
  style={{
    leftColor,
    rightColor,
    headColor,
    bodyColor: 'var(--theme-neutral-light)',
  }}
/>

{#if $user.experimentalFeatures}
  <div class="input-group">
    <label>
      Thickness
      <input
        type="number"
        bind:value={thickness}
        min="0.1"
        max="50"
        step="0.1"
        class="number"
      />
      <input
        type="range"
        bind:value={thickness}
        min="0.1"
        max="50"
        step="0.1"
        class="range"
      />
    </label>

    <label>
      Head Size
      <input
        type="number"
        bind:value={headRadiusParam}
        min="0.1"
        max="5"
        step="0.1"
        class="number"
      />
      <input
        type="range"
        bind:value={headRadiusParam}
        min="0.1"
        max="5"
        step="0.1"
        class="range"
      />
    </label>

    <label>
      Head Height
      <input
        type="number"
        bind:value={headHeightParam}
        min="0.1"
        max="3"
        step="0.1"
        class="number"
      />
      <input
        type="range"
        bind:value={headHeightParam}
        min="0.1"
        max="3"
        step="0.1"
        class="range"
      />
    </label>
  </div>

  <div class="input-group">
    <label>
      Left Color
      <input type="text" bind:value={leftColor} />
      <input type="color" bind:value={leftColor} />
    </label>

    <label>
      Right Color
      <input type="text" bind:value={rightColor} />
      <input type="color" bind:value={rightColor} />
    </label>

    <label>
      Head Color
      <input type="text" bind:value={headColor} />
      <input type="color" bind:value={headColor} />
    </label>
  </div>
{/if}

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

{#if !$features.enableStepRecording(name)}
  <div class="note">
    <Symbol>info</Symbol>
    {$t('step.wip-tracking')}
  </div>
{/if}
{#if $features.enableStepRecording(name) || $user.experimentalFeatures || !browser}
  <div class="label buttons">
    <a href="./learn">
      <Button class="light" symbol="school" text="record.learn-button"></Button>
    </a>
    <a href="./train">
      <Button class="light" symbol="exercise" text="record.train-button"
      ></Button>
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
  .note {
    display: grid;
    max-width: 300px;
    margin: 10px auto;
    justify-items: space-around;
    font-style: italic;
    grid-template-columns: auto auto;
  }
  input[type='color'] {
    width: 100%;
    margin: 15px;
  }
  .input-group {
    display: grid;
    gap: 10px;
    grid-template-columns: 1fr 1fr 1fr;
  }
</style>
