<script>
  import { page } from '$app/stores';
  import { t } from '$lib/i18n.js';
  import { getContext, setContext, tick } from 'svelte';
  import LiveRecording from '$lib/components/record/LiveRecording.svelte';
  import { Tracker } from '$lib/instructor/bouncy_instructor';

  const { getCourse } = getContext('courses');

  let id;
  /** @type {import('$lib/instructor/bouncy_instructor').Course } */
  let course;
  /** @type {number} */
  let lessonIndex;
  /** @type { import('$lib/instructor/bouncy_instructor').Lesson } */
  let lesson;
  /** @type {number | undefined} */
  let partIndex;
  /** @type {number | undefined} */
  let recordingStart = undefined;
  /** @type {number | undefined} */
  let recordingEnd = undefined;

  /** @type {() => any}*/
  let startCamera;
  /** @type {() => any}*/
  let startRecording;
  /** @type {() => any}*/
  let stopCamera;
  /** @type {() => any}*/
  let endRecording;
  /** @type {Tracker | undefined} */
  let tracker;

  let live = false;

  let bpm = 132;

  async function start() {
    live = true;
    await tick();
    startCamera();
    // await startRecording();
  }

  function stop() {
    // await startRecording();
    stopCamera();
    tracker?.clear();
    live = false;
  }

  function loadCourse() {
    id = $page.params.courseId;
    course = getCourse(id);
    lessonIndex = Number.parseInt($page.params.lessonIndex);
    lesson = course.lessons[lessonIndex];
    partIndex = undefined;

    tracker = course.tracker(lessonIndex);
    if (tracker) {
      tracker.setBpm(bpm);
      setContext('tracker', { tracker });
    } else {
      console.error('could not construct tracker for lesson');
    }
  }
  loadCourse();
</script>

<div class="outer">
  {#if !live}
    <p>TODO: tap to select bpm and sync to beat</p>
    <button class="light" on:click={start}>
      <span class="material-symbols-outlined button"> play_arrow </span>
      <p>{$t('courses.lesson.start-button')}</p>
    </button>
  {:else}
    <LiveRecording
      bind:startCamera
      bind:stopCamera
      bind:startRecording
      bind:endRecording
      bind:recordingStart
      bind:recordingEnd
      videoOpacity={0.5}
      enableLiveAvatar={true}
      enableInstructorAvatar={false}
      slowInstructor={false}
    ></LiveRecording>

    <button class="light" on:click={stop}>
      <span class="material-symbols-outlined"> camera </span>
      <p>{$t('courses.lesson.stop-button')}</p>
    </button>
  {/if}
</div>

<style>
  .outer {
    text-align: center;
  }
  button {
    width: 152px;
    height: 80px;
    margin: 10px;
  }
  button span {
    font-size: 42px;
  }
</style>
