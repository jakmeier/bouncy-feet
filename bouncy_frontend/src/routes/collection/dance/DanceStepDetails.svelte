<script>
  import UiBox from '$lib/components/ui/UiBox.svelte';
  import { stepById } from '$lib/instructor/bouncy_instructor';
  import Step from '../Step.svelte';

  /** @type {import("$lib/instructor/bouncy_instructor").StepInfo[]} */
  export let allSteps;
  /** @type {import("$lib/instructor/bouncy_instructor").StepInfo} */
  export let selectedStep;
  /** @type {import("$lib/instructor/bouncy_instructor").StepInfo[]} */
  $: variations = allSteps.filter((step) => step.name == selectedStep.name);
  /** @type {number} */
  export let poseIndex;
  /** @type {number} */
  export let animationTime;
  const stepSize = 100;
  /**
   * @type {(step: import('$lib/instructor/bouncy_instructor').StepInfo) => void }
   */
  export let selectedVariationCallback;
  /**
   * @type {(flipped: boolean) => void }
   */
  export let setFlippedCallback;
  /** @type {boolean} */
  export let flipped;

  const baseStep = stepById(selectedStep.id, false) || selectedStep;
  const flippedStep = stepById(selectedStep.id, true) || selectedStep;
  //    allSteps.find((step) => step.id == selectedStep.id);
</script>

<UiBox title="editor.direction">
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div class="outer-box">
    <p></p>
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div class="step" on:click={() => setFlippedCallback(false)}>
      <Step
        step={baseStep}
        {poseIndex}
        {animationTime}
        size={stepSize}
        borderWidth={flipped ? 2 : 5}
      />
      <p class="direction">
        <span class="material-symbols-outlined"> arrow_back </span>
      </p>
    </div>
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div class="step" on:click={() => setFlippedCallback(true)}>
      <Step
        step={flippedStep}
        {poseIndex}
        {animationTime}
        size={stepSize}
        borderWidth={flipped ? 5 : 2}
      />
      <p class="direction">
        <span class="material-symbols-outlined"> arrow_forward </span>
      </p>
    </div>
  </div>
</UiBox>

<UiBox title="collection.step.variation">
  <div class="outer-box">
    {#each variations as variation}
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <!-- svelte-ignore a11y-no-static-element-interactions -->
      <div class="step" on:click={() => selectedVariationCallback(variation)}>
        <Step
          step={variation}
          {poseIndex}
          {animationTime}
          size={stepSize}
          borderWidth={selectedStep.id === variation.id ? 5 : 2}
        />
        <p class="name">{variation.variation}</p>
      </div>
    {/each}
  </div>
</UiBox>

<style>
  .outer-box {
    display: flex;
    justify-content: center;
    gap: 20px;
    width: 100%;
  }

  .name {
    border-radius: 5px;
    text-align: center;
    margin: 0;
    padding: 2px;
    background-color: var(--theme-neutral-white);
  }

  .direction {
    margin: 0;
  }
</style>
