<script>
  import { t } from '$lib/i18n';
  import LessonEnd from './LessonEnd.svelte';
  import VideoReview from '../review/VideoReview.svelte';
  import Background from '../ui/sections/Background.svelte';
  import ScoreResults from './ScoreResults.svelte';
  import DarkSection from '../ui/sections/DarkSection.svelte';
  import LogoHeader from '../ui/header/LogoHeader.svelte';

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
  } = $props();

  /** @type {import("bouncy_instructor").DetectedStep[] | undefined} */
  let detectedSteps = $derived(detection.steps());
  let detectedPoses = $derived(detection.poseMisses + detection.poseMatches);
  // Challenges may continue past their initial score window to let dancers
  // recover from misses. Include those extra attempts in the denominator so a
  // review never displays more hits than available poses.
  let maximumPoses = $derived(Math.max(maximum ?? 0, detectedPoses));
  let scoreRatio = $derived(
    maximumPoses === 0 ? 0 : detection.poseMatches / maximumPoses
  );
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
          score={detection.poseMatches}
          maximum={maximumPoses}
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
