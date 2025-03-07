<script>
  import { getContext } from 'svelte';
  import CourseLesson from './CourseLesson.svelte';

  /**
   * @typedef {Object} Props
   * @property {string} courseId
   * @property {function} onDone
   */

  /** @type {Props}*/
  let { courseId, onDone } = $props();

  const { getCourse } = getContext('courses');

  let course = $derived(getCourse(courseId));
  let lessonIndex = $state(0);
</script>

{#if course}
  <CourseLesson {course} {onDone} {lessonIndex}></CourseLesson>
{:else}
  course missing
{/if}
