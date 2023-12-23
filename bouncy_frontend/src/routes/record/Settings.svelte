<script>
  import { t } from '$lib/i18n';

  /** @type {import("$lib/instructor/bouncy_instructor").Tracker} */
  export let tracker;

  let bpm = 120;
  let double = false;

  $: tracker.setBpm(bpm * (double ? 2 : 1));
</script>

<h2>{$t('record.settings.title')}</h2>
<p>{$t('record.settings.description')}</p>

<label>
  <input type="number" bind:value={bpm} min="60" max="180" class="number" />
  <input type="range" bind:value={bpm} min="60" max="180" class="range"/>
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
    width: min(60px,20vw);
  }
  .range {
    width: min(200px,30vw);
  }
  input[type='checkbox'] {
    width: 2em;
    height: 2em;
  }
</style>
