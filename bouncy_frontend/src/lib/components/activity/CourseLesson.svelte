<script>
  import { getContext, onDestroy } from 'svelte';
  import CourseLessonPreview from './CourseLessonPreview.svelte';
  import LiveActivity from './LiveActivity.svelte';
  import StandardPage from '../ui/StandardPage.svelte';
  import { DetectionResult } from '$lib/instructor/bouncy_instructor';
  import ActivityReview from './ActivityReview.svelte';
  import { registerTracker } from '$lib/stores/Beat';
  import { getUserContext } from '$lib/context';

  /**
   * @typedef {Object} Props
   * @property {string} courseId
   * @property {number} lessonIndex
   * @property {function} onDone
   * @property {()=>void} onBack
   */

  /** @type {Props}*/
  let { courseId, lessonIndex, onDone, onBack } = $props();

  const { getCourse } = getContext('courses');
  /** @type {UserContextData} */
  const user = getUserContext();

  /** @type {import('$lib/instructor/bouncy_instructor').Course} */
  let course = getCourse(courseId);
  let previewDone = $state(false);
  let trackingDone = $state(false);
  let liveActivity = $state();
  let teacherVideo = $derived(course.lessons[lessonIndex].frontVideo?.path());

  let detection = $state();
  let videoUrl = $state();
  let recordingStart = $state(0);
  let recordingEnd = $state(0);

  let tracker = course.tracker(lessonIndex);
  let unregisterTracker = registerTracker(tracker);

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

    const fullId = courseId;
    const limitedId = fullId.slice(0, 128);
    const sessionResult = user.submitCourseLesson(
      limitedId,
      lessonIndex,
      detection
    );
    if (sessionResult) {
      user.addDanceToStats(sessionResult);
    }
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
    unregisterTracker();
  });
</script>

{#if !previewDone}
  {#if course}
    <CourseLessonPreview
      {course}
      {lessonIndex}
      onDone={() => (previewDone = true)}
      {onBack}
    ></CourseLessonPreview>
  {:else}
    <StandardPage white><h3>bug: course missing</h3></StandardPage>
  {/if}
{:else if !trackingDone}
  <LiveActivity
    bind:this={liveActivity}
    onDone={onRecordingStopped}
    {teacherVideo}
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
