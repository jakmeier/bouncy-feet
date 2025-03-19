<script>
  import { browser } from '$app/environment';
  import { goto } from '$app/navigation';
  import CourseLesson from '$lib/components/activity/CourseLesson.svelte';
  import WarmUp from '$lib/components/activity/WarmUp.svelte';
  import AvatarStyleContext from '$lib/components/avatar/AvatarStyleContext.svelte';
  import ClassProgress from '$lib/components/class/ClassProgress.svelte';
  import StandardPage from '$lib/components/ui/StandardPage.svelte';
  import { t } from '$lib/i18n';
  import { ONBOARDING_STATE } from '$lib/onboarding';
  import { getContext, onMount } from 'svelte';

  /** @type {UserContextData}*/
  const { setUserMeta, clientSession } = getContext('user');

  const stepNames = ['Pendulum', 'Knee Up'];

  let progress = $state(initProgress());
  // svelte-ignore state_referenced_locally
  let selectedLesson = $state(Math.max(0, progress - 1));

  // svelte-ignore state_referenced_locally
  const initialProgress = progress > 0;
  let showProgressScreen = $state(initialProgress);
  let showEndScreen = $state(false);

  function onWarmupDone() {
    if (progress !== 0) return;
    progress = 1;
    setUserMeta('onboarding', ONBOARDING_STATE.FINISHED_FIRST_WARMUP);
    showProgressScreen = true;
  }

  function onLessonDone() {
    showProgressScreen = true;
    switch (progress) {
      case 1:
        setUserMeta('onboarding', ONBOARDING_STATE.FINISHED_FIRST_LESSON);
        progress = 2;
        break;
      case 2:
        setUserMeta('onboarding', ONBOARDING_STATE.FINISHED_SECOND_LESSON);
        progress = 3;
        break;
      case 3:
        setUserMeta('onboarding', ONBOARDING_STATE.FINISHED_INTRO_PART1);
        progress = 4;
        break;
    }
  }

  function initProgress() {
    switch (clientSession.meta.onboarding) {
      case ONBOARDING_STATE.FIRST_VISIT:
      case ONBOARDING_STATE.STARTED_FIRST_WARMUP: {
        return 0;
      }
      case ONBOARDING_STATE.FINISHED_FIRST_WARMUP:
      case ONBOARDING_STATE.STARTED_FIRST_LESSON: {
        return 1;
      }
      case ONBOARDING_STATE.FINISHED_FIRST_LESSON:
      case ONBOARDING_STATE.STARTED_SECOND_LESSON: {
        return 2;
      }
      case ONBOARDING_STATE.FINISHED_SECOND_LESSON:
      case ONBOARDING_STATE.STARTED_THIRD_LESSON: {
        return 3;
      }
      case ONBOARDING_STATE.FINISHED_INTRO_PART1:
      default:
        return 4;
    }
  }

  function onSelectLesson(lessonIndex) {
    selectedLesson = lessonIndex;
    showProgressScreen = false;
    resetScroll();
  }

  function onContinue() {
    showProgressScreen = false;
    resetScroll();

    if (progress === 1 && selectedLesson == 0) {
      setUserMeta('onboarding', ONBOARDING_STATE.STARTED_FIRST_LESSON);
    }
    if (progress === 2 && selectedLesson == 1) {
      setUserMeta('onboarding', ONBOARDING_STATE.STARTED_SECOND_LESSON);
    }
    if (progress === 3 && selectedLesson == 2) {
      setUserMeta('onboarding', ONBOARDING_STATE.STARTED_THIRD_LESSON);
    }
  }

  function onClassDone() {
    showProgressScreen = false;
    showEndScreen = true;
    resetScroll();
  }

  function onLeave() {
    goto('/coach/select');
  }

  async function resetScroll() {
    if (!browser) return;
    document.querySelector('.background')?.scrollTo(0, 0);
  }

  function backFromLesson() {
    showProgressScreen = true;
    resetScroll();
  }

  onMount(() => {
    if (progress > 0) {
      showProgressScreen = true;
      resetScroll();
    }
  });
</script>

{#if showProgressScreen}
  <ClassProgress {progress} {onContinue} onDone={onClassDone} {onSelectLesson}
  ></ClassProgress>
{:else if showEndScreen}
  <StandardPage mainColor title={$t('home.first-visit-done-title')}>
    <p>{$t('home.first-visit-done-0')}</p>
    <p>{$t('home.first-visit-done-1')}</p>
    <p>{$t('home.first-visit-done-2')}</p>
    <button onclick={onLeave}>
      {$t('courses.lesson.show-teachers-button')}
    </button>
  </StandardPage>
{:else}
  <!-- TODO: real video -->
  <AvatarStyleContext>
    {#if selectedLesson === 0}
      <WarmUp
        {stepNames}
        videoUrl={''}
        description={$t('record.warmup-preview-description')}
        audioControl={false}
        onDone={onWarmupDone}
        onBack={backFromLesson}
      ></WarmUp>
    {:else}
      <CourseLesson
        courseId="intro-lessons"
        lessonIndex={selectedLesson - 1}
        onDone={onLessonDone}
        onBack={backFromLesson}
      ></CourseLesson>
    {/if}
  </AvatarStyleContext>
{/if}
