<script>
  import Header from '$lib/components/ui/Header.svelte';
  import { t } from '$lib/i18n';
  import { counter } from '$lib/timer';
  import { getContext } from 'svelte';
  import Step from '../../collection/Step.svelte';

  const localCollectionCtx = getContext('localCollection');

  /** @type {import("svelte/store").Readable<StepWrapper[]>} */
  const steps = localCollectionCtx.steps;

  const stepTime = 300;
  const animationTime = stepTime * 0.7;
  const i = counter(-1, 1, stepTime);
</script>

<Header title={$t('editor.step.title')}></Header>

<div class="centered">
  <a href="./new">
    <button class="light big wide"> {$t('editor.step.new')} </button>
  </a>
</div>

<h2 class="box">{$t('editor.step.list')}</h2>

<div class="steps">
  {#each $steps as step}
    <div class="step">
      <Step {step} poseIndex={$i} {animationTime} />
    </div>
  {/each}
</div>

<style>
  .steps {
    display: flex;
    flex-wrap: wrap;
  }
  .step {
    max-width: 200px;
  }
  .centered {
    margin: 30px auto;
    text-align: center;
  }
</style>
