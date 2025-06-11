<script>
  import { t } from '$lib/i18n';
  import LessonEnd from './LessonEnd.svelte';
  import VideoReview from '../review/VideoReview.svelte';
  import Background from '../ui/sections/Background.svelte';
  import LessonEndResults from './LessonEndResults.svelte';
  import NextSectionArrow from '../ui/NextSectionArrow.svelte';
  import DarkSection from '../ui/sections/DarkSection.svelte';
  import LogoHeader from '../ui/LogoHeader.svelte';

  let { detection, videoUrl, recordingStart, recordingEnd, onRestart, onBack } =
    $props();

  /** @type {import("bouncy_instructor").DetectedStep[] | undefined} */
  let detectedSteps = $derived(detection.steps());
  let hitRate = $derived(
    detection.poseMatches / (detection.poseMisses + detection.poseMatches)
  );
  let passed = $derived(hitRate >= 0.6);

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
        <LessonEndResults {hitRate} {passed}></LessonEndResults>
      </div>
    </DarkSection>

    {#if recordingStart !== undefined && recordingEnd !== undefined}
      <VideoReview
        reviewVideoSrc={videoUrl}
        {detectedSteps}
        {recordingStart}
        {recordingEnd}
      ></VideoReview>

      <div class="hint">{$t('record.no-upload-hint')}</div>
    {:else}
      <div class="no-review">
        {$t('record.no-video-for-review')}
      </div>
    {/if}

    <div class="buttons">
      <button onclick={onRestart}>
        {$t('courses.end.again-button')}
      </button>
      <button onclick={onBack}> {$t('courses.end.back-button')}</button>
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
  .hint {
    margin-top: 2rem;
  }
</style>
