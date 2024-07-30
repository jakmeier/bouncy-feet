<script>
  import Header from '$lib/components/ui/Header.svelte';
  import Toggle from '$lib/components/ui/Toggle.svelte';
  import { t } from '$lib/i18n';
  import { getContext } from 'svelte';

  const user = getContext('user').store;
</script>

<Header title={$t('profile.settings.title')} />

<div id="control-panel">
  <div class="toggle-item">
    <span class="material-symbols-outlined">sports_score</span>
    <Toggle bind:isOn={$user.consentSendingStats} />
    <div>{$t('profile.settings.consent-title')}</div>
  </div>
  <div class="toggle-item">
    <span class="material-symbols-outlined">science</span>
    <Toggle bind:isOn={$user.experimentalFeatures} />
    <div>{$t('profile.settings.experimental-title')}</div>
  </div>
</div>

<style>
  #control-panel {
    display: grid;
    width: 100%;
    gap: 25px;
    margin-top: 35px;
  }

  .toggle-item {
    display: grid;
    grid-template-columns: 1fr 1fr 4fr;
    gap: 0.5rem;
    align-items: center;
  }

  .toggle-item span {
    font-size: 45px;
  }
</style>
