<script>
  import { getContext } from 'svelte';
  import DanceStats from './DanceStats.svelte';
  import { t } from '$lib/i18n';

  const user = getContext('user').store;
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

<style>
  .profile-pic {
    display: grid;
    grid-template-columns: max-content;
    justify-content: center;
    font-weight: 900;
  }
</style>
