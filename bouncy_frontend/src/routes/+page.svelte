<script>
  import { ONBOARDING_STATE } from '$lib/onboarding';
  import FirstVisit from './FirstVisit.svelte';
  import HomeFeed from './HomeFeed.svelte';
  import ContinueFirstCourse from './ContinueFirstCourse.svelte';
  import LoginRequiredContent from '$lib/components/profile/LoginRequiredContent.svelte';

  /**
   * @typedef {Object} Props
   * @property {import('./$types').PageData} data
   */

  /** @type {Props} */
  let { data } = $props();

  // (mockup) learn today step
  const featuredSteps = data
    .lookupSteps({
      uniqueNames: true,
      sources: ['basic', 'footwork'],
    })
    .filter((_, i) => (i & 1) == 0)
    .slice(0, 3);
</script>

<LoginRequiredContent>
  {#snippet guest({ apiUser })}
    <!-- stop formatting for the array in one of the conditions -->
    <!-- prettier-ignore -->
    {#if apiUser.meta.onboarding === ONBOARDING_STATE.FIRST_VISIT && !apiUser.skippedIntro()}
        <FirstVisit />
      {:else if apiUser.meta.onboarding === ONBOARDING_STATE.STARTED_FIRST_WARMUP && !apiUser.skippedIntro()}
        <ContinueFirstCourse />
      {:else if [
        ONBOARDING_STATE.FINISHED_FIRST_WARMUP,
        ONBOARDING_STATE.STARTED_FIRST_LESSON,
        ONBOARDING_STATE.FINISHED_FIRST_LESSON,
        ONBOARDING_STATE.STARTED_SECOND_LESSON,
        ONBOARDING_STATE.FINISHED_SECOND_LESSON,
        ONBOARDING_STATE.STARTED_THIRD_LESSON
      ].includes(apiUser.meta.onboarding) 
        && !apiUser.skippedIntro()
      }
        <!-- Maybe show a different continuation screen? -->
        <ContinueFirstCourse />
      {:else}
        <HomeFeed featuredDances={data.officialDances} {featuredSteps} />
      {/if}
  {/snippet}
</LoginRequiredContent>
