<script>
  import { goto } from '$app/navigation';
  import { base } from '$app/paths';
  import Course from '$lib/components/activity/Course.svelte';
  import WarmUp from '$lib/components/activity/WarmUp.svelte';
  import AvatarStyleContext from '$lib/components/avatar/AvatarStyleContext.svelte';
  import ClassProgress from '$lib/components/class/ClassProgress.svelte';
  import StandardPage from '$lib/components/ui/StandardPage.svelte';
  import { t } from '$lib/i18n';
  import { ONBOARDING_STATE } from '$lib/onboarding';
  import { getContext, onMount } from 'svelte';

  /**
   * @typedef {Object} Props
   * @property {import('./$types').PageData} data
   */

  /** @type {Props} */
  let { data } = $props();

  /** @type {UserContextData}*/
  const { setUserMeta, clientSession } = getContext('user');

  const step = data.lookupSteps({ stepName: 'Knee Up', uniqueNames: true })[0];

  // TODO: This should be defined in the course / lesson / warmup
  const description =
    'I will make some moves, you try to copy me. Can you keep up?';

  let progress = $state(initProgress());
  // svelte-ignore state_referenced_locally
  const initialProgress = progress > 0;
  let showProgressScreen = $state(initialProgress);

  function onWarmupDone() {
    progress = 1;
    setUserMeta('onboarding', ONBOARDING_STATE.FINISHED_FIRST_WARMUP);
    showProgressScreen = true;
  }

  function onLessonDone() {
    showProgressScreen = true;
    if (progress === 1) {
      setUserMeta('onboarding', ONBOARDING_STATE.FINISHED_FIRST_LESSON);
      progress = 2;
    } else {
      // TODO: second lesson + wrapup lesson
      // WIP: directly go to end
      setUserMeta('onboarding', ONBOARDING_STATE.FINISHED_INTRO_PART1);
      progress = 4;
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
      case ONBOARDING_STATE.FINISHED_SECOND_LESSON: {
        return 3;
      }
      case ONBOARDING_STATE.FINISHED_INTRO_PART1:
      default:
        return 4;
    }
  }

  function onContinue() {
    showProgressScreen = false;
    if (progress === 1) {
      setUserMeta('onboarding', ONBOARDING_STATE.STARTED_FIRST_LESSON);
    }
    if (progress === 2) {
      setUserMeta('onboarding', ONBOARDING_STATE.STARTED_SECOND_LESSON);
    }
    // TODO: handle further progress
  }

  function onClassDone() {
    showProgressScreen = false;
  }

  function onLeave() {
    goto('/coach/select');
  }

  onMount(() => {
    if (progress > 0) {
      showProgressScreen = true;
    }
  });
</script>

{#if showProgressScreen}
  <ClassProgress {progress} {onContinue} onDone={onClassDone}></ClassProgress>
{:else}
  <!-- TODO: real video, real step -->
  <AvatarStyleContext>
    {#if progress === 0}
      <WarmUp
        {step}
        videoUrl={`${base}`}
        {description}
        audioControl={false}
        onDone={onWarmupDone}
      ></WarmUp>
    {/if}

    {#if progress === 1 || progress === 2}
      <!-- TODO: Should go to a different lesson on progress = 2-->
      <Course courseId="intro-lessons" onDone={onLessonDone}></Course>
    {/if}
    {#if progress === 4}
      <!-- TODO: translate text -->
      <StandardPage mainColor title={'Done'}>
        <h1>Well Done!</h1>
        <h3>Congrats on finishing your first class with Bouncy Feet!</h3>
        <h3>
          You can now select a coach to develop your skill in the direction of
          your choice.
        </h3>
        <h3>And don't forget to come back for your daily vibe!</h3>
        <button onclick={onLeave}>
          {$t('courses.lesson.show-teachers-button')}
        </button>
      </StandardPage>
    {/if}
  </AvatarStyleContext>
{/if}

<!-- Then: Show list of active moves for repetition lesson -->
<!-- Then: First repetition lesson -->
<!-- Then: Show full review -->

<!-- Then: Show teacher selection -->
