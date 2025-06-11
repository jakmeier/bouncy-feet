<script>
  import { t } from '$lib/i18n';
  import { getContext } from 'svelte';
  import StandardPage from '../ui/StandardPage.svelte';
  import Thermometer from '../ui/svg/Thermometer.svelte';

  let { detection, onContinue } = $props();

  /** @type {UserContextData} */
  let { store: user } = getContext('user');

  //   TODO: Do something specific to warmup, where the energy level is measured rather than error.
  // let energyLevel = $derived(Math.min(1.0, detection.steps().length / 100));
  let energyLevel = 0.8;
  let lifetimeSteps = $derived($user.recordedSteps);
  // TODO
  // let weeklySteps = 873;
  // let dailySteps = 77;
  let steps = $derived(detection.steps().length);

  //   TODO: real data lines
  let relativeTemp = $derived(Math.min(energyLevel, 1.0));
  let absoluteTemp = $derived((relativeTemp * 40 + 10).toFixed(0));
  let line = $t('record.warmup-line-neutral-0');
  //   TODO: add more lines
  //   TODO: translate lines
  // let line = $derived.by(() => {
  //   if (relativeTemp > 0.8) {
  //     return 'Hot!';
  //   }
  //   if (relativeTemp > 0.5) {
  //     return 'Nice warm-up!';
  //   }
  //   if (relativeTemp > 0.1) {
  //     return "It's a start!";
  //   }
  //   return 'A bit tired, are we?';
  // });
</script>

<StandardPage title={$t('record.warmup-review-title')} white>
  <h3>{$t('record.warmup-result')}</h3>
  <div class="big">{steps} {$t('stats.num-steps')}</div>
  <div class="stats">
    <!-- <div>{dailySteps} {$t('stats.today')}</div> -->
    <!-- <div>{weeklySteps} {$t('stats.week')}</div> -->
    <!-- <div>{lifetimeSteps} {$t('stats.lifetime')}</div> -->
  </div>
  <h3>{$t('record.energy-level')}</h3>
  <div class="thermo">
    <Thermometer temperature={relativeTemp}></Thermometer>
    <div>
      <div class="big temperature">{absoluteTemp}°</div>
      <h3>{line}</h3>
    </div>
  </div>
  <div class="buttons">
    <button class="wide" onclick={onContinue}>
      {$t('courses.lesson.next-button')}
    </button>
  </div>
</StandardPage>

<style>
  .buttons {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  .stats {
    margin: 1rem;
    margin-bottom: 3rem;
    font-size: var(--font-large);
    text-align: right;
  }
  .big {
    font-size: var(--font-large);
  }
  .thermo {
    display: grid;
    grid-template-columns: 1fr 2fr;
    align-items: center;
    height: 30dvh;
    margin: 3rem 0;
    /* text-align: right; */
  }
</style>
