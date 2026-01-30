<script>
  import { page } from '$app/state';
  import { dev } from '$lib/stores/FeatureSelection';
  import DevUtility from '$lib/components/dev/DevUtility.svelte';
  import CourseLesson from '$lib/components/activity/CourseLesson.svelte';
  import { browser } from '$app/environment';
  import LoginRequiredContent from '$lib/components/profile/LoginRequiredContent.svelte';

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

<LoginRequiredContent>
  {#snippet guest({ apiUser })}
    <CourseLesson
      courseId={id}
      {lessonIndex}
      onDone={onLessonDone}
      onBack={backFromLesson}
      {apiUser}
    ></CourseLesson>

    {#if $dev}
      <DevUtility />
    {/if}
  {/snippet}
</LoginRequiredContent>
