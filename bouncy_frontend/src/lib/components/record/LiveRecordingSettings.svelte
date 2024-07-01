<script>
  import { t } from '$lib/i18n';
  import Toggle from '../ui/Toggle.svelte';

  export let enableLiveAvatar = false;
  export let enableInstructorAvatar = false;
</script>

<div id="control-panel">
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

<style>
  #control-panel {
    display: flex;
    gap: 0.5rem;
    align-items: center;
    justify-content: space-around;
    width: 100%;
    height: 120px;
    padding: 5px;
    margin-top: 15px;
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
    color: var(--theme-main);
  }
  .instructor {
    color: var(--theme-accent);
  }
</style>
