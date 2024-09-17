<script>
  import { t } from '$lib/i18n';

  /** @type {import("$lib/instructor/bouncy_instructor").Tracker} */
  export let tracker;
  export let expand = false;

  let bpm = 120;
  let double = false;

  $: tracker.setBpm(bpm * (double ? 2 : 1));

  function expandSettings(event) {
    if (
      event.type === 'click' ||
      (event.type === 'keydown' && (event.key === 'Enter' || event.key === ' '))
    ) {
      expand = !expand;
    }
  }
</script>

<h2>
  {$t('record.settings.title')}:
  {bpm}
  {$t('record.settings.bpm-unit')}
  <span
    translate="no"
    class="material-symbols-outlined"
    role="button"
    tabindex="0"
    on:click={expandSettings}
    on:keydown={expandSettings}
    aria-expanded={expand}
  >
    edit
  </span>
</h2>

{#if expand}
  <p>{$t('record.settings.description')}</p>

  <label>
    <input type="number" bind:value={bpm} min="60" max="180" class="number" />
    <input type="range" bind:value={bpm} min="60" max="180" class="range" />
    {$t('record.settings.bpm-setting')}
  </label>

  <label>
    <input type="checkbox" bind:checked={double} />
    {$t('record.settings.double-speed')}
  </label>
{/if}

<style>
  h2 {
    text-align: center;
  }
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
    width: min(60px, 20vw);
  }
  .range {
    width: min(200px, 30vw);
  }
  input[type='checkbox'] {
    width: 2em;
    height: 2em;
  }
</style>
