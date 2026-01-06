<script>
  import StepEditForm from '$lib/components/editor/StepEditForm.svelte';
  import SingleActionHeader from '$lib/components/ui/header/SingleActionHeader.svelte';
  import { t } from '$lib/i18n';

  /** @type {() => void} */
  let save = $state();
</script>

<SingleActionHeader
  title={$t('editor.step.new')}
  button="save"
  onAction={save}
/>

<StepEditForm bind:save></StepEditForm>
