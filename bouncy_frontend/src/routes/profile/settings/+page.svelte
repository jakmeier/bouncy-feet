<script>
  import Button from '$lib/components/ui/Button.svelte';
  import Symbol from '$lib/components/ui/Symbol.svelte';
  import Toggle from '$lib/components/ui/Toggle.svelte';
  import { locale, t } from '$lib/i18n';
  import {
    audioDelay,
    mediapipeDelayLastValue,
    mediapipeDelayNum,
    mediapipeDelayTotal,
    resetSystemStats,
    trackSyncDelayLastValue,
    trackSyncDelayNum,
    trackSyncDelayTotal,
    detectionDelayLastValue,
    detectionDelayNum,
    detectionDelayTotal,
  } from '$lib/stores/System';
  import { getUserContext } from '$lib/context';
  import LogoHeader from '$lib/components/ui/LogoHeader.svelte';

  const user = getUserContext().store;

  // format number
  function fnum(num) {
    return num.toFixed(2);
  }
</script>

<LogoHeader title={$t('profile.settings.title')} backButton />

<div id="control-panel">
  <div class="toggle-item">
    <Symbol size={45}>sports_score</Symbol>
    <Toggle bind:isOn={$user.consentSendingStats} />
    <div>{$t('profile.settings.consent-title')}</div>
  </div>
  <div class="toggle-item">
    <Symbol size={45}>science</Symbol>
    <Toggle bind:isOn={$user.experimentalFeatures} />
    <div>{$t('profile.settings.experimental-title')}</div>
  </div>
</div>

{#if $user.experimentalFeatures}
  <div class="system-infos">
    <Symbol size={45}>translate</Symbol>
    <div>{$locale}</div>

    <div>
      <Symbol size={45}>manufacturing</Symbol>
      <Button
        class="reset width60"
        text=""
        symbol="close"
        symbolSize={28}
        symbolClass="thin"
        on:click={resetSystemStats}
      ></Button>
    </div>

    <div>
      <h3>System Info</h3>
      <div class="system-info">
        <div>Audio delay</div>
        <div>{fnum($audioDelay)}ms</div>
      </div>

      <h3>Averages</h3>
      <div class="system-info">
        <div>MP delay</div>
        <div>{fnum($mediapipeDelayTotal / $mediapipeDelayNum)}ms</div>
      </div>

      <div class="system-info">
        <div>track delay</div>
        <div>{fnum($trackSyncDelayTotal / $trackSyncDelayNum)}ms</div>
      </div>

      <div class="system-info">
        <div>dance analysis delay</div>
        <div>{fnum($detectionDelayTotal / $detectionDelayNum)}ms</div>
      </div>

      <h3>Last Values</h3>

      <div class="system-info">
        <div>MP delay</div>
        <div>{fnum($mediapipeDelayLastValue)}ms</div>
      </div>

      <div class="system-info">
        <div>track delay</div>
        <div>{fnum($trackSyncDelayLastValue)}ms</div>
      </div>

      <div class="system-info">
        <div>dance analysis delay</div>
        <div>{fnum($detectionDelayLastValue)}ms</div>
      </div>
    </div>
  </div>
{/if}

<style>
  h3 {
    font-weight: 800;
    padding: 0;
    margin: 10px 0 0;
  }

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

  .system-infos {
    display: grid;
    grid-template-columns: 1fr 5fr;
    align-items: center;
    padding: 0.5rem 5px;
    gap: 0.5rem;
    font-size: var(--font-normal);
  }

  .system-info {
    display: grid;
    grid-template-columns: 2fr 1fr;
    margin: 10px 0;
    padding-right: 5px;
  }
  .system-info :nth-child(1) {
    justify-self: start;
  }
  .system-info :nth-child(2) {
    justify-self: end;
  }
</style>
