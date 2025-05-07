<script>
  import { run } from 'svelte/legacy';

  import { t } from '$lib/i18n';
  import {
    PoseWrapper,
    StepPositionBuilder,
    StepWrapper,
  } from '$lib/instructor/bouncy_instructor';
  import { getContext, onMount } from 'svelte';
  import DraggableList from '../ui/DraggableList.svelte';
  import UiBox from '../ui/UiBox.svelte';
  import Pose from '../Pose.svelte';
  import StepPositionDetails from './StepPositionDetails.svelte';
  import AnimatedStep from '../AnimatedStep.svelte';
  import { beforeNavigate } from '$app/navigation';
  import { beatCounter, bpm } from '$lib/stores/Beat';
  import Button from '../ui/Button.svelte';

  const localCollectionCtx = getContext('localCollection');
  const availablePoses = localCollectionCtx.poses;

  /** @param {StepWrapper} loadedStep */
  export function loadStep(loadedStep) {
    step = loadedStep;
    stepName = step.name;
  }

  export function save() {
    localCollectionCtx.addStep(step);
    isDirty = false;
  }

  /** @type {StepWrapper} */
  let step = $state(newStep());
  let stepName = $state($t('editor.name-input-placeholder'));

  let showAddNewItem = $state(false);
  let selectedIndex = $state(-1);
  let isDirty = false;
  let stepBpm = $state($bpm);

  /** @type {(position: StepPositionBuilder|undefined)=>void} */
  let loadPosition = $state();

  /** @param {number} index */
  function selectPosition(index) {
    if (!loadPosition) return;
    if (index === -1) {
      loadPosition(undefined);
    } else {
      loadPosition(stepPositionBuilders[index]);
    }
  }

  function newStep() {
    const idNum = Math.random().toFixed(6).substring(2);
    const id = `step-${idNum}`;
    const name = `Step ${idNum}`;
    const step = new StepWrapper(id, name, 'lab');
    return step;
  }

  /** @param {PoseWrapper} pose */
  function addPose(pose) {
    step.addPosition(new StepPositionBuilder(pose));
    isDirty = true;
    step = step;
  }

  /** @type {(item: any, index: number) => void} */
  function onRemove(_item, index) {
    step.removePosition(index);
    isDirty = true;
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
    isDirty = true;
    step = step;
    // console.log(step.poses().map((pose) => pose.name('en')));
    return swappedIndex;
  }

  /** @param {StepPositionBuilder} position */
  function onPositionDetailsChanged(position) {
    if (selectedIndex) {
      step.removePosition(selectedIndex);
      step.insertPosition(selectedIndex, position);
      isDirty = true;
      step = step;
    } else {
      console.warn("no index selected, can't update step position");
    }
  }

  beforeNavigate(({ cancel }) => {
    if (isDirty) {
      if (!confirm($t('editor.confirm-leave'))) {
        cancel();
      }
    }
  });

  let stepPositionBuilders = $derived(step.positions());
  run(() => {
    selectPosition(selectedIndex);
  });
</script>

<AnimatedStep {step} size={200}></AnimatedStep>

<div class="step-settings">
  <Button
    class="small"
    symbolSize={28}
    symbol="first_page"
    on:click={() => beatCounter.reset()}
  ></Button>
  <label>
    {$t('courses.lesson.bpm-label')}
    <input
      type="range"
      bind:value={stepBpm}
      onchange={() => {
        $bpm = stepBpm;
      }}
      min="30"
      max="240"
      class="range"
    />
  </label>

  <input
    id="name"
    name="name"
    bind:value={stepName}
    onchange={() => (step.name = stepName)}
  />
</div>

<DraggableList
  items={stepPositionBuilders}
  bind:selectedIndex
  bind:showAddNewItem
  {onDragMove}
  {onRemove}
>
  {#snippet main({ item: position, index })}
    <div>
      <div class="pose" class:selected={index === selectedIndex}>
        <Pose pose={position.pose()}></Pose>
      </div>
    </div>
  {/snippet}
  {#snippet name({ item: position, index })}
    <div class:selected={index === selectedIndex}>
      {position.pose().name('en')}
    </div>
  {/snippet}
</DraggableList>

<StepPositionDetails bind:loadPosition onChange={onPositionDetailsChanged} />

{#if showAddNewItem}
  <UiBox title="editor.pick-pose-prompt">
    <div class="available-poses">
      {#each $availablePoses as pose}
        <div
          onclick={() => addPose(pose)}
          role="button"
          tabindex={0}
          onkeydown={(event) => {
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
  .step-settings {
    margin: auto;
    width: 95%;
    text-align: center;
    width: max(200px, 67%);
  }

  label {
    display: flex;
    align-items: center;
    gap: 20px;
  }

  input#name {
    width: 100%;
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

  .pose {
    width: 100px;
    padding: 2px;
  }
  .selected {
    font-weight: 800;
  }
  .pose.selected {
    background-color: var(--theme-neutral-light);
    border-radius: 20px;
  }

  @media (min-width: 730px) {
    .available-poses {
      grid-template-columns: repeat(5, 1fr);
    }
  }
</style>
