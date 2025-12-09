<script>
  import { formatDistanceStrict } from 'date-fns';
  import { t } from '$lib/i18n';

  /**
   * @typedef {Object} Props
   * @property {number} [seconds]
   * @property {number} [numDances]
   * @property {number} [numSteps]
   * @property {number} [veryFast]
   * @property {number} [fast]
   * @property {number} [mid]
   * @property {number} [slow]
   * @property {number} [averageBpm]
   */

  /** @type {Props} */
  let {
    seconds = 0,
    numDances = 0,
    numSteps = 0,
    veryFast = 0,
    fast = 0,
    mid = 0,
    slow = 0,
    averageBpm = 0,
  } = $props();
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
      {formatDistanceStrict(0, seconds * 1000)}
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
  }
  .right {
    text-align: right;
  }
</style>
