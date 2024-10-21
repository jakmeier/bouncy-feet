<script>
  import Header from '$lib/components/ui/Header.svelte';
  import { t } from '$lib/i18n';
  import { counter } from '$lib/timer';
  import { getContext } from 'svelte';
  import Step from '../../collection/Step.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import { goto } from '$app/navigation';
  import { StepWrapper } from '$lib/instructor/bouncy_instructor';
  import Popup from '$lib/components/ui/Popup.svelte';

  const localCollectionCtx = getContext('localCollection');

  /** @type {import("svelte/store").Readable<StepWrapper[]>} */
  const steps = localCollectionCtx.steps;

  const stepTime = 300;
  const animationTime = stepTime * 0.7;
  const i = counter(-1, 1, stepTime);

  let selectedIndex = -1;
  /** @type {import('svelte/store').Writable.<boolean>}*/
  let showDeleteConfirmation;

  /** @param {number} index */
  function select(index) {
    selectedIndex = index;
  }

  function editStep() {
    const stepId = $steps[selectedIndex].id;
    goto(`./${stepId}`);
  }

  function deleteStep() {
    $showDeleteConfirmation = true;
  }

  function deleteConfirmed() {
    const stepId = $steps[selectedIndex].id;
    localCollectionCtx.removeStep(stepId);
    $showDeleteConfirmation = false;
    selectedIndex = -1;
  }

  function cancelDelete() {
    $showDeleteConfirmation = false;
  }
</script>

<Header title={$t('editor.step.title')}></Header>

<div class="centered">
  <a href="./new">
    <button class="light big wide"> {$t('editor.step.new')} </button>
  </a>
</div>

<h2 class="box">{$t('editor.step.list')}</h2>

<div class="steps">
  {#each $steps as step, index}
    <div
      class="step"
      on:click={() => select(index)}
      on:keydown={() => select(index)}
      role="button"
      tabindex={0}
    >
      <div class:bold={selectedIndex === index}>{step.name}</div>
      <div class="inner-step">
        <Step
          {step}
          poseIndex={$i}
          {animationTime}
          borderWidth={selectedIndex === index ? 4 : 2}
        />
        {#if selectedIndex === index}
          <div class="selected">
            <Button
              class="light small"
              symbolSize={18}
              symbol="edit"
              on:click={editStep}
            />
            <Button
              class="light small"
              symbolSize={18}
              symbol="delete"
              on:click={deleteStep}
            />
          </div>
        {/if}
      </div>
    </div>
  {/each}
</div>

<Popup bind:isOpen={showDeleteConfirmation} title={"editor.delete-confirmation-title"}>
  <p>
    {$t("editor.step.delete-confirmation0")}
    "{$steps[selectedIndex].name}"
    {$t("editor.step.delete-confirmation1")}
  </p>

  <button class="light wide" on:click={cancelDelete}>{$t("editor.cancel-delete-button")}</button>
  <button class="light wide" on:click={deleteConfirmed}>{$t("editor.confirm-delete-button")}</button>
</Popup>

<style>
  .steps {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
  }
  .step {
    max-width: 200px;
  }
  .inner-step {
    max-width: 200px;
    position: relative;
  }
  .centered {
    margin: 30px auto;
    text-align: center;
  }

  .selected {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;

    background-color: #c2bfff80;
    border-radius: 20px;

    display: flex;
    align-items: center;
    justify-content: space-around;
  }

  .bold {
    font-weight: 800;
    text-shadow: gray 0px 1px 2px;
  }
</style>
