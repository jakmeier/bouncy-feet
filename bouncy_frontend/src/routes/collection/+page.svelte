<script>
  import { t } from '$lib/i18n.js';
  import { counter } from '$lib/timer';
  import { getContext } from 'svelte';
  import DanceAnimation from '../DanceAnimation.svelte';
  import Area from '$lib/components/ui/Area.svelte';
  import Step from './Step.svelte';
  import { features } from '$lib/stores/FeatureSelection';
  import { browser } from '$app/environment';
  import Experience from '$lib/components/Experience.svelte';

  /** @type {import('./$types').PageData} */
  export let data;

  const localCollection = getContext('localCollection');
  const localDances = localCollection.dances;
  const user = getContext('user').store;

  const uniqueNameSteps = data.lookupSteps({ uniqueNames: true });

  const stepTime = 300;
  // animationTime < stepTime will freeze the position for a moment, which makes
  // it clearer. If the difference is too much, it looks robotic.
  const animationTime = stepTime * 0.7;

  const i = counter(-1, 1, stepTime);
  const danceSize = '150px';
  const borderRadius = '25px';
</script>

<h1 class="colored">{$t('collection.title')}</h1>

{#if !browser || $features.enableDanceCollection}
  <h2 class="box">{$t('collection.dances-subtitle')}</h2>
  <div class="dance-table">
    {#each [...data.officialDances, ...$localDances] as dance}
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
    {#if $features.enableDanceCreator}
      <div>
        <a href={`./dance/new`} class="no-line-link">
          <Area width={danceSize} height={danceSize} {borderRadius}>
            <span class="material-symbols-outlined add-button">
              add_circle
            </span>
            {$t('collection.new-dance-button')}
          </Area>
        </a>
      </div>
    {/if}
  </div>
{/if}

<h2 class="box">{$t('collection.steps-subtitle')}</h2>
<div class="step-table">
  {#each uniqueNameSteps as step}
    {#if !step.name.includes('Idle')}
      <div class="step">
        <a href={`./step/${step.name}`}>
          <Step {step} poseIndex={$i} {animationTime} />
          <!-- TODO: translations -->
          <h3>{step.name}</h3>
        </a>
        <Experience
          xp={$user.userSteps[step.name]
            ? $user.userSteps[step.name].experience
            : 0}
        ></Experience>
      </div>
    {/if}
  {/each}
</div>

<style>
  h1 {
    margin: 5px 0;
  }
  h3 {
    margin: 5px;
  }
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
  .step {
    display: grid;
    align-content: space-between;
  }

  .add-button {
    font-size: 100px;
  }
  .no-line-link {
    text-decoration: none;
  }
</style>
