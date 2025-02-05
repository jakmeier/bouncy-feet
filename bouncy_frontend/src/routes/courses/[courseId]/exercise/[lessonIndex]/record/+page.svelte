<script>
  import { page } from '$app/stores';
  import { t } from '$lib/i18n.js';
  import { getContext, onDestroy, tick } from 'svelte';
  import LiveRecording from '$lib/components/record/LiveRecording.svelte';
  import { DetectionState, Tracker } from '$lib/instructor/bouncy_instructor';
  import BeatSelector from '$lib/components/record/BeatSelector.svelte';
  import Popup from '$lib/components/ui/Popup.svelte';
  import LessonEndResults from './LessonEndResults.svelte';
  import VideoReview from '$lib/components/review/VideoReview.svelte';
  import Audio from '$lib/components/BeatAudio.svelte';
  import { registerTracker, setHalfSpeed } from '$lib/stores/Beat';
  import Button from '$lib/components/ui/Button.svelte';
  import { writable } from 'svelte/store';
  import LessonEnd from './LessonEnd.svelte';

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
  let recordingStart = undefined;
  /** @type {number | undefined} */
  let recordingEnd = undefined;

  /** @type {number} */
  let bpmDetectionCounter;
  /** @type {number} */
  let beatStart;
  let beatDetected = false;
  /** @type {import('svelte/store').Writable<boolean>} */
  let showHint;

  let detectionResult;
  /** @type {string} */
  let videoUrl;
  /** @type {import("$lib/instructor/bouncy_instructor").DetectedStep[] | undefined} */
  let detectedSteps;

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
  let useFixedBpm = false;
  /** @type {import('svelte/store').Writable<boolean>} */
  let beatHintPopUpIsOpen = writable(true);
  /** @type {import('svelte/store').Writable<boolean>} */
  let startExercisePopUpIsOpen = writable(false);

  let showHintBeforeStart = true;
  // wait for a user input before showing stats
  let showResults = false;

  async function start() {
    if (showHintBeforeStart) {
      // only show the hint once per sessions
      showHintBeforeStart = false;
      $startExercisePopUpIsOpen = true;
      return;
    }
    live = true;
    await tick();
    await startCamera();
    await startRecording();
  }

  async function stop() {
    stopCamera();
    tracker?.finishTracking();

    detectionResult = tracker?.lastDetection;
    detectedSteps = detectionResult?.steps();
    const videoBlob = await endRecording();

    if (videoBlob) {
      if (videoUrl) {
        URL.revokeObjectURL(videoUrl);
      }
      videoUrl = URL.createObjectURL(videoBlob);
    } else {
      console.warn('ended recording and did not get video blob', videoBlob);
    }
  }

  function restart() {
    tracker?.clear();
    bpmDetectionCounter = -1;
    beatStart = 0;
    live = false;
    recordingStart = undefined;
    recordingEnd = undefined;
    showResults = false;
  }

  function goBack() {
    window.history.back();
    window.history.back();
  }

  function closeBeatPopUp() {
    $beatHintPopUpIsOpen = false;
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
      setHalfSpeed(tracker.halfSpeed);
      registerTracker(tracker);
    } else {
      console.error('could not construct tracker for lesson');
    }
  }
  loadCourse();

  let hitRate = 0.0;
  let passed = false;
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
  $: if ($trackingState === DetectionState.TrackingDone) trackingDone();

  onDestroy(() => {
    if (videoUrl) {
      URL.revokeObjectURL(videoUrl);
    }
  });
</script>

<div class="outer">
  {#if !live}
    <!-- TODO: beat selector should be a layer above, optionally, by default it should be music from the app -->
    <BeatSelector
      bind:counter={bpmDetectionCounter}
      bind:bpmSelected={beatDetected}
      bind:useFixedBpm
    ></BeatSelector>
    <Button
      class={beatDetected ? 'light wide' : 'locked wide'}
      on:click={beatDetected ? start : () => showHint.set(true)}
      symbol="play_arrow"
      text="courses.lesson.start-button"
    ></Button>
  {:else if $trackingState === DetectionState.TrackingDone}
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
        {$t('record.no-video-for-review')}
      {/if}

      <div class="buttons">
        <Button
          class="light wide"
          on:click={restart}
          symbol=""
          text="courses.end.again-button"
        ></Button>
        <Button
          class="light wide"
          on:click={goBack}
          symbol=""
          text="courses.end.back-button"
        ></Button>
      </div>
    {/if}
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
      enableInstructorAvatar={true}
      forceBeat
    ></LiveRecording>

    <Button
      class="light wide"
      on:click={stop}
      symbol="camera"
      text="courses.lesson.stop-button"
    ></Button>
  {/if}
</div>

<Audio isOn={useFixedBpm && $trackingState !== DetectionState.TrackingDone}
></Audio>

<Popup bind:isOpen={showHint} showOkButton title={'common.hint-popup-title'}>
  {$t('record.estimate-bpm-hint')}
</Popup>

<Popup
  isOpen={beatHintPopUpIsOpen}
  title={'courses.lesson.exercise-popup-title'}
>
  <div>
    {$t('courses.lesson.exercise-beat-description')}
  </div>
  <button class="light wide" on:click={closeBeatPopUp}
    >{$t('courses.lesson.own-music-button')}</button
  >
  <button
    class="light wide"
    on:click={() => {
      useFixedBpm = true;
      closeBeatPopUp();
    }}>{$t('courses.lesson.play-beat-button')}</button
  >
  <slot />
</Popup>

<Popup
  isOpen={startExercisePopUpIsOpen}
  title={'courses.lesson.exercise-popup-title'}
>
  <div>
    {$t('courses.lesson.exercise-start-description')}
  </div>
  <button class="light wide" on:click={closeStartExercisePopUp}>OK</button>
  <slot />
</Popup>

<style>
  .outer {
    text-align: center;
  }
  .buttons {
    display: flex;
    flex-direction: column;
    margin-top: 10px;
  }
</style>
