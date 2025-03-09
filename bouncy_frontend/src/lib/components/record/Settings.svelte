<script>
  import { run } from 'svelte/legacy';

  import { t } from '$lib/i18n';

  
  /**
   * @typedef {Object} Props
   * @property {import("bouncy_instructor").Tracker} tracker
   * @property {boolean} [expand]
   */

  /** @type {Props} */
  let { tracker, expand = $bindable(false) } = $props();

  let bpm = $state(120);
  let double = $state(false);

  run(() => {
    tracker.setBpm(bpm * (double ? 2 : 1));
  });

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
    onclick={expandSettings}
    onkeydown={expandSettings}
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
