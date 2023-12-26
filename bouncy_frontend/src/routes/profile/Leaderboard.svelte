<script>
  import { t } from '$lib/i18n';
  /** @type {{name: string, steps: number}[]} */
  export let users = [];
</script>

<div class="container">
  <div class="header">#</div>
  <div class="header">{$t('leaderboard.user-header')}</div>
  <div class="header">{$t('leaderboard.steps-header')}</div>
  {#each users as user, i}
    <div>{i + 1}.</div>
    <div>{user.name}</div>
    <div>{user.steps}</div>
  {/each}
</div>

<style>
  .container {
    display: grid;
    grid-template-columns: 1fr 6fr 3fr;
  }
  .header {
    font-family: 'Walkway Oblique';
    font-size: smaller;
    opacity: 0.5;
  }
</style>
