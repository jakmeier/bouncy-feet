<script>
  import { page } from '$app/stores';
  import { readable } from 'svelte/store';
  import { t } from '$lib/i18n.js';
  import Step from '../Step.svelte';
  import BackButton from '$lib/BackButton.svelte';
  import Select from 'svelte-select';

  /** @type {import('./$types').PageData} */
  export let data;

  const name = $page.params.slug;
  const variations = data.allSteps.filter((step) => step.name === name);
  const selectItems = variations.map((step) => {
    return { value: step, label: $t(`step.variation.${step.variation}`) };
  });

  let bpm = 240;
  $: stepTime = 60_000 / bpm;
  $: animationTime = stepTime * 0.85;

  // A chain of `setTimeout` calls which uses live values of `stepTime` for the
  // delay. The `fuse` variable holds the handle of the currently outstanding
  // `setTimeout` call, which is canceled when the store is destroyed.
  let fuse;
  const a = readable({ i: 0, bpm }, (set) => {
    const foo = () =>
      setTimeout(() => {
        set({ i: $a.i + 1, bpm });
        foo();
      }, stepTime);
    fuse = foo();
    return () => clearTimeout(fuse);
  });

  let selected = selectItems[0];
</script>

<BackButton />

<h1>{name}</h1>

<Step step={selected.value} poseIndex={$a.i} {animationTime} size={200} />

<label>
  {$t('learn.step.speed')}
  <input type="number" bind:value={bpm} min="30" max="300" class="number" />
  <input type="range" bind:value={bpm} min="30" max="300" class="range" />
</label>

{#if selectItems.length > 1}
  <div class="label">
    {$t('learn.step.variation')}
    <Select
      bind:value={selected}
      items={selectItems}
      showChevron={true}
      clearable={false}
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

<style>
  h1 {
    text-align: center;
  }
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
</style>
