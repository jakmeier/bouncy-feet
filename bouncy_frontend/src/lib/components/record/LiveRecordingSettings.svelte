<script>
  import { t } from '$lib/i18n';
  import Symbol from '../ui/Symbol.svelte';
  import Toggle from '../ui/Toggle.svelte';

  /**
   * @typedef {Object} Props
   * @property {boolean} [enableLiveAvatar]
   * @property {boolean} [enableInstructorAvatar]
   * @property {number} [videoOpacity]
   */

  /** @type {Props} */
  let {
    enableLiveAvatar = $bindable(false),
    enableInstructorAvatar = $bindable(false),
    videoOpacity = $bindable(0.5),
  } = $props();
</script>

<div id="control-panel">
  <div class="toggle-items">
    <div class="toggle-item">
      <Symbol size={70}>directions_walk</Symbol>
      <div>{$t('record.settings.enable-tracking')}</div>
      <Toggle bind:isOn={enableLiveAvatar} />
    </div>
    <div class="toggle-item">
      <Symbol size={70}>directions_walk</Symbol>
      <div>{$t('record.settings.enable-instructor')}</div>
      <Toggle bind:isOn={enableInstructorAvatar} />
    </div>
  </div>
  <div class="toggle-item">
    <div>{$t('record.settings.opacity')}</div>
    <input
      type="range"
      bind:value={videoOpacity}
      min="0.0"
      max="1.0"
      class="range"
      step="0.01"
    />
  </div>
</div>

<style>
  #control-panel {
    display: grid;
    width: 100%;
    height: 120px;
    padding: 5px;
    margin-top: 15px;
  }

  .toggle-items {
    display: flex;
    gap: 0.5rem;
    align-items: center;
    justify-content: space-around;
  }

  .toggle-item {
    display: grid;
    grid-template-columns: 1fr;
    justify-items: center;
  }
</style>
