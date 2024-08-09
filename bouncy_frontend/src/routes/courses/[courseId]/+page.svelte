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

<h2>{course.name}</h2>

<Step {step} poseIndex={$i} {animationTime} size={150}></Step>

<ol>
  {#each course.lessons as lesson, i}
    <li>
      {lesson.name}
    </li>
    <div class="note">
      {$t('courses.course-overview.lesson')}
      {lesson.parts.length}
      {#if lesson.parts.length === 1}
        {$t('courses.course-overview.part')}
      {:else}
        {$t('courses.course-overview.parts')}
      {/if}
    </div>
    <a href="./{i}">
      {$t('courses.course-overview.start-lesson')}
    </a>
  {/each}
</ol>

<style>
  .note {
    font-style: italic;
  }
  li {
    margin-top: 20px;
  }
</style>
