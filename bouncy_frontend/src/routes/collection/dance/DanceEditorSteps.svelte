<script>
  import Step from '../Step.svelte';
  import { counter } from '$lib/timer';
  import Area from '$lib/components/ui/Area.svelte';
  import SelectStep from './SelectStep.svelte';

  /** @type {import("$lib/instructor/bouncy_instructor").StepInfo[]} */
  export let availableSteps;
  /** @type {import('$lib/instructor/bouncy_instructor').DanceBuilder} */
  export let danceBuilder;

  const bpm = 240;
  const stepTime = 60_000 / bpm;
  const animationTime = stepTime * 0.85;
  const stepSize = 100;
  const borderRadius = '25px';

  const beatCounter = counter(-1, 1, stepTime);

  /**
   * @type {import('$lib/instructor/bouncy_instructor').StepInfo[]}
   */
  let steps = danceBuilder.danceInfo().steps();

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

  // Drag-and-drop stuff below
  /** @type {string | null} */
  let draggedStep = null;

  /**
   * @param {DragEvent} event
   * @param {import("$lib/instructor/bouncy_instructor").StepInfo} step
   */
  function handleDragStart(event, step) {
    draggedStep = step.id;

    if (event.dataTransfer) {
      event.dataTransfer.effectAllowed = 'move';
      event.dataTransfer.dropEffect = 'move';
    }
    danceBuilder = danceBuilder;
    steps = danceBuilder.danceInfo().steps();
  }

  /**
   * @param {{preventDefault: () => void;}} event
   * @param {number} index
   */
  function handleDragOver(event, index) {
    event.preventDefault();

    const draggedIndex = steps.findIndex((step) => step.id === draggedStep);
    if (draggedStep && draggedIndex !== -1 && draggedIndex !== index) {
      console.log('moving', draggedIndex, 'to', index);
      danceBuilder.removeStep(draggedIndex);
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
  }
</script>

<div class="outer">
  <div class="steps-container">
    {#each steps as step, i}
      <div
        class="step"
        draggable="true"
        on:dragstart={(event) => handleDragStart(event, step)}
        on:dragover={(event) => handleDragOver(event, i)}
        on:drop={handleDrop}
        on:dragend={handleDrop}
        style="opacity: {step.id === draggedStep ? 0.3 : 1.0}"
      >
        <Step {step} poseIndex={$beatCounter} {animationTime} size={stepSize} />
        <p class="handle" style="width: {stepSize}px">
          <span class="material-symbols-outlined">open_with</span>
        </p>
        <p style="width: {stepSize}px">{step.name}</p>
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
    steps={availableSteps}
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
    margin: 2px;
    transition: all 0.2s ease-in-out;
  }
  .step p {
    background-color: var(--theme-neutral-light);
    border-radius: 5px;
    text-align: center;
    margin: 0;
    padding: 2px;
  }
  p.handle {
    margin-bottom: 5px;
  }
</style>
