<script>
  import { t } from '$lib/i18n';
  import Toggle from '../ui/Toggle.svelte';

  export let enableLiveAvatar = false;
  export let enableInstructorAvatar = false;
  /** @type{number} */
  export let videoOpacity = 0.5;
</script>

<div id="control-panel">
  <div class="toggle-items">
    <div class="toggle-item">
      <span class="material-symbols-outlined tracking">directions_walk</span>
      <div>{$t('record.settings.enable-tracking')}</div>
      <Toggle bind:isOn={enableLiveAvatar} />
    </div>
    <div class="toggle-item">
      <span class="material-symbols-outlined instructor">directions_walk</span>
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

  .toggle-item span {
    font-size: 70px;
  }

  .tracking {
    color: var(--theme-neutral-dark);
  }
  .instructor {
    color: var(--theme-main);
  }
</style>
