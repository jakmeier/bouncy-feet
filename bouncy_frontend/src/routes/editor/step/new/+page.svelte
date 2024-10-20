<script>
  import StepEditForm from '$lib/components/editor/StepEditForm.svelte';
  import Header from '$lib/components/ui/Header.svelte';
  import { t } from '$lib/i18n';

  /** @type {() => void} */
  let save;
</script>

<Header title={$t('editor.step.new')} button="save" on:click={save}></Header>

<StepEditForm bind:save></StepEditForm>
