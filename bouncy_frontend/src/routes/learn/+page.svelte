<script>
  import { ONBOARDING_STATE } from '$lib/onboarding';
  import LoginRequiredContent from '$lib/components/profile/LoginRequiredContent.svelte';
  import LearnIntro from './LearnIntro.svelte';
  import ContinueFirstCourse from './ContinueFirstCourse.svelte';
</script>

<LoginRequiredContent>
  {#snippet guest({ apiUser })}
    <!-- stop formatting for the array in one of the conditions -->
    <!-- prettier-ignore -->
    {#if apiUser.meta.onboarding === ONBOARDING_STATE.FIRST_VISIT}
        <LearnIntro {apiUser} />
      {:else if apiUser.meta.onboarding === ONBOARDING_STATE.STARTED_FIRST_WARMUP}
        <ContinueFirstCourse {apiUser}/>
      {:else if [
        ONBOARDING_STATE.FINISHED_FIRST_WARMUP,
        ONBOARDING_STATE.STARTED_FIRST_LESSON,
        ONBOARDING_STATE.FINISHED_FIRST_LESSON,
        ONBOARDING_STATE.STARTED_SECOND_LESSON,
        ONBOARDING_STATE.FINISHED_SECOND_LESSON,
        ONBOARDING_STATE.STARTED_THIRD_LESSON
      ].includes(apiUser.meta.onboarding) 
      }
        <!-- Maybe show a different continuation screen? -->
        <ContinueFirstCourse {apiUser}/>
      {:else}
        TODO: all document, go back to home feed or something
      {/if}
  {/snippet}
</LoginRequiredContent>
