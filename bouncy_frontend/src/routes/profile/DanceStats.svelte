<script>
  import { formatDuration, intervalToDuration } from 'date-fns';
  import { t } from '$lib/i18n';

  export let seconds = 0;
  export let numDances = 0;
  export let numSteps = 0;
  export let veryFast = 0;
  export let fast = 0;
  export let mid = 0;
  export let slow = 0;
  export let averageBpm = 0;
</script>

<div id="container">
  <div>{$t('stats.num-steps')}</div>
  <div class="right">{numSteps}</div>

  {#if numDances >= 1}
    <div>{$t('stats.num-dances')}</div>
    <div class="right">{numDances}</div>
  {/if}

  {#if slow >= 1}
    <div>{$t('stats.slow')}</div>
    <div class="right">{slow}</div>
  {/if}

  {#if mid >= 1}
    <div>{$t('stats.mid')}</div>
    <div class="right">{mid}</div>
  {/if}

  {#if fast >= 1}
    <div>{$t('stats.fast')}</div>
    <div class="right">{fast}</div>
  {/if}

  {#if veryFast >= 1}
    <div>{$t('stats.very-fast')}</div>
    <div class="right">{veryFast}</div>
  {/if}

  {#if seconds > 0}
    <div>{$t('stats.time-danced')}</div>
    <div class="right">
      <!-- TODO: use locale for duration formatting -->
      {formatDuration(intervalToDuration({ start: 0, end: seconds * 1000 }))}
    </div>
  {/if}

  {#if averageBpm >= 1}
    <div>{$t('stats.average-bpm')}</div>
    <div class="right">{averageBpm.toFixed(0)}</div>
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
