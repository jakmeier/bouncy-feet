<script>
  import { page } from '$app/stores';
  import { readable } from 'svelte/store';
  import { t } from '$lib/i18n.js';
  import Step from '../Step.svelte';
  import BackButton from '$lib/BackButton.svelte';

  /** @type {import('./$types').PageData} */
  export let data;

  const name = $page.params.slug;
  const variations = data.allSteps.filter((step) => step.name === name);

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
</script>

<BackButton />

<h1>{name}</h1>

{#if variations.length > 0}
  <Step step={variations[0]} poseIndex={$a.i} {animationTime} size={200} />
{/if}

<label>
  {$t('learn.step.speed')}
  <input type="number" bind:value={bpm} min="10" max="300" class="number" />
  <input type="range" bind:value={bpm} min="10" max="300" class="range" />
</label>

<!-- 
    TODO: render multiple variations, something like this but with information included what 
    is different about this variation, like left first. Translated, of course.
    {#each variations as step, i}
    <h3>{Variation 'I'.repeat(i + 1)}</h3>
    <Step {step} poseIndex={0} animationTime={300} />
{/each} 
-->

<style>
  h1 {
    text-align: center;
  }
  label {
    display: grid;
    justify-items: center;
    margin: auto;
    max-width: 300px;
    background-color: var(--theme-neutral-light);
    border-radius: 10px;
    padding: 10px;
  }
</style>
