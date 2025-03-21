<script>
  import { t } from '$lib/i18n';
  import LessonEnd from './LessonEnd.svelte';
  import VideoReview from '../review/VideoReview.svelte';
  import Arrow from '../ui/Arrow.svelte';
  import Background from '../ui/sections/Background.svelte';
  import StandardPage from '../ui/StandardPage.svelte';
  import LessonEndResults from './LessonEndResults.svelte';

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

{#if !showResults}
  <LessonEnd bind:showResults></LessonEnd>
{:else}
  <StandardPage black>
    <div class="top-summary">
      <LessonEndResults {hitRate} {passed}></LessonEndResults>
      <div class="down-marker">
        <div class="down-marker-text">{$t('record.review-details')}</div>
        <div class="down-marker-arrow">
          <Arrow color="var(--theme-neutral-white)" />
        </div>
      </div>
    </div>

    {#if recordingStart !== undefined && recordingEnd !== undefined}
      <VideoReview
        reviewVideoSrc={videoUrl}
        {detectedSteps}
        {recordingStart}
        {recordingEnd}
      ></VideoReview>
    {:else}
      <div class="no-review">
        {$t('record.no-video-for-review')}
      </div>
    {/if}

    <div class="vspace down-marker-arrow">
      <Arrow></Arrow>
    </div>

    <div class="buttons">
      <button class="wide" onclick={onRestart}>
        {$t('courses.end.again-button')}
      </button>
      <button class="wide" onclick={onBack}>
        {$t('courses.end.back-button')}</button
      >
    </div>
  </StandardPage>
{/if}

<style>
  .buttons {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  .no-review {
    margin: 2rem;
  }
  .top-summary {
    height: 100dvh;
    display: flex;
    flex-direction: column;
    justify-content: space-around;
  }
  .down-marker {
    margin: 5rem auto 0;
  }
  .down-marker-text {
    text-align: center;
  }
  .down-marker-arrow {
    margin: auto;
    max-width: 3rem;
    max-height: 3rem;
  }
  .vspace {
    margin: 1.5rem auto;
  }
</style>
