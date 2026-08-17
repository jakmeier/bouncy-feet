<script>
  import { t } from '$lib/i18n';
  import LessonEnd from './LessonEnd.svelte';
  import VideoReview from '../review/VideoReview.svelte';
  import Background from '../ui/sections/Background.svelte';
  import ScoreResults from './ScoreResults.svelte';
  import DarkSection from '../ui/sections/DarkSection.svelte';
  import LogoHeader from '../ui/header/LogoHeader.svelte';
  import { detectionScore } from '$lib/activityScore.js';

  let {
    detection,
    videoUrl,
    recordingStart,
    recordingEnd,
    onRestart,
    onBack,
    maximum = undefined,
    threshold = 0.6,
    passed = undefined,
    scoreMode = 'poses',
  } = $props();

  /** @type {'poses' | 'steps'} */
  let scoringMode = $derived(scoreMode === 'steps' ? 'steps' : 'poses');
  /** @type {import("bouncy_instructor").DetectedStep[] | undefined} */
  let detectedSteps = $derived(detection.steps());
  let scoreCounters = $derived(detectionScore(detection, scoringMode));
  let detectedOpportunities = $derived(
    scoreCounters.hits + scoreCounters.misses
  );
  let score = $derived(scoreCounters.hits);
  let maximumScore = $derived(Math.max(maximum ?? 0, detectedOpportunities));
  let scoreRatio = $derived(maximumScore === 0 ? 0 : score / maximumScore);
  let didPass = $derived(passed ?? scoreRatio >= threshold);

  let showResults = $state(false);
</script>

<Background
  bgColor="var(--theme-neutral-dark)"
  color="var(--theme-neutral-white)"
></Background>

<div
  style="background-color: var(--theme-neutral-dark); color: var(--theme-neutral-white); --background-color: var(--theme-neutral-dark); --color: var(--theme-neutral-white);"
>
  {#if !showResults}
    <LessonEnd bind:showResults></LessonEnd>
  {:else}
    <DarkSection fillScreen arrow arrowText={$t('record.review-details')}>
      <LogoHeader black />

      <div class="top-summary">
        <ScoreResults
          {score}
          maximum={maximumScore}
          {threshold}
          passed={didPass}
        />
      </div>
    </DarkSection>

    {#if recordingStart !== undefined && recordingEnd !== undefined}
      <VideoReview
        reviewVideoSrc={videoUrl}
        {detectedSteps}
        reviewHint="record.local-review-hint"
        {recordingStart}
        {recordingEnd}
        mode={scoringMode}
      ></VideoReview>
    {:else}
      <div class="no-review">
        {$t('record.no-video-for-review')}
      </div>
    {/if}

    <div class="buttons">
      <button onclick={onRestart}>
        {$t('activity.review.again-button')}
      </button>
      <button onclick={onBack}> {$t('activity.review.done-button')}</button>
    </div>
  {/if}
</div>

<style>
  .buttons {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin-top: 2rem;
  }
  .no-review {
    margin: 2rem;
  }
  .top-summary {
    display: flex;
    flex-direction: column;
    justify-content: space-around;
  }
</style>
