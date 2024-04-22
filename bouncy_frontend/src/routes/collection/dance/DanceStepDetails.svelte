<script>
  import UiBox from '$lib/components/ui/UiBox.svelte';
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
</script>

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
</style>
