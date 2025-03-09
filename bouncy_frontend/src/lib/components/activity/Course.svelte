<script>
  import { getContext, onDestroy } from 'svelte';
  import CourseLessonPreview from './CourseLessonPreview.svelte';
  import LiveActivity from './LiveActivity.svelte';
  import StandardPage from '../ui/StandardPage.svelte';
  import { DetectionResult } from 'bouncy_instructor';
  import ActivityReview from './ActivityReview.svelte';
  import { registerTracker } from '$lib/stores/Beat';

  /**
   * @typedef {Object} Props
   * @property {string} courseId
   * @property {function} onDone
   */

  /** @type {Props}*/
  let { courseId, onDone } = $props();

  const { getCourse } = getContext('courses');

  let course = $derived(getCourse(courseId));
  let lessonIndex = $state(0);
  let previewDone = $state(false);
  let trackingDone = $state(false);
  let liveActivity = $state();

  let detection = $state();
  let videoUrl = $state();
  let recordingStart = $state(0);
  let recordingEnd = $state(0);

  let tracker = $derived(course.tracker(lessonIndex));
  $effect(() => registerTracker(tracker));

  /**
   * @param {DetectionResult} newDetection
   * @param {number} newRecordingStart
   * @param {number} newRecordingEnd
   * @param {string} newVideoUrl
   */
  function onRecordingStopped(
    newDetection,
    newRecordingStart,
    newRecordingEnd,
    newVideoUrl
  ) {
    detection = newDetection;

    recordingStart = newRecordingStart;
    recordingEnd = newRecordingEnd;

    videoUrl = newVideoUrl;

    trackingDone = true;
  }

  function onLeaveReview() {
    onDone();
  }

  function onRestart() {
    previewDone = false;
    if (videoUrl) {
      URL.revokeObjectURL(videoUrl);
    }
    videoUrl = '';
    detection = null;
    if (liveActivity) {
      liveActivity.clear();
    }
    tracker.clear();
    trackingDone = false;
  }

  onDestroy(() => {
    if (videoUrl) {
      URL.revokeObjectURL(videoUrl);
      videoUrl = '';
    }
  });
</script>

{#if !previewDone}
  {#if course}
    <CourseLessonPreview
      {course}
      {lessonIndex}
      onDone={() => (previewDone = true)}
    ></CourseLessonPreview>
  {:else}
    <StandardPage white><h3>bug: course missing</h3></StandardPage>
  {/if}
{:else if !trackingDone}
  <LiveActivity
    bind:this={liveActivity}
    onDone={onRecordingStopped}
    videoOpacity={1.0}
    enableLiveAvatar={false}
    enableInstructorAvatar={true}
  ></LiveActivity>
{:else if detection !== undefined}
  <ActivityReview
    {detection}
    {recordingStart}
    {recordingEnd}
    {videoUrl}
    onBack={onLeaveReview}
    {onRestart}
  ></ActivityReview>
{:else}
  <StandardPage white><h3>bug: missing detection</h3></StandardPage>
{/if}
