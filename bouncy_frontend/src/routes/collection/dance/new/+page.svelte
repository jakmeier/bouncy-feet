<script>
  import { t } from '$lib/i18n.js';
  import Header from '$lib/Header.svelte';
  import DanceAnimation from '../../../DanceAnimation.svelte';
  import Step from '../../Step.svelte';
  import { counter } from '$lib/timer';
  import { DanceInfo } from '$lib/instructor/bouncy_instructor';
  import Area from '../../../record/Area.svelte';
  import SelectStep from './SelectStep.svelte';

  /** @type {import('./$types').PageData} */
  export let data;

  const bpm = 240;
  const stepTime = 60_000 / bpm;
  const animationTime = stepTime * 0.85;
  const danceSize = 250;
  const stepSize = 100;
  const borderRadius = '25px';

  const beatCounter = counter(-1, 1, stepTime);

  let id = $t('collection.new-dance.default-dance-name');
  /** @type {DanceInfo | null} */
  let dance = null;
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
    preview();
    return true;
  }

  function preview() {
    if (steps.length > 0) {
      // cursed: need to rust-clone `StepInfo`s before they are moved in a
      // vector, which leaves the original values nulled out.
      const stepsClone = steps.map((s) => s.rustClone());
      dance = new DanceInfo('tempDance', stepsClone);
    }
  }
</script>

<Header title={id} />

<div class="page">
  <div
    class="dance"
    style="max-width: {danceSize}px; max-height: {danceSize}px"
  >
    {#if dance !== null}
      <DanceAnimation {dance} />
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
