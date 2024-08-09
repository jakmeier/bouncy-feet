<script>
  import { page } from '$app/stores';
  import { t } from '$lib/i18n.js';
  import Header from '$lib/components/ui/Header.svelte';
  import { getContext } from 'svelte';

  const { getCourse } = getContext('courses');

  const id = $page.params.courseId;
  const course = getCourse(id);
</script>

<Header title={$t('courses.course-overview.title')} />

<h2>{course.name}</h2>

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
