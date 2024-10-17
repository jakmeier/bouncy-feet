<script>
  import Step from '../Step.svelte';
  import { counter } from '$lib/timer';
  import SelectStep from './SelectStep.svelte';
  import DanceStepDetails from './DanceStepDetails.svelte';
  import DraggableList from '$lib/components/ui/DraggableList.svelte';
  import { StepWrapper } from '$lib/instructor/bouncy_instructor';

  /** @type {import("$lib/instructor/bouncy_instructor").StepWrapper[]} */
  export let availableSteps;
  /** @type {import("$lib/instructor/bouncy_instructor").StepWrapper[]} */
  $: uniqueSteps = availableSteps.filter(
    (step, index, self) => index === self.findIndex((t) => t.name === step.name)
  );
  /** @type {import('$lib/instructor/bouncy_instructor').DanceBuilder} */
  export let danceBuilder;

  export let animationTime;
  export let stepTime;

  const stepSize = 100;

  export let beatCounter = counter(-1, 1, stepTime);

  /**
   * @type {StepWrapper[]}
   */
  let steps = danceBuilder.danceInfo().steps();
  /** @type {number} */
  let selectedStepIndex = -1;

  let addNewStepActive = false;
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
    <div slot="main" let:item={step} let:index>
      <Step
        {step}
        poseIndex={$beatCounter}
        {animationTime}
        size={stepSize}
        borderWidth={selectedStepIndex === index ? 5 : 2}
      />
    </div>
    <div slot="name" let:item={step}>
      {step.name}
    </div>
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
