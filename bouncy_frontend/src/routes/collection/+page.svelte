<script>
  import { t } from '$lib/i18n.js';
  import { counter } from '$lib/timer';
  import { getContext } from 'svelte';
  import DanceAnimation from '../DanceAnimation.svelte';
  import Area from '$lib/components/ui/Area.svelte';
  import Step from './Step.svelte';
  import { features } from '$lib/stores/FeatureSelection';
  import { browser, dev } from '$app/environment';
  import Experience from '$lib/components/Experience.svelte';
  import { getUserContext } from '$lib/context';

  /**
   * @typedef {Object} Props
   * @property {import('./$types').PageData} data
   */

  /** @type {Props} */
  let { data } = $props();

  const localCollection = getContext('localCollection');
  const localDances = localCollection.dances;
  const user = getUserContext().store;

  const stepTime = 300;
  // animationTime < stepTime will freeze the position for a moment, which makes
  // it clearer. If the difference is too much, it looks robotic.
  const animationTime = stepTime * 0.7;

  const i = counter(-1, 1, stepTime);
  const danceSize = '150px';
  const borderRadius = '25px';
</script>

<h1 class="colored">{$t('collection.title')}</h1>

<h2 class="box"><b>{$t('collection.steps-subtitle')}</b></h2>
<h2 class="box">
  <div class="gradient-text">{$t('collection.basic-steps-subtitle')}</div>
</h2>
<div class="step-table">
  {#each data.lookupSteps({ uniqueNames: true, sources: ['basic'] }) as step}
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
  {/each}
</div>

<h2 class="box">
  <div class="gradient-text">
    {$t('collection.shapes-steps-subtitle')}
    <div></div>
  </div>
</h2>
<div class="step-table">
  {#each data.lookupSteps({ uniqueNames: true, sources: ['shapes'] }) as step}
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
  {/each}
</div>

<h2 class="box">
  <div class="gradient-text">{$t('collection.footwork-steps-subtitle')}</div>
</h2>

<div class="step-table">
  {#each data.lookupSteps({ uniqueNames: true, sources: ['footwork'] }) as step}
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
  {/each}
</div>

<h2 class="box">
  <div class="gradient-text">{$t('collection.rm-steps-subtitle')}</div>
</h2>
<div class="step-table">
  {#each data.lookupSteps( { uniqueNames: true, sources: ['rm_variations'] } ) as step}
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
  {/each}
</div>

{#if dev}
  <h2 class="box"><div class="gradient-text">DEV STEPS</div></h2>
  <div class="step-table">
    {#each data.lookupSteps({ uniqueNames: true, sources: ['misc'] }) as step}
      <div class="step">
        <a href={`./step/${step.name}`}>
          <Step {step} poseIndex={$i} {animationTime} />
          <h3>{step.name}</h3>
        </a>
        <Experience
          xp={$user.userSteps[step.name]
            ? $user.userSteps[step.name].experience
            : 0}
        ></Experience>
      </div>
    {/each}
  </div>
{/if}

{#if !browser || $features.enableDanceCollection}
  <h2 class="box"><div><b>{$t('collection.dances-subtitle')}</b></div></h2>
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
            <span class="material-symbols-outlined add-button" translate="no">
              add_circle
            </span>
            {$t('collection.new-dance-button')}
          </Area>
        </a>
      </div>
    {/if}
  </div>
{/if}

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
