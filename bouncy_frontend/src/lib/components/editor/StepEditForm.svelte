<script>
  import { t } from '$lib/i18n';
  import {
    PoseWrapper,
    StepPositionBuilder,
    StepWrapper,
  } from '$lib/instructor/bouncy_instructor';
  import { getContext } from 'svelte';
  import DraggableList from '../ui/DraggableList.svelte';
  import UiBox from '../ui/UiBox.svelte';
  import Pose from '../Pose.svelte';
  import StepPositionDetails from './StepPositionDetails.svelte';
  import AnimatedStep from '../AnimatedStep.svelte';

  const localCollectionCtx = getContext('localCollection');
  const availablePoses = localCollectionCtx.poses;

  /** @param {StepWrapper} loadedStep */
  export function loadStep(loadedStep) {
    step = loadedStep;
    stepName = step.name;
  }

  export function save() {
    localCollectionCtx.addStep(step);
  }

  /** @type {StepWrapper} */
  let step = newStep();
  let stepName = $t('editor.name-input-placeholder');
  $: stepPositionBuilders = step.positions();

  let showAddNewItem = false;
  let selectedIndex = -1;

  function newStep() {
    const idNum = Math.random().toFixed(6).substring(2);
    const id = `step-${idNum}`;
    const name = `Step ${idNum}`;
    const step = new StepWrapper(id, name, 'lab');
    localCollectionCtx.addStep(step);
    return step;
  }

  /** @param {PoseWrapper} pose */
  function addPose(pose) {
    step.addPosition(new StepPositionBuilder(pose));
    step = step;
  }

  /** @type {(item: any, index: number) => void} */
  function onRemove(_item, index) {
    step.removePosition(index);
    step = step;
  }

  /**
   * @type {(
   *    draggedItem: StepPositionBuilder,
   *    draggedIndex: number,
   *    swappedItem: StepPositionBuilder,
   *    swappedIndex: number
   * )
   * => number
   * } */
  function onDragMove(draggedItem, draggedIndex, swappedItem, swappedIndex) {
    if (draggedIndex < swappedIndex) {
      step.removePosition(swappedIndex);
      step.insertPosition(draggedIndex, swappedItem);
    } else if (draggedIndex > swappedIndex) {
      step.removePosition(draggedIndex);
      step.insertPosition(swappedIndex, draggedItem);
    }
    step = step;
    console.log(step.poses().map((pose) => pose.name('en')));
    return swappedIndex;
  }
</script>

<AnimatedStep {step} size={200}></AnimatedStep>

<input
  id="name"
  name="name"
  bind:value={stepName}
  on:change={() => (step.name = stepName)}
/>

<DraggableList
  items={stepPositionBuilders}
  bind:selectedIndex
  bind:showAddNewItem
  {onDragMove}
  {onRemove}
>
  <slot slot="main" let:item={position}>
    <div class="pose">
      <Pose pose={position.pose()}></Pose>
    </div>
  </slot>
  <slot slot="name" let:item={position}>{position.pose().name('en')}</slot>
</DraggableList>

{#if selectedIndex !== -1}
  <StepPositionDetails position={stepPositionBuilders[selectedIndex]} />
{/if}

{#if showAddNewItem}
  <UiBox title="editor.pick-pose-prompt">
    <div class="available-poses">
      {#each $availablePoses as pose}
        <div
          on:click={() => addPose(pose)}
          role="button"
          tabindex={0}
          on:keydown={(event) => {
            if (event.key === 'Enter' || event.key === ' ') {
              addPose(pose);
            }
          }}
        >
          <div class="pose">
            <div>{pose.name('en')}</div>
            <Pose {pose}></Pose>
          </div>
        </div>
      {/each}
    </div>
  </UiBox>
{/if}

<style>
  input#name {
    display: block;
    margin: auto;
    width: 95%;
    text-align: center;
  }

  .available-poses {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    align-items: end;
    gap: 10px;
  }
  .available-poses div {
    /* ensure equal size per grid column */
    min-width: 0;
  }

  @media (min-width: 730px) {
    .available-poses {
      grid-template-columns: repeat(5, 1fr);
    }
  }
</style>
