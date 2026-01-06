<script>
  import StepEditForm from '$lib/components/editor/StepEditForm.svelte';
  import LogoHeader from '$lib/components/ui/header/LogoHeader.svelte';
  import { t } from '$lib/i18n';

  /** @type {() => void} */
  let save = $state();
</script>

<LogoHeader
  title={$t('editor.step.new')}
  button="save"
  onAction={save}
  backButton
/>

<StepEditForm bind:save></StepEditForm>
