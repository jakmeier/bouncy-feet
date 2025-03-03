<script>
  import { run } from 'svelte/legacy';

  import { page } from '$app/stores';
  import { t } from '$lib/i18n.js';
  import { getContext, onDestroy, onMount, tick } from 'svelte';
  import LiveRecording from '$lib/components/record/LiveRecording.svelte';
  import { DetectionState, Tracker } from '$lib/instructor/bouncy_instructor';
  import Popup from '$lib/components/ui/Popup.svelte';
  import LessonEndResults from './LessonEndResults.svelte';
  import VideoReview from '$lib/components/review/VideoReview.svelte';
  import Audio from '$lib/components/BeatAudio.svelte';
  import { registerTracker } from '$lib/stores/Beat';
  import Button from '$lib/components/ui/Button.svelte';
  import { writable } from 'svelte/store';
  import LessonEnd from './LessonEnd.svelte';
  import { dev } from '$lib/stores/FeatureSelection';
  import DevUtility from '$lib/components/dev/DevUtility.svelte';
  import LightBackground from '$lib/components/ui/sections/LightBackground.svelte';
  /**
   * @typedef {Object} Props
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let { children } = $props();

  const { getCourse } = getContext('courses');
  const { recordFinishedLesson, computeDanceStats, addDanceToStats } =
    getContext('user');

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
  let recordingStart = $state(undefined);
  /** @type {number | undefined} */
  let recordingEnd = $state(undefined);

  /** @type {number} */
  let beatStart;
  let beatDetected = false;
  /** @type {import('svelte/store').Writable<boolean>} */
  let showHint = $state();

  let detectionResult;
  /** @type {string} */
  let videoUrl = $state();
  /** @type {import("$lib/instructor/bouncy_instructor").DetectedStep[] | undefined} */
  let detectedSteps = $state();

  /** @type {() => any}*/
  let startCamera = $state();
  /** @type {() => Promise<void>}*/
  let startRecording = $state();
  /** @type {() => Promise<void>}*/
  let stopLiveRecording = $state();
  /** @type {Tracker | undefined} */
  let tracker = $state();
  /** @type {import('svelte/store').Readable<DetectionState> | null} */
  let trackingState = $derived(tracker ? tracker.detectionState : null);

  let useFixedBpm = false;
  /** @type {import('svelte/store').Writable<boolean>} */
  let startExercisePopUpIsOpen = writable(false);

  let showHintBeforeStart = true;
  // wait for a user input before showing stats
  let showResults = $state(false);

  async function start() {
    if (showHintBeforeStart) {
      // only show the hint once per sessions
      showHintBeforeStart = false;
      $startExercisePopUpIsOpen = true;
      return;
    }
    await tick();
    await startCamera();
    await startRecording();
  }

  /**
   * @param {Blob | undefined} videoBlob
   */
  async function onRecordingStopped(videoBlob) {
    tracker?.finishTracking();
    if (videoBlob) {
      if (videoUrl) {
        URL.revokeObjectURL(videoUrl);
      }
      videoUrl = URL.createObjectURL(videoBlob);
    } else {
      console.warn('ended recording and did not get video blob', videoBlob);
    }

    detectionResult = tracker?.lastDetection;
    detectedSteps = detectionResult?.steps();
  }

  function restart() {
    tracker?.clear();
    beatStart = 0;
    recordingStart = undefined;
    recordingEnd = undefined;
    showResults = false;
  }

  function goBack() {
    window.history.back();
    window.history.back();
  }

  function closeStartExercisePopUp() {
    $startExercisePopUpIsOpen = false;
    start();
  }

  function loadCourse() {
    id = $page.params.courseId;
    course = getCourse(id);
    lessonIndex = Number.parseInt($page.params.lessonIndex);
    lesson = course.lessons[lessonIndex];
    partIndex = undefined;

    if (lessonIndex >= 0) {
      tracker = course.tracker(lessonIndex);
    } else {
      tracker = course.trainingTracker();
    }
    if (tracker) {
      registerTracker(tracker);
    } else {
      console.error('could not construct tracker for lesson');
    }
  }
  loadCourse();

  let hitRate = $state(0.0);
  let passed = $state(false);
  async function trackingDone() {
    await stop();
    // TODO: use bpm and accuracy stats to give star rating
    const detected = tracker.lastDetection;
    hitRate =
      detected.poseMatches / (detected.poseMisses + detected.poseMatches);
    passed = hitRate >= 0.6;
    if (passed) {
      recordFinishedLesson(id, lessonIndex, 1);
    }
    const sessionResult = computeDanceStats(detected.steps());
    if (sessionResult) {
      addDanceToStats(sessionResult);
    }
  }
  run(() => {
    if ($trackingState === DetectionState.TrackingDone) trackingDone();
  });

  onMount(start);

  onDestroy(() => {
    if (videoUrl) {
      URL.revokeObjectURL(videoUrl);
    }
  });
</script>

<LightBackground />

<div class="outer">
  {#if $trackingState === DetectionState.TrackingDone}
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
          on:click={restart}
          symbol=""
          text="courses.end.again-button"
        ></Button>
        <Button
          class="wide"
          on:click={goBack}
          symbol=""
          text="courses.end.back-button"
        ></Button>
      </div>
    {/if}
  {:else}
    <LiveRecording
      bind:startCamera
      bind:startRecording
      bind:stop={stopLiveRecording}
      bind:recordingStart
      bind:recordingEnd
      onStop={onRecordingStopped}
      videoOpacity={0.5}
      enableLiveAvatar={true}
      enableInstructorAvatar={true}
      forceBeat
    ></LiveRecording>
  {/if}
</div>

<Audio isOn={useFixedBpm && $trackingState !== DetectionState.TrackingDone}
></Audio>

<Popup bind:isOpen={showHint} showOkButton title={'common.hint-popup-title'}>
  {$t('record.estimate-bpm-hint')}
</Popup>

<Popup
  isOpen={startExercisePopUpIsOpen}
  title={'courses.lesson.exercise-popup-title'}
>
  <div>
    {$t('courses.lesson.exercise-start-description')}
  </div>
  <button class="wide" onclick={closeStartExercisePopUp}>OK</button>
  {@render children?.()}
</Popup>

{#if $dev}
  <DevUtility />
{/if}

<style>
  .outer {
    text-align: center;
    min-height: 100dvh;
  }
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
