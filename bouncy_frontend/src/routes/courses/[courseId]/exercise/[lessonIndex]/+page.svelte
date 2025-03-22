<script>
  import { browser } from '$app/environment';
  import { page } from '$app/state';
  import CourseLesson from '$lib/components/activity/CourseLesson.svelte';

  /** @type {string} */
  let id = page.params.courseId;
  /** @type {number} */
  let lessonIndex = Number.parseInt(page.params.lessonIndex);

  async function resetScroll() {
    if (!browser) return;
    document.querySelector('.background')?.scrollTo(0, 0);
  }

  function backFromLesson() {
    window.history.back();
  }

  function onLessonDone() {
    resetScroll();
    window.history.back();
  }
</script>

<CourseLesson
  courseId={id}
  {lessonIndex}
  onDone={onLessonDone}
  onBack={backFromLesson}
></CourseLesson>
