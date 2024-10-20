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

  const localCollectionCtx = getContext('localCollection');
  const availablePoses = localCollectionCtx.poses;

  /** @param {StepWrapper} loadedStep */
  export function loadStep(loadedStep) {
    step = loadedStep;
    stepName = step.name;
  }

  /** @type {StepWrapper} */
  let step = newStep();
  let stepName = $t('editor.name-input-placeholder');
  $: stepPositionBuilders = step.positions();

  let showAddNewItem = false;

  function newStep() {
    const idNum = Math.random().toFixed(6).substring(2);
    const id = `step-${idNum}`;
    const name = `Step ${idNum} Name`;
    const step = new StepWrapper(id, name, 'lab');
    localCollectionCtx.addStep(step);
    return step;
  }

  /** @param {PoseWrapper} pose */
  function addPose(pose) {
    step.addPosition(new StepPositionBuilder(pose));
    step = step;
  }
</script>

<input id="name" name="name" bind:value={stepName} />

<DraggableList items={stepPositionBuilders} bind:showAddNewItem>
  <slot slot="main" let:item={position}>
    <div class="pose">
      <Pose pose={position.pose()}></Pose>
    </div>
  </slot>
  <slot slot="name" let:item={position}>{position.pose().name('en')}</slot>
</DraggableList>

{#if showAddNewItem}
  <UiBox title="editor.pick-pose-prompt">
    <div>
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
            <p>{pose.name('en')}</p>
            <Pose {pose}></Pose>
          </div>
        </div>
      {/each}
    </div>
  </UiBox>
{/if}
