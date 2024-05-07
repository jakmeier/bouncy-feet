<script>
  import Step from '../Step.svelte';
  import { counter } from '$lib/timer';
  import Area from '$lib/components/ui/Area.svelte';
  import SelectStep from './SelectStep.svelte';
  import DanceStepDetails from './DanceStepDetails.svelte';

  /** @type {import("$lib/instructor/bouncy_instructor").StepInfo[]} */
  export let availableSteps;
  /** @type {import("$lib/instructor/bouncy_instructor").StepInfo[]} */
  $: uniqueSteps = availableSteps.filter(
    (step, index, self) => index === self.findIndex((t) => t.name === step.name)
  );
  /** @type {import('$lib/instructor/bouncy_instructor').DanceBuilder} */
  export let danceBuilder;

  export let animationTime;
  export let stepTime;

  const stepSize = 100;
  const borderRadius = '25px';

  const beatCounter = counter(-1, 1, stepTime);

  /**
   * @type {import('$lib/instructor/bouncy_instructor').StepInfo[]}
   */
  let steps = danceBuilder.danceInfo().steps();
  let selectedStepIndex = -1;

  let stepSelectionActive = false;
  /** @param {import('$lib/instructor/bouncy_instructor').StepInfo} stepInfo */
  function selectedCallback(stepInfo) {
    if (!stepInfo) {
      return false;
    }

    selectedStepIndex = steps.length;
    steps.push(stepInfo);
    danceBuilder.addStep(stepInfo.id);
    // trigger reload in ui (can I do better?)
    danceBuilder = danceBuilder;

    return true;
  }

  /** @param {import('$lib/instructor/bouncy_instructor').StepInfo} stepInfo */
  function selectedVariationCallback(stepInfo) {
    if (selectedStepIndex !== -1) {
      steps[selectedStepIndex] = stepInfo;
      danceBuilder.removeStep(selectedStepIndex);
      danceBuilder.insertStep(selectedStepIndex, stepInfo.id);
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

  /**
   * @param {{preventDefault: () => void;}} event
   * @param {number} index
   */
  function handleRemove(event, index) {
    event.preventDefault();
    selectedStepIndex = -1;
    danceBuilder.removeStep(index);
    danceBuilder = danceBuilder;
    steps = danceBuilder.danceInfo().steps();
  }

  // Drag-and-drop stuff below
  /** @type {string | null} */
  let draggedStep = null;
  /** @type {number} */
  let draggedStepIndex = -1;

  /**
   * @param {DragEvent} event
   * @param {number} index
   */
  function handleDragStart(event, index) {
    draggedStep = steps[index].id;
    draggedStepIndex = index;
    selectedStepIndex = -1;

    if (event.dataTransfer) {
      event.dataTransfer.effectAllowed = 'move';
      event.dataTransfer.dropEffect = 'move';
    }
  }

  /**
   * @param {{preventDefault: () => void;}} event
   * @param {number} index
   */
  function handleDragOver(event, index) {
    event.preventDefault();

    if (draggedStep && draggedStepIndex !== -1 && draggedStepIndex !== index) {
      danceBuilder.removeStep(draggedStepIndex);
      draggedStepIndex = index;
      danceBuilder.insertStep(index, draggedStep);
      danceBuilder = danceBuilder;
      steps = danceBuilder.danceInfo().steps();
    }
  }

  /**
   * @param {DragEvent & { currentTarget: EventTarget & HTMLDivElement; }} event
   */
  function handleDrop(event) {
    event.preventDefault();
    draggedStep = null;
    draggedStepIndex = -1;
  }

  function clickAddButton() {
    stepSelectionActive = !stepSelectionActive;
    selectedStepIndex = -1;
  }

  /**
   * @param {number} i
   */
  function selectStep(i) {
    if (selectedStepIndex === i) {
      selectedStepIndex = -1;
    } else {
      selectedStepIndex = i;
      stepSelectionActive = false;
    }
  }
</script>

<div class="outer">
  <div class="steps-container">
    {#each steps as step, i}
      <!-- svelte-ignore a11y-no-static-element-interactions -->
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <div
        class="step"
        draggable="true"
        on:dragstart={(event) => handleDragStart(event, i)}
        on:dragover={(event) => handleDragOver(event, i)}
        on:drop={handleDrop}
        on:dragend={handleDrop}
        on:click={() => selectStep(i)}
        style="opacity: {i === draggedStepIndex ? 0.3 : 1.0}"
      >
        <div class="fixed-size">
          <div class="center">
            <Step
              {step}
              poseIndex={$beatCounter}
              {animationTime}
              size={stepSize}
              borderWidth={selectedStepIndex === i ? 5 : 2}
            />
          </div>
        </div>
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <div class="delete-button" on:click={(event) => handleRemove(event, i)}>
          <span class="material-symbols-outlined">close</span>
        </div>
        <p class="handle" style="width: {stepSize}px">
          <span class="material-symbols-outlined">open_with</span>
        </p>
        <p style="width: {stepSize}px">{step.name}</p>
      </div>
    {/each}

    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div on:click={clickAddButton}>
      <Area width="{stepSize}px" height="{stepSize}px" {borderRadius}>
        <span
          class="material-symbols-outlined add-button"
          style="font-size: {stepSize / 2}px"
        >
          add_circle
        </span>
      </Area>
    </div>
  </div>

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
    bind:show={stepSelectionActive}
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
  .steps-container {
    display: flex;
    overflow: auto hidden;
  }
  .step {
    position: relative;
    margin: 2px;
    transition: all 0.2s ease-in-out;
  }
  .step p,
  div.delete-button {
    border-radius: 5px;
    text-align: center;
    margin: 0;
    padding: 2px;
  }
  .step p {
    background-color: var(--theme-neutral-light);
  }
  p.handle {
    background-color: var(--theme-accent-light);
    margin-bottom: 5px;
  }
  div.delete-button {
    background-color: var(--theme-accent-light);
    position: absolute;
    width: 35px;
    height: 25px;
    top: 0px;
    right: 0px;
  }
  div.delete-button:hover {
    background-color: var(--theme-accent);
  }

  .fixed-size {
    height: 115px;
    width: 110px;
  }
  .center {
    margin: auto;
  }
</style>
