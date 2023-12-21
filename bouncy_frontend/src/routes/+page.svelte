<script>
  import { t } from '$lib/i18n.js';
  import { onMount } from 'svelte';
  import Step from './Step.svelte';
  import { readable } from 'svelte/store';

  /** @type {import('./$types').PageData} */
  export let data;

  const i = readable(0, (set) => {
    const handle = setInterval(() => {
      set($i + 1);
    }, 300);

    return () => clearInterval(handle);
  });
</script>

<h1>{$t('home.title')}</h1>
<p>[intro video]</p>
<p>[recommendations]</p>

<h2>{$t('home.steps')}</h2>
<div class="step-table">
  {#each data.allSteps as step}
    <Step {step} poseIndex={$i} />
  {/each}
</div>

<style>
  .step-table {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    text-align: center;
    gap: 20px 0px;
  }
</style>
