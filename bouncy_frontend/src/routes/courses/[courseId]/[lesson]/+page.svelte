<script>
  import { page } from '$app/stores';
  import { t } from '$lib/i18n.js';
  import Header from '$lib/components/ui/Header.svelte';
  import { getContext } from 'svelte';
  import LessonPart from './LessonPart.svelte';

  const { getCourse } = getContext('courses');

  const id = $page.params.courseId;
  const course = getCourse(id);
  const lessonIndex = Number.parseInt($page.params.lesson);
  const lesson = course.lessons[lessonIndex];

  let partIndex = 0;
  $: lessonPart = lesson.parts[partIndex];
</script>

<Header title={$t('courses.course-overview.lesson')} />

<h2>{course.name} {lessonIndex + 1}</h2>

<p>{lesson.explanation}</p>

<LessonPart {lessonPart}></LessonPart>
