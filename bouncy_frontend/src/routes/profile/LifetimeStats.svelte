<script>
  import { t, locale, dateLocale } from '$lib/i18n';
  import { formatDistance } from 'date-fns';

  let steps = 6876;
  let seconds = 918;
  let activities = 52;
  let uniqueSteps = 7;

  const formatOpts = $derived(dateLocale($locale));

  function num(n) {
    return n.toLocaleString($locale);
  }
</script>

<h3>{$t('stats.lifetime-stats-title')}</h3>
<div class="big">{num(steps)} {$t('stats.num-steps')}</div>
<div class="big">{num(activities)} {$t('stats.num-lessons')}</div>

<h3>{$t('stats.learned-stats-title')}</h3>
<div class="big">{num(uniqueSteps)} {$t('stats.unique-steps')}</div>

<h3>{$t('stats.fun-stats-title')}</h3>

<div class="big">
  {formatDistance(0, seconds * 1000, formatOpts)}
  {$t('stats.time-danced-2')}
</div>

<style>
  .big {
    font-size: var(--font-large);
    /* margin-bottom: 1rem; */
  }
</style>
