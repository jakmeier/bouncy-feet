<script>
  import { formatDuration, intervalToDuration } from 'date-fns';
  import { t } from '$lib/i18n';

  export let seconds = 0;
  export let numDances = 0;
  export let numSteps = 0;
</script>

<div id="container">
  <div>{$t('stats.num-steps')}</div>
  <div class=right>{numSteps}</div>

  {#if numDances >= 1}
    <div>{$t('stats.num-dances')}</div>
    <div class=right>{numDances}</div>
  {/if}

  {#if seconds > 0}
    <div>{$t('stats.time-danced')}</div>
    <div class=right>
      <!-- TODO: use locale for duration formatting -->
      {formatDuration(intervalToDuration({ start: 0, end: seconds }))}
    </div>
  {/if}
</div>

<style>
  #container {
    display: grid;
    grid-template-columns: 1fr 1fr;
    margin: 10px;
  }
  .right {
    text-align: right;
  }
</style>
