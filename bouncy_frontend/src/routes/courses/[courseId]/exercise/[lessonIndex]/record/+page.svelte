<script>
  import { page } from '$app/stores';
  import { t } from '$lib/i18n.js';
  import { getContext, setContext, tick } from 'svelte';
  import LiveRecording from '$lib/components/record/LiveRecording.svelte';
  import { DetectionState, Tracker } from '$lib/instructor/bouncy_instructor';
  import BeatSelector from '$lib/components/record/BeatSelector.svelte';
  import Popup from '$lib/components/ui/Popup.svelte';
  import Header from '$lib/components/ui/Header.svelte';

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
    <Header backButton title={''}></Header>
    <p>done</p>
    <!-- TODO: clean this up -->
    <p>{tracker.lastDetection.poseMatches} hits</p>
    <p>{tracker.lastDetection.poseMisses} misses</p>
    <p>{tracker.trackedBeats} tracked beats</p>

    <!-- show how many i got right anf how much i got wrong -->
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
