<script>
  import { t } from '$lib/i18n';
  import Toggle from '../ui/Toggle.svelte';

  /**
   * @typedef {Object} Props
   * @property {ComboInfo} details bindable
   * @property {boolean} dirty bindable
   */
  /** @type {Props} */
  let { details = $bindable(), dirty = $bindable(false) } = $props();

  let isPublic = $derived(!details.is_private);
  $effect(() => {
    if (details.is_private !== !isPublic) {
      dirty = true;
      details.is_private = !isPublic;
    }
  });
</script>

<form oninput={() => (dirty = true)}>
  <label for="title"> {$t('profile.combo.form-name')} </label>
  <input
    type="text"
    name="title"
    required
    maxlength="64"
    bind:value={details.title}
  />

  <label for="isPublic"> {$t('profile.combo.public-toggle')}? </label>
  <Toggle name="isPublic" bind:isOn={isPublic} />

  <!-- TODO: sort order -->
  <!-- TODO: free_form_category -->
</form>

<style>
  form {
    display: grid;
    gap: 1rem;
    grid-template-columns: 1fr;
  }

  @media (min-width: 730px) {
    form {
      grid-template-columns: min-content auto;
    }
  }
</style>
