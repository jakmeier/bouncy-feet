<script>
  import { t } from '$lib/i18n';
  import Toggle from '../ui/Toggle.svelte';

  /**
   * @typedef {Object} Props
   * @property {UserSearchConfig} searchConfig --bindable
   */

  /** @type {Props} */
  let { searchConfig = $bindable() } = $props();
</script>

<div class="top-control">
  <div class="column">
    <label for="search"> {$t('users.search-label')} </label>
    <input id="search" name="search" bind:value={searchConfig.searchTerm} />
  </div>

  <div class="column end">
    <label for="isPublic"> {$t('users.show-guests-label')} </label>
    <div class="toggle-wrapper">
      <Toggle name="isPublic" bind:isOn={searchConfig.showGuests} />
    </div>
  </div>
</div>

<style>
  .top-control {
    position: relative;
    display: grid;
    width: 100%;
    grid-template-columns: 2fr 1fr;
    align-items: center;
    gap: 2rem;
  }

  .column {
    display: flex;
    flex-direction: column;
  }

  .end {
    justify-self: end;
  }

  .toggle-wrapper {
    margin-left: auto;
  }

  input {
    width: 100%;
    max-width: 50vw;
  }
</style>
