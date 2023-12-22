<script>
  import { t } from '$lib/i18n';

  /** @type {import("$lib/instructor/bouncy_instructor").Tracker} */
  export let tracker;

  let bpm = 120;
  let double = false;

  $: tracker.setBpm(bpm * (double ? 2 : 1));
</script>

<label>
  <input type="number" bind:value={bpm} min="60" max="180" class="number" />
  <input type="range" bind:value={bpm} min="60" max="180" />
  {$t('record.settings.bpm')}
</label>

<label>
  <input type="checkbox" bind:checked={double} />
  {$t('record.settings.double-speed')}
</label>

<style>
  label {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }
  label input {
    margin: 10px 0;
    padding: 0.5em;
  }
  .number {
    width: 75px;
  }
  input[type='checkbox'] {
    width: 2em;
    height: 2em;
  }
</style>
