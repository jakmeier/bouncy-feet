<script>
  import { run } from 'svelte/legacy';

  import { page } from '$app/state';
  import { t } from '$lib/i18n.js';
  import Step from '../../Step.svelte';
  import Select from 'svelte-select';
  import { dynamicCounter } from '$lib/timer';
  import { features } from '$lib/stores/FeatureSelection';
  import { browser } from '$app/environment';
  import Info from '$lib/components/ui/Info.svelte';
  import Symbol from '$lib/components/ui/Symbol.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import { getUserContext } from '$lib/context';
  import BackHeader from '$lib/components/ui/header/BackHeader.svelte';

  const user = getUserContext().store;
  const name = page.params.stepName;
  const variations = data.lookupSteps({
    uniqueNames: false,
    stepName: name,
  });
  const selectItems = variations.map((step) => {
    return { value: step, label: $t(`step.variation.${step.variation}`) };
  });

  let degree = $state(0);
  let bpm = $state(120);
  // step time is a half-beat
  let stepTime = $derived(30_000 / bpm);
  let animationTime = $derived(stepTime * 0.85);

  const beatCounter = dynamicCounter(-1, 1, stepTime);
  run(() => {
    beatCounter.setDelay(stepTime);
  });

  let selected = $state(selectItems[0]);
</script>

<BackHeader title={name} />

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
      --font-size="var(--font-normal)"
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
      <Button symbol="school" text="record.learn-button"></Button>
    </a>
    <a href="./train">
      <Button symbol="exercise" text="record.train-button"></Button>
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
