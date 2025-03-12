<script>
  import Step from '../Step.svelte';
  import { counter } from '$lib/timer';
  import SelectStep from './SelectStep.svelte';
  import DanceStepDetails from './DanceStepDetails.svelte';
  import DraggableList from '$lib/components/ui/DraggableList.svelte';
  import { StepWrapper } from '$lib/instructor/bouncy_instructor';

  
  


  const stepSize = 100;

  /**
   * @typedef {Object} Props
   * @property {import("bouncy_instructor").StepWrapper[]} availableSteps
   * @property {import('bouncy_instructor').DanceBuilder} danceBuilder
   * @property {any} animationTime
   * @property {any} stepTime
   * @property {any} [beatCounter]
   */

  /** @type {Props} */
  let {
    availableSteps,
    danceBuilder = $bindable(),
    animationTime,
    stepTime,
    beatCounter = counter(-1, 1, stepTime)
  } = $props();

  /**
   * @type {StepWrapper[]}
   */
  let steps = $state(danceBuilder.danceInfo().steps());
  /** @type {number} */
  let selectedStepIndex = $state(-1);

  let addNewStepActive = $state(false);
  /** @param {StepWrapper} stepWrapper */
  function selectedCallback(stepWrapper) {
    if (!stepWrapper) {
      return false;
    }

    selectedStepIndex = steps.length;
    steps.push(stepWrapper);
    danceBuilder.addStep(stepWrapper.id);
    // trigger reload in ui (can I do better?)
    danceBuilder = danceBuilder;

    return true;
  }

  /** @param {StepWrapper} stepWrapper */
  function selectedVariationCallback(stepWrapper) {
    if (selectedStepIndex !== -1) {
      steps[selectedStepIndex] = stepWrapper;
      danceBuilder.removeStep(selectedStepIndex);
      danceBuilder.insertStep(selectedStepIndex, stepWrapper.id);
    }
  }

  /** @param {boolean} flipped */
  function setFlippedCallback(flipped) {
    if (selectedStepIndex !== -1) {
      danceBuilder.setOrientation(selectedStepIndex, flipped);
      steps = danceBuilder.danceInfo().steps();
      danceBuilder = danceBuilder;
    }
  }

  /** @type {(item: any, index: number) => void} */
  function onRemove(_item, index) {
    danceBuilder.removeStep(index);
    danceBuilder = danceBuilder;
    steps = danceBuilder.danceInfo().steps();
  }

  /**
   * @type {(
   *    draggedItem: StepWrapper,
   *    draggedIndex: number,
   *    swappedItem: StepWrapper,
   *    swappedIndex: number
   * )
   * => number
   * } */
  function onDragMove(draggedItem, draggedIndex, _swappedItem, swappedIndex) {
    let flipped = danceBuilder.isFlipped(draggedIndex);
    danceBuilder.removeStep(draggedIndex);
    danceBuilder.insertStep(swappedIndex, draggedItem.id);
    danceBuilder.setOrientation(swappedIndex, flipped);
    danceBuilder = danceBuilder;
    steps = danceBuilder.danceInfo().steps();
    return swappedIndex;
  }
  /** @type {import("bouncy_instructor").StepWrapper[]} */
  let uniqueSteps = $derived(availableSteps.filter(
    (step, index, self) => index === self.findIndex((t) => t.name === step.name)
  ));
</script>

<div class="outer">
  <DraggableList
    items={steps}
    bind:selectedIndex={selectedStepIndex}
    bind:showAddNewItem={addNewStepActive}
    elementSize={stepSize}
    {onDragMove}
    {onRemove}
  >
    {#snippet main({ item: step, index })}
        <div   >
        <Step
          {step}
          poseIndex={$beatCounter}
          {animationTime}
          size={stepSize}
          borderWidth={selectedStepIndex === index ? 5 : 2}
        />
      </div>
      {/snippet}
    {#snippet name({ item: step })}
        <div  >
        {step.name}
      </div>
      {/snippet}
  </DraggableList>

  {#if selectedStepIndex !== -1}
    <DanceStepDetails
      allSteps={availableSteps}
      selectedStep={steps[selectedStepIndex]}
      poseIndex={$beatCounter}
      {animationTime}
      {selectedVariationCallback}
      {setFlippedCallback}
      flipped={danceBuilder.isFlipped(selectedStepIndex)}
    ></DanceStepDetails>
  {/if}

  <SelectStep
    bind:show={addNewStepActive}
    {selectedCallback}
    steps={uniqueSteps}
    poseIndex={$beatCounter}
    {animationTime}
  ></SelectStep>
</div>

<style>
  .outer {
    overflow: hidden;
  }
</style>
