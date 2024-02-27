<script>
  import { t } from '$lib/i18n.js';
  import Header from '$lib/components/Header.svelte';
  import DanceAnimation from '../../../DanceAnimation.svelte';
  import Step from '../../Step.svelte';
  import { counter } from '$lib/timer';
  import { DanceBuilder, DanceInfo } from '$lib/instructor/bouncy_instructor';
  import Area from '$lib/components/Area.svelte';
  import SelectStep from './SelectStep.svelte';
  import { getContext } from 'svelte';

  /** @type {import('./$types').PageData} */
  export let data;

  const bpm = 240;
  const stepTime = 60_000 / bpm;
  const animationTime = stepTime * 0.85;
  const danceSize = 250;
  const stepSize = 100;
  const borderRadius = '25px';

  const beatCounter = counter(-1, 1, stepTime);
  const localCollection = getContext('localCollection');
  const dances = localCollection.dances;

  let id = $t('collection.new-dance.default-dance-name');
  /** @type {DanceBuilder} */
  let danceBuilder = new DanceBuilder('tempPreviewDance');
  $: dancePreview = danceBuilder.danceInfo();
  /**
   * @type {import('$lib/instructor/bouncy_instructor').StepInfo[]}
   */
  let steps = [];

  let stepSelectionActive = false;

  /** @param {import('$lib/instructor/bouncy_instructor').StepInfo} stepInfo */
  function selectedCallback(stepInfo) {
    if (!stepInfo) {
      return false;
    }

    steps.push(stepInfo);
    danceBuilder.addStep(stepInfo.id);
    // trigger reload in ui (can I do better?)
    danceBuilder = danceBuilder;

    return true;
  }

  function save() {
    // TODO: pick name
    localCollection.addDanceBuilder(danceBuilder);
  }
</script>

<Header title={id} button="save" on:click={save} />

<div class="page">
  <div
    class="dance"
    style="max-width: {danceSize}px; max-height: {danceSize}px"
  >
    {#if dancePreview.length() > 0}
      <DanceAnimation dance={dancePreview} />
    {/if}
  </div>

  <div class="steps-container">
    {#each steps as step}
      <div class="step">
        <a href={`../../step/${step.name}`}>
          <Step
            {step}
            poseIndex={$beatCounter}
            {animationTime}
            size={stepSize}
          />
          <p style="width: {stepSize}px">{step.name}</p>
        </a>
      </div>
    {/each}
  </div>
  <div on:click={() => (stepSelectionActive = !stepSelectionActive)}>
    <Area width="{stepSize}px" height="{stepSize}px" {borderRadius}>
      <span
        class="material-symbols-outlined add-button"
        style="font-size: {stepSize / 2}px"
      >
        add_circle
      </span>
    </Area>
  </div>

  <SelectStep
    bind:show={stepSelectionActive}
    {selectedCallback}
    steps={data.uniqueNameSteps}
  ></SelectStep>
</div>

<style>
  .page {
    overflow: hidden;
  }
  .dance {
    margin: auto;
  }
  .steps-container {
    display: flex;
    overflow: auto hidden;
  }
  .step {
    margin: 2px;
  }
  .step p {
    background-color: var(--theme-neutral-light);
    border-radius: 5px;
    text-align: center;
    margin: 0;
    padding: 2px;
  }
  .step a {
    font-weight: 400;
  }
</style>
