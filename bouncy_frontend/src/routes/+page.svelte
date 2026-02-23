<script>
  import UserLoader from '$lib/components/profile/UserLoader.svelte';
  import { USER_AUH_STATE } from '$lib/enum_types';
  import { getUserContext } from '$lib/stores/context';
  import FirstVisit from './FirstVisit.svelte';
  import HomeFeed from './HomeFeed.svelte';

  /**
   * @typedef {Object} Props
   * @property {import('./$types').PageData} data
   */

  /** @type {Props} */
  let { data } = $props();

  /** @type {UserContextData} */
  const userCtx = getUserContext();

  let loading = $state(true);

  // (mockup) learn today step
  const featuredSteps = data
    .lookupSteps({
      uniqueNames: true,
      sources: ['basic', 'footwork'],
    })
    .filter((_, i) => (i & 1) == 0)
    .slice(0, 3);

  /** @param {BfError} err */
  function setError(err) {
    // TODO: show to user?
    console.log('error when loading user id', err);
  }
</script>

<UserLoader bind:loading {setError} />

{#if !loading && userCtx.authState === USER_AUH_STATE.Anonymous}
  <FirstVisit />
{:else}
  <HomeFeed featuredDances={data.officialDances} {featuredSteps} />
{/if}
