<script>
  import { page } from '$app/stores';
  import { t } from '$lib/i18n.js';
  import Header from '$lib/components/ui/Header.svelte';
  import { getContext, onMount } from 'svelte';
  import Step from '../../../../collection/Step.svelte';
  import Video from '$lib/components/ui/Video.svelte';
  import { base } from '$app/paths';
  import LightBackground from '$lib/components/ui/sections/LightBackground.svelte';
  import Popup from '$lib/components/ui/Popup.svelte';
  import { writable } from 'svelte/store';
  import { beatCounter, timeBetweenMoves } from '$lib/stores/Beat';

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
  /** @type {import("$lib/instructor/bouncy_instructor").LessonPart | undefined} */
  let exercise;
  /** @type {boolean} */
  let done;
  /** @type {string} */
  let title;

  let isVideoOpen = writable(false);
  let size = 100;

  function loadCourse() {
    id = $page.params.courseId;
    course = getCourse(id);
    lessonIndex = Number.parseInt($page.params.lessonIndex);
    lesson = course.lessons[lessonIndex];
    title = $t('courses.lesson.title') + ' ' + (lessonIndex + 1);
    partIndex = undefined;
    exercise = undefined;
    done = false;
  }
  loadCourse();

  const { setTrack } = getContext('music');
  onMount(() => {
    setTrack('120bpm_tech_house');
  });
</script>

<LightBackground />

<Header {title} />

<!-- TODO: translated texts -->
<div class="description">
  Kick, switch, kick, switch. Stop stalling, let's get moving!
</div>

<div class="background-strip">
  <div class="overview">
    <!-- <h3>{$t('courses.lesson.steps-subtitle')}</h3> -->
    {#each lesson.parts as part, index}
      <div class="exercise-part">
        <Step
          step={part.step}
          poseIndex={$beatCounter}
          animationTime={$timeBetweenMoves * 0.7}
          {size}
        ></Step>
        <b>4x</b>
      </div>
      {#if index !== lesson.parts.length - 1}
        <div class="arrow">→</div>
      {/if}
    {/each}
  </div>
</div>

<!-- TODO: translated texts -->
<div class="details">
  <div>Foot steps</div>
  <div>TODO</div>
  <div>Duration</div>
  <div>TODO</div>
  <div>Explainer video</div>
  <button on:click={() => ($isVideoOpen = true)}>-></button>
  <div>Song</div>
  <div>TODO</div>
</div>

<div class="controls">
  <a href="./record">
    <button>{$t('courses.lesson.start-button')}</button>
  </a>
</div>

<Popup bind:isOpen={isVideoOpen} showOkButton>
  <div class="video-wrapper">
    {#if lesson.video && lesson.video.length > 0}
      <Video path={`${base}${lesson.video}`}></Video>
    {/if}
  </div>
</Popup>

<style>
  .overview {
    display: flex;
    justify-content: space-around;
  }

  .exercise-part {
    text-align: center;
  }

  .arrow {
    line-height: 100px;
  }

  .controls {
    text-align: center;
  }

  .details,
  .description {
    margin: 2em 0em;
  }

  .background-strip {
    margin: 10px -100%;
    padding: 2rem;
    background-color: var(--theme-main);
    rotate: 8deg;
  }
  .background-strip .overview {
    rotate: -8deg;
  }

  .details {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
    align-items: center;
  }

  .video-wrapper {
    width: 100vw;
  }
</style>
