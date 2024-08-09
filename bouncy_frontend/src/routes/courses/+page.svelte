<script>
  import { t } from '$lib/i18n';
  import { getContext } from 'svelte';
  import Header from '$lib/components/ui/Header.svelte';

  const { courses } = getContext('courses');
</script>

<Header title={$t('courses.title')} />

{#each $courses as course, i}
  <a class="course" href={course.id}>
    <div class="course-symbol">{i + 1}</div>
    <div class="course-name">{course.name}</div>
  </a>
{/each}

<div class="grayed-out">
  <div class="course-symbol">?</div>
  <div class="course-name">{$t('courses.more-coming')}</div>
</div>

<style>
  .course {
    text-decoration: none;
  }
  .course-symbol {
    text-align: center;
    border: solid 1px;
    width: 80%;
    margin: 25px auto 5px;
    font-size: 72px;
    height: 120px;
    line-height: 120px;
  }
  .course-name {
    text-align: center;
    font-weight: 800;
    font-size: 1.25em;
  }
  .grayed-out {
    color: var(--theme-neutral-gray);
  }
</style>
