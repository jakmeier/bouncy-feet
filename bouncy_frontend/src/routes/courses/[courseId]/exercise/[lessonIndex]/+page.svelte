<script>
  import { page } from '$app/stores';
  import { t } from '$lib/i18n.js';
  import Header from '$lib/components/ui/Header.svelte';
  import { getContext, onMount } from 'svelte';
  import { counter } from '$lib/timer';
  import Step from '../../../../collection/Step.svelte';
  import Video from '$lib/components/ui/Video.svelte';
  import { base } from '$app/paths';
  import Button from '$lib/components/ui/Button.svelte';
  import LightBackground from '$lib/components/ui/sections/LightBackground.svelte';

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

  let size = 100;
  // TODO: set from course lesson
  let bpm = 132;
  // use half speed
  $: stepTime = 60_000 / bpm;
  $: animationTime = Math.min(stepTime * 0.7, 300);
  $: i = counter(-1, 1, stepTime);

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

<div class="background-strip">
  {#if lesson.video && lesson.video.length > 0}
    <Video path={`${base}${lesson.video}`}></Video>
  {/if}
</div>

<!-- <h3>{$t('courses.lesson.steps-subtitle')}</h3> -->
<div class="overview">
  {#each lesson.parts as part, index}
    <div class="exercise-part">
      <Step step={part.step} poseIndex={$i} {animationTime} {size}></Step>
      <b>4x</b>
    </div>
    {#if index !== lesson.parts.length - 1}
      <div class="arrow">→</div>
    {/if}
  {/each}
</div>

<div class="controls">
  <a href="./record">
    <Button class="big" symbol="start" text="courses.lesson.start-button" />
  </a>
</div>

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

  .background-strip {
    margin: 10px -100%;
    padding: 5px;
    background-color: var(--theme-neutral-light);
    display: grid;
  }
</style>
