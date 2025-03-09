<script>
  import Symbol from '$lib/components/ui/Symbol.svelte';
  import UiBox from '$lib/components/ui/UiBox.svelte';
  import { stepById } from 'bouncy_instructor';
  import Step from '../Step.svelte';

  
  
  
  
  const stepSize = 100;
  
  
  
  /**
   * @typedef {Object} Props
   * @property {import("bouncy_instructor").StepWrapper[]} allSteps
   * @property {import("bouncy_instructor").StepWrapper} selectedStep
   * @property {number} poseIndex
   * @property {number} animationTime
   * @property {(step: import('bouncy_instructor').StepWrapper) => void } selectedVariationCallback
   * @property {(flipped: boolean) => void } setFlippedCallback
   * @property {boolean} flipped
   */

  /** @type {Props} */
  let {
    allSteps,
    selectedStep,
    poseIndex,
    animationTime,
    selectedVariationCallback,
    setFlippedCallback,
    flipped
  } = $props();

  const sideway =
    selectedStep.skeleton(0).sideway || selectedStep.skeleton(1).sideway;
  const baseStep = stepById(selectedStep.id, false) || selectedStep;
  const flippedStep = stepById(selectedStep.id, true) || selectedStep;
  /** @type {import("bouncy_instructor").StepWrapper[]} */
  let variations = $derived(allSteps.filter((step) => step.name == selectedStep.name));
</script>

{#if sideway}
  <UiBox title="editor.direction">
    <!-- svelte-ignore a11y_no_static_element_interactions -->

    <div class="outer-box">
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <div class="step" onclick={() => setFlippedCallback(false)}>
        <div class="fixed-size">
          <div class="center">
            <Step
              step={baseStep}
              {poseIndex}
              {animationTime}
              size={stepSize}
              borderWidth={flipped ? 2 : 5}
            />
          </div>
        </div>
        <p class="direction">
          <Symbol>arrow_back</Symbol>
        </p>
      </div>
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <div class="step" onclick={() => setFlippedCallback(true)}>
        <div class="fixed-size">
          <div class="center">
            <Step
              step={flippedStep}
              {poseIndex}
              {animationTime}
              size={stepSize}
              borderWidth={flipped ? 5 : 2}
            />
          </div>
        </div>
        <p class="direction">
          <Symbol>arrow_forward</Symbol>
        </p>
      </div>
    </div>
  </UiBox>
{/if}

{#if variations.length > 1}
  <UiBox title="collection.step.variation">
    <div class="outer-box">
      {#each variations as variation}
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div class="step" onclick={() => selectedVariationCallback(variation)}>
          <div class="fixed-size">
            <div class="center">
              <Step
                step={variation}
                {poseIndex}
                {animationTime}
                size={stepSize}
                borderWidth={selectedStep.id === variation.id ? 5 : 2}
              />
            </div>
          </div>
          <p class="name">{variation.variation}</p>
        </div>
      {/each}
    </div>
  </UiBox>
{/if}

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

  .fixed-size {
    height: 120px;
    width: 120px;
  }
  .center {
    margin: auto;
  }
</style>
