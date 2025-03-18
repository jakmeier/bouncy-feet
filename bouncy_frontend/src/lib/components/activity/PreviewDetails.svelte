<script>
  import { t, locale, dateLocale } from '$lib/i18n';
  import { formatDuration, intervalToDuration } from 'date-fns';
  import { base } from '$app/paths';

  //   TODO: set proper difficulty / energy
  let { beats, bpm, durationMs, difficulty = 1, energy = 3 } = $props();

  /** @type {import('date-fns').FormatDurationOptions} */
  const formatOpts = $derived({
    ...dateLocale($locale),
  });
  /** @type {import('date-fns').Duration} */
  let trainingDuration = $derived(
    intervalToDuration({ start: 0, end: durationMs })
  );
</script>

<div class="overview">
  <div class="row">
    <div class="flex">
      <div>
        {beats}
        {$t('courses.lesson.num-beats-label')}
      </div>
      <div>
        @ {$bpm} bpm
      </div>
    </div>
    <div>
      {formatDuration(trainingDuration, formatOpts)}
    </div>
  </div>

  <div class="rated">
    <div>{$t('record.preview-difficulty-title')}</div>
    <div class="rating">
      {#each { length: 5 } as _, i}
        <img
          src="{base}/img/symbols/bf_eye_black.svg"
          alt="bf_eye"
          class:shadow={i >= difficulty}
        />
      {/each}
    </div>

    <div>{$t('record.preview-energy-title')}</div>
    <div class="rating">
      {#each { length: 5 } as _, i}
        <img
          src="{base}/img/symbols/bf_eye_black.svg"
          alt="bf_eye"
          class:shadow={i >= energy}
        />
      {/each}
    </div>
  </div>
</div>

<style>
  .overview img {
    height: 2rem;
  }

  .overview {
    margin: 2em 0em 3rem;
  }

  .row {
    display: grid;
    grid-template-columns: 1fr max-content;
    gap: 1.5rem;
  }

  .rated {
    margin: 3rem 0;
    display: grid;
    grid-template-columns: max-content auto;
    gap: 2rem 1rem;
    align-items: center;
    justify-content: left;
  }
  .rating {
    justify-self: left;
    display: flex;
    gap: 0.25rem;
    align-items: center;
    justify-content: end;
  }

  .flex {
    display: flex;
    flex-wrap: wrap;
    gap: 7px;
  }

  .shadow {
    opacity: 10%;
  }
</style>
