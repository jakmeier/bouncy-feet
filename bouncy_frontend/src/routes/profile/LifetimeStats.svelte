<script>
  import { t, locale, dateLocale } from '$lib/i18n';
  import { formatDistance } from 'date-fns';
  import { getContext } from 'svelte';

  /** @type {UserContextData} */
  let { store: user } = getContext('user');

  let steps = $derived($user.recordedSteps);
  let seconds = $derived($user.recordedSeconds);
  let activities = $derived($user.recordedDances);
  // let uniqueSteps = 7; // TODO

  const formatOpts = $derived(dateLocale($locale));

  function num(n) {
    return n.toLocaleString($locale);
  }
</script>

<h1>{$t('stats.lifetime-stats-title')}</h1>

<div class="block">
  <div>{$t('stats.lifetime-stats-subtitle')}</div>
  <div>{num(steps)} {$t('stats.num-steps')}</div>
  <div>{num(activities)} {$t('stats.num-lessons')}</div>
  <div>
    {formatDistance(0, seconds * 1000, formatOpts)}
    {$t('stats.time-danced-2')}
  </div>
</div>

<!-- <div class="block">
  <div>{$t('stats.learned-stats-title')}</div>
  <div>{num(uniqueSteps)} {$t('stats.unique-steps')}</div>
</div> -->

<div>{$t('stats.fun-stats-title')}</div>

<style>
  .block {
    margin-bottom: 1rem;
  }
</style>
