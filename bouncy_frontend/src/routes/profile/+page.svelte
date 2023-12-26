<script>
  import { getContext } from 'svelte';
  import DanceStats from './DanceStats.svelte';
  import { t } from '$lib/i18n';
  import { submitStats, fetchLeaderboard } from '$lib/stats';
  import Leaderboard from './Leaderboard.svelte';

  const user = getContext('user').store;
  let scoreboardData = [];

  async function submit() {
    await submitStats($user);
    refreshLeaderboard();
  }

  async function refreshLeaderboard() {
    let result = await fetchLeaderboard();
    if(result) {
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
<h2>{$t('profile.stats-title')}</h2>
<DanceStats
  seconds={$user.recordedSeconds}
  numSteps={$user.recordedSteps}
  numDances={$user.recordedDances}
/>

<label>
  <input type="text" bind:value={$user.publicName} />
</label>
<button on:click={submit}>{$t('profile.submit-stats')}</button>

<h2>{$t('profile.leaderboard-title')}</h2>
<Leaderboard users={scoreboardData} />

<style>
  .profile-pic {
    display: grid;
    grid-template-columns: max-content;
    justify-content: center;
    font-weight: 900;
    text-align: center;
  }
</style>
