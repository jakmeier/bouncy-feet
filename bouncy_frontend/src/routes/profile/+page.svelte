<script>
  import { getContext } from 'svelte';
  import DanceStats from './DanceStats.svelte';
  import { t } from '$lib/i18n';
  import { submitStats, fetchLeaderboard } from '$lib/stats';
  import Leaderboard from './Leaderboard.svelte';
  import Popup from '$lib/components/ui/Popup.svelte';
  import { writable } from 'svelte/store';

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
</script>

<div class="profile-pic">
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

<Popup title={$t('profile.consent.title')} bind:isOpen={showStatsSharingPopup}>
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
</style>
