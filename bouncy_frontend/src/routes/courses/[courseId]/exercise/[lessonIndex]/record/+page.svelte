<script>
  import { page } from '$app/stores';
  import { t } from '$lib/i18n.js';
  import { getContext, setContext, tick } from 'svelte';
  import LiveRecording from '$lib/components/record/LiveRecording.svelte';
  import { DetectionState, Tracker } from '$lib/instructor/bouncy_instructor';
  import BeatSelector from '$lib/components/record/BeatSelector.svelte';
  import Popup from '$lib/components/ui/Popup.svelte';
  import LessonEnd from './LessonEnd.svelte';

  const { getCourse } = getContext('courses');
  const { recordFinishedLesson } = getContext('user');

  /** @type {string} */
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

  /** @type {number} */
  let bpmDetectionCounter;
  /** @type {number} */
  let beatStart;
  $: beatDetected = bpmDetectionCounter >= 3;
  /** @type {import('svelte/store').Writable<boolean>} */
  let showHint;

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
  /** @type {import('svelte/store').Readable<DetectionState> | null} */
  $: trackingState = tracker ? tracker.detectionState : null;

  let live = false;

  let bpm = 132;

  async function start() {
    live = true;
    await tick();
    startCamera();
  }

  function stop() {
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

  function updateBeat() {
    if (tracker) {
      tracker.setBpm(bpm);
    } else {
      console.warn('tracker not set');
    }
  }
  $: beatStart, updateBeat();

  let hitRate = 0.0;
  let passed = false;
  function trackingDone() {
    // TODO: use bpm and accuracy stats to give star rating
    const detected = tracker.lastDetection;
    hitRate =
      detected.poseMatches / (detected.poseMisses + detected.poseMatches);
    passed = hitRate > 0.6;
    if (passed) {
      recordFinishedLesson(id, lessonIndex, 1);
    }
  }
  $: if ($trackingState === DetectionState.TrackingDone) trackingDone();
</script>

<div class="outer">
  {#if !live}
    <BeatSelector
      bind:bpm
      bind:counter={bpmDetectionCounter}
      bind:lastTap={beatStart}
    ></BeatSelector>
    <button
      class={beatDetected ? 'light' : 'locked'}
      on:click={beatDetected ? start : () => showHint.set(true)}
    >
      <span class="material-symbols-outlined button"> play_arrow </span>
      <p>{$t('courses.lesson.start-button')}</p>
    </button>
  {:else if $trackingState === DetectionState.TrackingDone}
    <LessonEnd
      {hitRate}
      {passed}
      hits={tracker?.lastDetection.poseMatches}
      misses={tracker?.lastDetection.poseMisses}
    ></LessonEnd>
  {:else}
    <LiveRecording
      bind:startCamera
      bind:stopCamera
      bind:startRecording
      bind:endRecording
      bind:recordingStart
      bind:recordingEnd
      {beatStart}
      {bpm}
      videoOpacity={0.5}
      enableLiveAvatar={true}
      enableInstructorAvatar={true}
      forceBeat
    ></LiveRecording>

    <button class="light" on:click={stop}>
      <span class="material-symbols-outlined"> camera </span>
      <p>{$t('courses.lesson.stop-button')}</p>
    </button>
  {/if}
</div>

<Popup bind:isOpen={showHint} showOkButton title={'common.hint-popup-title'}>
  {$t('record.estimate-bpm-hint')}
</Popup>

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
  .locked {
    background-color: var(--theme-neutral-gray);
    color: var(--theme-neutral-dark);
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
