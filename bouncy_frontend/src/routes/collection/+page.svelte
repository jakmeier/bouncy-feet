<script>
  import { t } from '$lib/i18n.js';
  import { counter } from '$lib/timer';
  import DanceAnimation from '../DanceAnimation.svelte';
  import Area from '../record/Area.svelte';
  import Step from './Step.svelte';

  /** @type {import('./$types').PageData} */
  export let data;

  const stepTime = 300;
  // animationTime < stepTime will freeze zhe position for a moment, which makes
  // it clearer. If the difference is too much, it looks robotic.
  const animationTime = stepTime * 0.85;

  const i = counter(-1, 1, stepTime);
  const danceSize = '150px';
  const borderRadius = '25px';
</script>

<h1>{$t('collection.title')}</h1>

<h2>{$t('collection.dances-subtitle')}</h2>
<div class="dance-table">
  {#each data.allDances as dance}
    <div>
      <a href={`./dance/${dance.id}`}>
        <Area width={danceSize} height={danceSize} {borderRadius}>
          <DanceAnimation {dance} />
        </Area>
        <!-- TODO: id to translated name -->
        <h3>{dance.id}</h3>
      </a>
    </div>
  {/each}
  <div>
    <Area width={danceSize} height={danceSize} {borderRadius}>
      <span class="material-symbols-outlined add-button"> add_circle </span>
      {$t('collection.new-dance-button')}
    </Area>
  </div>
</div>

<h2>{$t('collection.steps-subtitle')}</h2>
<div class="step-table">
  {#each data.uniqueNameSteps as step}
    {#if !step.name.includes('Idle')}
      <a href={`./step/${step.name}`}>
        <Step {step} poseIndex={$i} {animationTime} />
        <!-- TODO: translations -->
        <h3>{step.name}</h3>
      </a>
    {/if}
  {/each}
</div>

<style>
  .step-table,
  .dance-table {
    display: grid;
    text-align: center;
    gap: 20px 0px;
  }
  .step-table {
    grid-template-columns: 1fr 1fr 1fr;
  }
  .dance-table {
    grid-template-columns: 1fr 1fr;
  }

  .add-button {
    font-size: 100px;
  }
</style>
