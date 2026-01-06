<script>
  import { t } from '$lib/i18n';

  /**
   * @typedef {Object} Props
   * @property {string} name -- bindable
   * @property {string} description -- bindable
   * @property {()=>void} onSubmit
   */
  /** @type {Props} */
  let { name = $bindable(), description = $bindable(), onSubmit } = $props();

  /** @param {Event} event */
  async function save(event) {
    event.preventDefault();
    onSubmit();
  }
</script>

<form onsubmit={save}>
  <label for="title"> {$t('playlist.form-title')}: </label>
  <input type="text" name="title" required maxlength="120" bind:value={name} />

  <label for="description"> {$t('playlist.form-description')}: </label>
  <textarea
    name="description"
    maxlength="1000"
    rows="5"
    cols="36"
    bind:value={description}
  ></textarea>

  <button type="submit">{$t('playlist.save-button')}</button>
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
