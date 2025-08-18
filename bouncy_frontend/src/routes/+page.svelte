<script>
  import { ONBOARDING_STATE } from '$lib/onboarding';
  import FirstVisit from './FirstVisit.svelte';
  import HomeFeedB from './HomeFeed_b.svelte';
  import ContinueFirstCourse from './ContinueFirstCourse.svelte';
  import { getUserContext } from '$lib/context';

  /** @type {UserContextData}*/
  const user = getUserContext();

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

{#await user.clientSession then clientSession}
  {#if clientSession.meta.onboarding === ONBOARDING_STATE.FIRST_VISIT}
    <FirstVisit />
  {:else if clientSession.meta.onboarding === ONBOARDING_STATE.STARTED_FIRST_WARMUP}
    <ContinueFirstCourse />
  {:else if [ONBOARDING_STATE.FINISHED_FIRST_WARMUP, ONBOARDING_STATE.STARTED_FIRST_LESSON, ONBOARDING_STATE.FINISHED_FIRST_LESSON, ONBOARDING_STATE.STARTED_SECOND_LESSON, ONBOARDING_STATE.FINISHED_SECOND_LESSON, ONBOARDING_STATE.STARTED_THIRD_LESSON].includes(clientSession.meta.onboarding)}
    <!-- Maybe show a different continuation screen? -->
    <ContinueFirstCourse />
  {:else}
    <HomeFeedB featuredDances={data.officialDances} {featuredSteps} />
  {/if}
{/await}
