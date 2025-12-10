<script>
  import { t } from '$lib/i18n';

  /**
   * @typedef {Object} Props
   * @property {string} label
   * @property {string} [initValue]
   * @property {(value: string) => void} onSelected
   */

  /** @type {Props} */
  let { label, initValue = '', onSelected } = $props();

  let currentValue = $state(initValue);

  function submit() {
    onSelected(currentValue);
  }
</script>

<form class="inputs">
  <label for="publicName">{$t(label)}</label>
  <input id="publicName" type="text" bind:value={currentValue} />
  <button onclick={submit} class="wide"
    >{$t('profile.update-name-button')}</button
  >
</form>

<style>
  form {
    display: grid;
    gap: 1rem;
    grid-template-columns: 1fr;
    justify-items: center;
  }
</style>
