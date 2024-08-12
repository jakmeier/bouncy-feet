<script>
  import { page } from '$app/stores';
  import { t } from '$lib/i18n.js';
  import Header from '$lib/components/ui/Header.svelte';
  import { getContext } from 'svelte';
  import Step from '../../collection/Step.svelte';
  import { counter } from '$lib/timer';

  const { getCourse } = getContext('courses');

  const id = $page.params.courseId;
  const course = getCourse(id);
  const step = course.featuredStep();

  const stepTime = 300;
  const animationTime = stepTime * 0.7;
  const i = counter(-1, 1, stepTime);
</script>

<Header title={$t('courses.course-overview.title')} />

<h3>{course.name}</h3>

<Step {step} poseIndex={$i} {animationTime} size={100} borderWidth={0}></Step>

<div class="ol">
  {#each course.lessons as lesson, index}
    <div class="index">{index + 1}</div>
    <div>
      {#if lesson.parts.length > 0}
        <Step
          step={lesson.parts[lesson.parts.length - 1].step}
          poseIndex={$i}
          {animationTime}
          size={75}
          borderWidth={1}
          lineWidth={2.5}
        ></Step>
      {/if}
    </div>
    <div class="li">
      {lesson.name}
      <div class="note">
        {$t('courses.course-overview.lesson')}
        {lesson.parts.length}
        {#if lesson.parts.length === 1}
          {$t('courses.course-overview.part')}
        {:else}
          {$t('courses.course-overview.parts')}
        {/if}
      </div>
      <a href="./{index}">
        {$t('courses.course-overview.start-lesson')}
      </a>
    </div>
  {/each}
</div>

<style>
  .note {
    font-style: italic;
  }
  .ol {
    display: grid;
    margin: 15px 10px 10px;
    grid-template-columns: min-content 75px auto;
    gap: 10px;
    align-items: center;
  }
  .li {
    padding: 10px;
  }
  .index {
    padding: 10px;
  }
</style>
