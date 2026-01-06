<script>
  import { page } from '$app/state';
  import StepEditForm from '$lib/components/editor/StepEditForm.svelte';
  import SingleActionHeader from '$lib/components/ui/header/SingleActionHeader.svelte';
  import { t } from '$lib/i18n';
  import { StepWrapper } from '$lib/instructor/bouncy_instructor';
  import { getContext, onMount } from 'svelte';

  let stepId = page.params.stepId;
  const localCollectionCtx = getContext('localCollection');
  const steps = localCollectionCtx.steps;

  /** @type {() => void} */
  let save = $state();
  /** @type {(loadedStep: StepWrapper) => void} */
  let loadStep = $state();

  onMount(() => {
    const step = $steps.find(
      (/** @type {StepWrapper} */ step) => step.id === stepId
    );
    if (step) {
      loadStep(step);
    } else {
      console.error('step', stepId, 'not found');
    }
  });
</script>

<SingleActionHeader
  title={$t('editor.step.edit')}
  button="save"
  onAction={save}
/>

<StepEditForm bind:save bind:loadStep></StepEditForm>
