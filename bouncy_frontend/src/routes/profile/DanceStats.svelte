<script>
  import { formatDuration, intervalToDuration } from 'date-fns';
  import { t } from '$lib/i18n';

  export let seconds = 0;
  export let numDances = 0;
  export let numSteps = 0;
</script>

<p>{$t('stats.num-steps')}: {numSteps}</p>
{#if numDances >= 1}
  <p>{$t('stats.num-dances')}: {numDances}</p>
{/if}
{#if seconds > 0}
  <p>
    <!-- TODO: use locale for duration formatting -->
    {$t('stats.time-danced')}
    {formatDuration(intervalToDuration({ start: 0, end: seconds }))}
  </p>
{/if}
