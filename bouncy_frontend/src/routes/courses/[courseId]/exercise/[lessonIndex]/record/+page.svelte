<script>
  import { page } from '$app/state';
  import { dev } from '$lib/stores/FeatureSelection';
  import DevUtility from '$lib/components/dev/DevUtility.svelte';
  import CourseLesson from '$lib/components/activity/CourseLesson.svelte';
  import { browser } from '$app/environment';

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

{#if $dev}
  <DevUtility />
{/if}
