<script>
  import Header from '$lib/components/ui/Header.svelte';
  import { t } from '$lib/i18n';
  import { counter } from '$lib/timer';
  import { getContext } from 'svelte';
  import Step from '../../collection/Step.svelte';
  import { goto } from '$app/navigation';
  import { StepWrapper } from '$lib/instructor/bouncy_instructor';
  import EditOrDeleteList from '$lib/components/ui/EditOrDeleteList.svelte';

  const localCollectionCtx = getContext('localCollection');

  /** @type {import("svelte/store").Readable<StepWrapper[]>} */
  const steps = localCollectionCtx.steps;

  const stepTime = 300;
  const animationTime = stepTime * 0.7;
  const i = counter(-1, 1, stepTime);

  /** @param {number} index */
  function editStep(index) {
    const stepId = $steps[index].id;
    goto(`./${stepId}`);
  }

  /** @param {number} index */
  function deleteConfirmed(index) {
    const stepId = $steps[index].id;
    localCollectionCtx.removeStep(stepId);
  }
</script>

<Header title={$t('editor.step.title')}></Header>

<div class="centered">
  <a href="./new">
    <button class="light big wide"> {$t('editor.step.new')} </button>
  </a>
</div>

<h2 class="box">{$t('editor.step.list')}</h2>

<EditOrDeleteList items={$steps} onDelete={deleteConfirmed} onEdit={editStep}>
  <div slot="item" let:item let:index let:selected>
    <div class:bold={selected}>{item.name}</div>
    <Step
      step={item}
      poseIndex={$i}
      {animationTime}
      borderWidth={selected ? 4 : 2}
    />
  </div>
  <div slot="confirm-delete-text" let:item>
    <p>
      {$t('editor.step.delete-confirmation0')}
      "{item.name}"
      {$t('editor.step.delete-confirmation1')}
    </p>
  </div>
</EditOrDeleteList>

<style>
  .centered {
    margin: 30px auto;
    text-align: center;
  }

  .bold {
    font-weight: 800;
    text-shadow: gray 0px 1px 2px;
  }
</style>
