<script>
  import { t } from '$lib/i18n';
  import Toggle from '../ui/Toggle.svelte';

  /**
   * @typedef {Object} Props
   * @property {ComboInfo} details bindable
   * @property {boolean} dirty bindable (todo: would be nice to track what is dirty, to avoid updating all values which requires multiple API calls)
   * @property {Beat} [beat] bindable
   */
  /** @type {Props} */
  let {
    details = $bindable(),
    dirty = $bindable(false),
    beat = $bindable(),
  } = $props();

  let isPublic = $derived(!details.is_private);
  $effect(() => {
    if (details.is_private !== !isPublic) {
      dirty = true;
      details.is_private = !isPublic;
    }
  });
  let halfSpeed = $derived(beat?.subbeat_per_move === 2);

  /** @param {Event} event */
  function inputChanged(event) {
    dirty = true;
  }

  function halfSpeedChanged() {
    if (beat) {
      beat.subbeat_per_move = halfSpeed ? 2 : 1;
    }
  }
</script>

<form oninput={inputChanged}>
  {#if beat}
    <div class="beat-subform">
      <label for="bpm"> {$t('editor.video.music-bpm')}</label>
      <input type="number" name="bpm" bind:value={beat.bpm} />

      <label for="halfSpeed"> {$t('editor.video.half-speed')}? </label>
      <Toggle
        name="halfSpeed"
        bind:isOn={halfSpeed}
        onInput={halfSpeedChanged}
      />

      <label for="offset"> {$t('editor.video.beat-offset')}</label>
      <input type="number" name="offset" bind:value={beat.start} />
    </div>
  {/if}

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

  .beat-subform {
    display: grid;
    gap: 1rem;
    grid-template-columns: auto auto;
    grid-column-start: 1;
    grid-column-end: 3;
    justify-items: right;

    background-color: var(--theme-neutral-light);
    padding: 0.5rem;
    margin: 0 -0.5rem;
    border-radius: 0.5rem;
  }
  .beat-subform label {
    justify-self: left;
  }

  @media (min-width: 730px) {
    form {
      grid-template-columns: min-content auto;
    }
  }
</style>
