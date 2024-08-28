<script>
  import { getContext } from 'svelte';
  import DanceStats from './DanceStats.svelte';
  import { t } from '$lib/i18n';
  import { submitStats, fetchLeaderboard } from '$lib/stats';
  import Leaderboard from './Leaderboard.svelte';
  import Popup from '$lib/components/ui/Popup.svelte';
  import { writable } from 'svelte/store';
  import Header from '$lib/components/ui/Header.svelte';
  import { goto } from '$app/navigation';
  import Toggle from '$lib/components/ui/Toggle.svelte';
  import { dev, displayedVersion } from '$lib/stores/FeatureSelection';

  const user = getContext('user').store;
  let scoreboardData = [];
  let showStatsSharingPopup = writable(!$user.consentSendingStats);

  async function submit() {
    await submitStats($user);
    refreshLeaderboard();
  }

  function consent(yes) {
    if (yes === true) {
      $user.consentSendingStats = true;
    }
    $showStatsSharingPopup = false;
  }

  async function refreshLeaderboard() {
    let result = await fetchLeaderboard();
    if (result) {
      scoreboardData = result;
    }
  }
  refreshLeaderboard();

  function openSettings() {
    goto('./settings');
  }

  let unlockHiddenFeatures = 0;
  let lastIncrease = Date.now();
  function clickProfile() {
    if (lastIncrease + 500 > Date.now()) {
      unlockHiddenFeatures += 1;
    } else {
      unlockHiddenFeatures = 0;
    }
    lastIncrease = Date.now();
  }
  let devFeaturesOn = $dev;
  let version005 = false;
  let standardVersion = $displayedVersion;
  $: devFeaturesOn ? $dev || window.toggleDev() : $dev && window.toggleDev();
  $: version005, displayedVersion.set(version005 ? 0.005 : standardVersion);
</script>

<Header
  title={$t('profile.title')}
  backButton={false}
  button="menu"
  on:click={openSettings}
/>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="profile-pic" on:click={clickProfile}>
  <span class="material-symbols-outlined" style="font-size:100px">
    person
  </span>
  {$user.publicName}
</div>
<h2 class="box">{$t('profile.stats-title')}</h2>
<DanceStats
  seconds={$user.recordedSeconds}
  numSteps={$user.recordedSteps}
  numDances={$user.recordedDances}
/>

<h2 class="box">{$t('profile.leaderboard-title')}</h2>
<Leaderboard users={scoreboardData} />
<form class="inputs">
  <button on:click={submit} class="light">{$t('profile.submit-stats')}</button>
</form>
<form class="inputs">
  <label for="publicName">{$t('profile.public-name')}</label>
  <input id="publicName" type="text" bind:value={$user.publicName} />
</form>

<Popup title={'profile.consent.title'} bind:isOpen={showStatsSharingPopup}>
  <div>{$t('profile.consent.text0')}</div>
  <div>{$t('profile.consent.question')}</div>

  <div class="buttons">
    <button class="light" on:click={() => consent(true)}>
      <p>{$t('profile.consent.yes')}</p>
    </button>

    <button class="light" on:click={() => consent(false)}>
      <p>{$t('profile.consent.no')}</p>
    </button>
  </div>
</Popup>

{#if unlockHiddenFeatures > 6}
  <div class="hidden-features">
    <div>Dev Features</div>
    <Toggle bind:isOn={devFeaturesOn} />
    <div>Version 0.005</div>
    <Toggle bind:isOn={version005} />
  </div>
{/if}

<style>
  .profile-pic {
    display: grid;
    grid-template-columns: max-content;
    justify-content: center;
    font-weight: 900;
    text-align: center;
  }

  .inputs {
    /* display grid allows to fit input field to width */
    display: grid;
    gap: 5px;
    margin: 25px 5px;
  }

  .buttons {
    display: grid;
    grid-template-columns: auto auto;
    gap: 30px;
  }

  .hidden-features {
    background-color: var(--theme-neutral-dark);
    color: var(--theme-neutral-white);
    border-radius: 10px;
    padding: 10px;
    text-align: center;
    display: grid;
    grid-template-columns: 1fr 1fr;
    align-items: center;
    gap: 10px;
    margin: 5px;
  }
</style>
