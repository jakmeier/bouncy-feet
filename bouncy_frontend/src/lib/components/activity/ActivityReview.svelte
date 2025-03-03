<script>
  import { t } from '$lib/i18n';
  import LessonEnd from '../../../routes/courses/[courseId]/exercise/[lessonIndex]/record/LessonEnd.svelte';
  import LessonEndResults from '../../../routes/courses/[courseId]/exercise/[lessonIndex]/record/LessonEndResults.svelte';
  import VideoReview from '../review/VideoReview.svelte';
  import Button from '../ui/Button.svelte';

  let { detection, videoUrl, recordingStart, recordingEnd, onRestart, onBack } =
    $props();

  /** @type {import("$lib/instructor/bouncy_instructor").DetectedStep[] | undefined} */
  let detectedSteps = $derived(detection.steps());
  let hitRate = $derived(
    detection.poseMatches / (detection.poseMisses + detection.poseMatches)
  );
  let passed = $derived(hitRate >= 0.6);

  let showResults = $state(false);
</script>

{#if !showResults}
  <LessonEnd bind:showResults></LessonEnd>
{:else}
  <LessonEndResults {hitRate} {passed}></LessonEndResults>

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

  <div class="buttons">
    <Button
      class="wide"
      onclick={onRestart}
      symbol=""
      text="courses.end.again-button"
    ></Button>
    <Button
      class="wide"
      onclick={onBack}
      symbol=""
      text="courses.end.back-button"
    ></Button>
  </div>
{/if}

<style>
  .buttons {
    display: flex;
    flex-direction: column;
    margin-top: 10px;
    gap: 1rem;
  }
  .no-review {
    margin: 2rem;
  }
</style>
