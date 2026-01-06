<script>
  import { run } from 'svelte/legacy';

  import { page } from '$app/state';
  import { t } from '$lib/i18n.js';
  import { getContext } from 'svelte';
  import Exercise from './Exercise.svelte';
  import Explanation from '$lib/components/ui/Explanation.svelte';
  import { goto } from '$app/navigation';
  import LogoHeader from '$lib/components/ui/header/LogoHeader.svelte';

  const { getCourse } = getContext('courses');

  let id;
  /** @type {import('bouncy_instructor').Course } */
  let course = $state();
  /** @type {number} */
  let lessonIndex;
  /** @type { import('bouncy_instructor').Lesson } */
  let lesson = $state();
  /** @type {number | undefined} */
  let partIndex = $state();
  /** @type {import("bouncy_instructor").LessonPart | undefined} */
  let exercise = $state();
  /** @type {boolean} */
  let done = $state();
  /** @type {string} */
  let title = $state();

  function loadCourse() {
    id = page.params.courseId;
    course = getCourse(id);
    lessonIndex = Number.parseInt(page.params.lesson);
    lesson = course.lessons[lessonIndex];
    title = $t('courses.lesson.title') + ' ' + (lessonIndex + 1);
    partIndex = undefined;
    exercise = undefined;
    done = false;
  }
  loadCourse();

  let explanationText;
  run(() => {
    explanationText = lesson.explanation || '';
  });

  /** @type {number} */
  let outerWidth = $state(320);
  let explanationWidth = $derived(
    outerWidth >= 320 ? 300 : Math.max(outerWidth - 20, 100)
  );

  function next() {
    if (done) {
      goto(`../${lessonIndex + 1}`).then(loadCourse);
    } else if (partIndex === undefined) {
      partIndex = 0;
      explanationText = lesson.parts[partIndex]?.explanation || '';
    } else if (partIndex < lesson.parts.length - 1) {
      partIndex += 1;
      explanationText = lesson.parts[partIndex]?.explanation || '';
    } else {
      partIndex = undefined;
      explanationText = $t('courses.lesson.done-text');
      done = true;
    }
  }
  function startExercise() {
    exercise = lesson.parts[partIndex];
  }
  function stopExercise() {
    exercise = undefined;
    next();
  }
  function goBack() {
    window.history.back();
  }
</script>

<LogoHeader {title} backButton />

<div bind:clientWidth={outerWidth}>
  <div class="subtitle">
    {course.name}
  </div>

  {#if exercise === undefined}
    <Explanation text={explanationText} width={explanationWidth}></Explanation>
  {:else}
    <Exercise lessonPart={exercise}></Exercise>
  {/if}

  <div class="controls">
    {#if done}
      <button onclick={goBack}>{$t('courses.lesson.back-button')}</button>
    {/if}
    {#if exercise === undefined && partIndex !== undefined && !done}
      <button onclick={startExercise}
        >{$t('courses.lesson.start-button')}</button
      >
    {/if}
    {#if exercise}
      <button onclick={stopExercise}>{$t('courses.lesson.stop-button')}</button>
    {/if}
    {#if partIndex === undefined}
      <button onclick={next}>{$t('courses.lesson.next-button')}</button>
    {/if}
  </div>
</div>

<style>
  .subtitle {
    font-style: italic;
    text-align: right;
    margin: 2px 5px 25px;
  }
  .controls {
    display: flex;
    justify-content: center;
    padding: 10px;
  }
  button {
    width: 100px;
    height: 61.8px;
    margin: 10px;
  }
</style>
