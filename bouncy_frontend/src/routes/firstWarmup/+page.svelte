<script>
  import { base } from '$app/paths';
  import WarmUp from '$lib/components/activity/WarmUp.svelte';
  import AvatarStyleContext from '$lib/components/avatar/AvatarStyleContext.svelte';
  import ClassProgress from '$lib/components/class/ClassProgress.svelte';
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
  let showProgressScreen = $state(false);

  function onWarmupDone() {
    progress = 1;
    setUserMeta('onboarding', ONBOARDING_STATE.FINISHED_FIRST_WARMUP);
    showProgressScreen = true;
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
      default:
        return 4;
    }
  }

  onMount(() => {
    console.log("progress", progress);
    console.log("clientSession.meta.onboarding", clientSession.meta.onboarding);
    if (progress > 0) {
      showProgressScreen = true;
    }
  });
</script>

{#if showProgressScreen}
  <ClassProgress
    {progress}
    onContinue={() => {
      showProgressScreen = false;
    }}
  ></ClassProgress>
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

    {#if progress === 1}
      TODO: show next lesson
    {/if}
  </AvatarStyleContext>
{/if}

<!-- After: Simplified review  -->
<!-- Then: First (real) lesson: step side-to-side -->
<!-- Then: Show full review -->
<!-- Then: Second lesson: heel forward left and right -->
<!-- Then: Show full review -->

<!-- Then: Show list of active moves for repetition lesson -->
<!-- Then: First repetition lesson -->
<!-- Then: Show full review -->

<!-- Then: Show teacher selection -->
