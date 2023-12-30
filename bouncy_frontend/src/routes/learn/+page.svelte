<script>
  import { t } from '$lib/i18n.js';
  import Step from './Step.svelte';
  import { readable } from 'svelte/store';

  /** @type {import('./$types').PageData} */
  export let data;

  const stepTime = 300;
  // animationTime < stepTime will freeze zhe position for a moment, which makes
  // it clearer. If the difference is too much, it looks robotic.
  const animationTime = stepTime * 0.85;

  const i = readable(0, (set) => {
    const handle = setInterval(() => {
      set($i + 1);
    }, stepTime);

    return () => clearInterval(handle);
  });
</script>

<h1>{$t('learn.title')}</h1>

<h2>{$t('home.steps')}</h2>
<div class="step-table">
  {#each data.uniqueNameSteps as step}
    {#if !step.name.includes('Idle')}
      <a href={`./${step.name}`}>
        <Step {step} poseIndex={$i} {animationTime} />
        <!-- TODO: translations -->
        <h3>{step.name}</h3>
      </a>
    {/if}
  {/each}
</div>

<style>
  .step-table {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    text-align: center;
    gap: 20px 0px;
  }

  a {
    color: var(--theme-neutral-dark);
  }
</style>
