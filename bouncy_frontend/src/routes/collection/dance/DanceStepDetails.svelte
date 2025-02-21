<script>
  import Symbol from '$lib/components/ui/Symbol.svelte';
  import UiBox from '$lib/components/ui/UiBox.svelte';
  import { stepById } from '$lib/instructor/bouncy_instructor';
  import Step from '../Step.svelte';

  /** @type {import("$lib/instructor/bouncy_instructor").StepWrapper[]} */
  export let allSteps;
  /** @type {import("$lib/instructor/bouncy_instructor").StepWrapper} */
  export let selectedStep;
  /** @type {import("$lib/instructor/bouncy_instructor").StepWrapper[]} */
  $: variations = allSteps.filter((step) => step.name == selectedStep.name);
  /** @type {number} */
  export let poseIndex;
  /** @type {number} */
  export let animationTime;
  const stepSize = 100;
  /**
   * @type {(step: import('$lib/instructor/bouncy_instructor').StepWrapper) => void }
   */
  export let selectedVariationCallback;
  /**
   * @type {(flipped: boolean) => void }
   */
  export let setFlippedCallback;
  /** @type {boolean} */
  export let flipped;

  const sideway =
    selectedStep.skeleton(0).sideway || selectedStep.skeleton(1).sideway;
  const baseStep = stepById(selectedStep.id, false) || selectedStep;
  const flippedStep = stepById(selectedStep.id, true) || selectedStep;
</script>

{#if sideway}
  <UiBox title="editor.direction">
    <!-- svelte-ignore a11y-no-static-element-interactions -->

    <div class="outer-box">
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <div class="step" on:click={() => setFlippedCallback(false)}>
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
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <div class="step" on:click={() => setFlippedCallback(true)}>
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
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <!-- svelte-ignore a11y-no-static-element-interactions -->
        <div class="step" on:click={() => selectedVariationCallback(variation)}>
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
